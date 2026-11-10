//! Stato condiviso dell'app: la connessione al database esiste solo dopo lo sblocco.

use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;

use rusqlite::Connection;
use serde::Serialize;
use zeroize::Zeroizing;

use crate::db::crittografia::{apri_database_criptato, crea_database_criptato};
use crate::db::migrazioni::applica_migrazioni;
use crate::errore::ErroreApp;

/// Dopo questo numero di password errate consecutive ogni tentativo viene rallentato.
pub const TENTATIVI_PRIMA_DEL_RALLENTAMENTO: u32 = 5;
pub const ATTESA_DOPO_TROPPI_TENTATIVI: Duration = Duration::from_secs(3);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StatoDatabase {
    /// Primo avvio: il database non è ancora stato creato.
    DaCreare,
    Bloccato,
    Sbloccato,
}

pub struct StatoApp {
    percorso_database: PathBuf,
    connessione: Mutex<Option<Connection>>,
    tentativi_falliti: Mutex<u32>,
}

impl StatoApp {
    pub fn nuovo(percorso_database: PathBuf) -> Self {
        Self {
            percorso_database,
            connessione: Mutex::new(None),
            tentativi_falliti: Mutex::new(0),
        }
    }

    pub fn stato_database(&self) -> StatoDatabase {
        if self.connessione.lock().expect("lock connessione").is_some() {
            StatoDatabase::Sbloccato
        } else if self.percorso_database.exists() {
            StatoDatabase::Bloccato
        } else {
            StatoDatabase::DaCreare
        }
    }

    pub fn crea_database(
        &self,
        password: Zeroizing<String>,
        conferma_password: Zeroizing<String>,
    ) -> Result<(), ErroreApp> {
        if *password != *conferma_password {
            return Err(ErroreApp::nuovo(
                "password_diverse",
                "Le due password non coincidono",
            ));
        }
        if let Some(cartella) = self.percorso_database.parent() {
            std::fs::create_dir_all(cartella).map_err(|errore| {
                ErroreApp::nuovo(
                    "cartella_dati",
                    format!("Impossibile creare la cartella dati: {errore}"),
                )
            })?;
        }
        let mut connessione = crea_database_criptato(&self.percorso_database, &password)?;
        applica_migrazioni(&mut connessione)?;
        *self.connessione.lock().expect("lock connessione") = Some(connessione);
        Ok(())
    }

    pub fn sblocca_database(&self, password: Zeroizing<String>) -> Result<(), ErroreApp> {
        if *self.tentativi_falliti.lock().expect("lock tentativi")
            >= TENTATIVI_PRIMA_DEL_RALLENTAMENTO
        {
            std::thread::sleep(ATTESA_DOPO_TROPPI_TENTATIVI);
        }

        let risultato = apri_database_criptato(&self.percorso_database, &password).and_then(
            |mut connessione| {
                applica_migrazioni(&mut connessione)?;
                Ok(connessione)
            },
        );

        let mut tentativi_falliti = self.tentativi_falliti.lock().expect("lock tentativi");
        match risultato {
            Ok(connessione) => {
                *tentativi_falliti = 0;
                *self.connessione.lock().expect("lock connessione") = Some(connessione);
                Ok(())
            }
            Err(errore) => {
                *tentativi_falliti += 1;
                Err(errore.into())
            }
        }
    }

    /// Chiude la connessione: per tornare operativi serve di nuovo la password.
    pub fn blocca_database(&self) {
        *self.connessione.lock().expect("lock connessione") = None;
    }

    pub fn tentativi_falliti(&self) -> u32 {
        *self.tentativi_falliti.lock().expect("lock tentativi")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const PASSWORD: &str = "password-di-test";

    fn segreto(testo: &str) -> Zeroizing<String> {
        Zeroizing::new(testo.to_string())
    }

    fn stato_in_cartella_temporanea() -> (tempfile::TempDir, StatoApp) {
        let cartella = tempfile::tempdir().unwrap();
        let stato = StatoApp::nuovo(cartella.path().join("dati").join("palestra.db"));
        (cartella, stato)
    }

    #[test]
    fn al_primo_avvio_il_database_e_da_creare() {
        let (_cartella, stato) = stato_in_cartella_temporanea();
        assert_eq!(stato.stato_database(), StatoDatabase::DaCreare);
    }

    #[test]
    fn dopo_la_creazione_il_database_e_sbloccato() {
        let (_cartella, stato) = stato_in_cartella_temporanea();
        stato
            .crea_database(segreto(PASSWORD), segreto(PASSWORD))
            .unwrap();
        assert_eq!(stato.stato_database(), StatoDatabase::Sbloccato);
    }

    #[test]
    fn password_e_conferma_devono_coincidere() {
        let (_cartella, stato) = stato_in_cartella_temporanea();
        let errore = stato
            .crea_database(segreto(PASSWORD), segreto("altra-password"))
            .unwrap_err();
        assert_eq!(errore.codice, "password_diverse");
        assert_eq!(stato.stato_database(), StatoDatabase::DaCreare);
    }

    #[test]
    fn password_troppo_corta_e_rifiutata() {
        let (_cartella, stato) = stato_in_cartella_temporanea();
        let errore = stato
            .crea_database(segreto("corta"), segreto("corta"))
            .unwrap_err();
        assert_eq!(errore.codice, "password_troppo_corta");
    }

    #[test]
    fn dopo_il_blocco_serve_la_password() {
        let (_cartella, stato) = stato_in_cartella_temporanea();
        stato
            .crea_database(segreto(PASSWORD), segreto(PASSWORD))
            .unwrap();
        stato.blocca_database();
        assert_eq!(stato.stato_database(), StatoDatabase::Bloccato);

        stato.sblocca_database(segreto(PASSWORD)).unwrap();
        assert_eq!(stato.stato_database(), StatoDatabase::Sbloccato);
    }

    #[test]
    fn password_errata_resta_bloccato_e_conta_i_tentativi() {
        let (_cartella, stato) = stato_in_cartella_temporanea();
        stato
            .crea_database(segreto(PASSWORD), segreto(PASSWORD))
            .unwrap();
        stato.blocca_database();

        let errore = stato
            .sblocca_database(segreto("password-errata"))
            .unwrap_err();

        assert_eq!(errore.codice, "password_errata");
        assert_eq!(stato.stato_database(), StatoDatabase::Bloccato);
        assert_eq!(stato.tentativi_falliti(), 1);
    }

    #[test]
    fn lo_sblocco_riuscito_azzera_i_tentativi() {
        let (_cartella, stato) = stato_in_cartella_temporanea();
        stato
            .crea_database(segreto(PASSWORD), segreto(PASSWORD))
            .unwrap();
        stato.blocca_database();
        let _ = stato.sblocca_database(segreto("password-errata"));

        stato.sblocca_database(segreto(PASSWORD)).unwrap();

        assert_eq!(stato.tentativi_falliti(), 0);
    }

    #[test]
    fn lo_stato_si_serializza_in_snake_case() {
        assert_eq!(
            serde_json::to_string(&StatoDatabase::DaCreare).unwrap(),
            "\"da_creare\""
        );
    }
}
