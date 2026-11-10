//! Stato condiviso dell'app: la connessione al database esiste solo dopo lo sblocco.

use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;

use chrono::NaiveDateTime;
use rusqlite::Connection;
use serde::Serialize;
use zeroize::Zeroizing;

use crate::db::backup::{
    crea_cartella, esegui_backup, esegui_backup_giornaliero, nome_backup_manuale,
    nome_backup_pre_aggiornamento, nome_backup_pre_migrazione, nome_backup_pre_ripristino,
    sostituisci_database_con_backup, verifica_backup,
};
use crate::db::crittografia::{apri_database_criptato, cambia_password, crea_database_criptato};
use crate::db::impostazioni::{
    BACKUP_CONSERVATI_PREDEFINITI, CHIAVE_BACKUP_CONSERVATI, leggi_impostazione_numerica,
    richiedi_configurazione_iniziale,
};
use crate::db::migrazioni::{
    applica_migrazioni, serve_backup_pre_migrazione, versione_schema_corrente,
};
use crate::errore::ErroreApp;

/// Dopo questo numero di password errate consecutive ogni tentativo viene rallentato.
pub const TENTATIVI_PRIMA_DEL_RALLENTAMENTO: u32 = 5;
pub const ATTESA_DOPO_TROPPI_TENTATIVI: Duration = Duration::from_secs(3);
const NOME_CARTELLA_BACKUP: &str = "backup";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StatoDatabase {
    /// Primo avvio: il database non è ancora stato creato.
    DaCreare,
    Bloccato,
    Sbloccato,
}

/// Esito dello sblocco: il database è aperto, ma un backup automatico potrebbe essere fallito.
#[derive(Debug, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EsitoSblocco {
    pub avviso_backup: Option<String>,
}

pub struct StatoApp {
    percorso_database: PathBuf,
    cartella_backup: PathBuf,
    connessione: Mutex<Option<Connection>>,
    tentativi_falliti: Mutex<u32>,
    adesso: fn() -> NaiveDateTime,
}

impl StatoApp {
    pub fn nuovo(percorso_database: PathBuf) -> Self {
        Self::con_orologio(percorso_database, || chrono::Local::now().naive_local())
    }

    /// Come `nuovo`, con un orologio sostituibile nei test.
    pub fn con_orologio(percorso_database: PathBuf, adesso: fn() -> NaiveDateTime) -> Self {
        let cartella_backup = percorso_database.parent().map_or_else(
            || PathBuf::from(NOME_CARTELLA_BACKUP),
            |cartella| cartella.join(NOME_CARTELLA_BACKUP),
        );
        Self {
            percorso_database,
            cartella_backup,
            connessione: Mutex::new(None),
            tentativi_falliti: Mutex::new(0),
            adesso,
        }
    }

    pub fn oggi(&self) -> chrono::NaiveDate {
        (self.adesso)().date()
    }

    pub fn cartella_backup(&self) -> &Path {
        &self.cartella_backup
    }

    pub fn stato_database(&self) -> StatoDatabase {
        if self.connessione_bloccata().is_some() {
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
            return Err(errore_password_diverse());
        }
        if let Some(cartella) = self.percorso_database.parent() {
            crea_cartella(cartella)?;
        }
        let mut connessione = crea_database_criptato(&self.percorso_database, &password)?;
        applica_migrazioni(&mut connessione)?;
        richiedi_configurazione_iniziale(&connessione)?;
        *self.connessione_bloccata() = Some(connessione);
        Ok(())
    }

    /// Cambia la password del database sbloccato. I backup già fatti restano con la vecchia.
    pub fn cambia_password(
        &self,
        password_attuale: Zeroizing<String>,
        nuova_password: Zeroizing<String>,
        conferma_password: Zeroizing<String>,
    ) -> Result<(), ErroreApp> {
        if *nuova_password != *conferma_password {
            return Err(errore_password_diverse());
        }
        self.con_connessione(|connessione| {
            cambia_password(
                connessione,
                &self.percorso_database,
                &password_attuale,
                &nuova_password,
            )
        })
    }

    pub fn sblocca_database(&self, password: Zeroizing<String>) -> Result<EsitoSblocco, ErroreApp> {
        if *self.tentativi_bloccati() >= TENTATIVI_PRIMA_DEL_RALLENTAMENTO {
            std::thread::sleep(ATTESA_DOPO_TROPPI_TENTATIVI);
        }

        let risultato = self.apri_e_aggiorna(&password);

        let mut tentativi_falliti = self.tentativi_bloccati();
        match risultato {
            Ok(connessione) => {
                *tentativi_falliti = 0;
                let esito = EsitoSblocco {
                    avviso_backup: self.backup_giornaliero(&connessione),
                };
                *self.connessione_bloccata() = Some(connessione);
                Ok(esito)
            }
            Err(errore) => {
                *tentativi_falliti += 1;
                Err(errore)
            }
        }
    }

    /// Chiude la connessione: per tornare operativi serve di nuovo la password.
    pub fn blocca_database(&self) {
        *self.connessione_bloccata() = None;
    }

    pub fn tentativi_falliti(&self) -> u32 {
        *self.tentativi_bloccati()
    }

    /// Copia criptata nella cartella dei backup automatici, prima di installare un aggiornamento.
    pub fn esegui_backup_pre_aggiornamento(
        &self,
        versione_installata: &str,
    ) -> Result<PathBuf, ErroreApp> {
        let connessione = self.connessione_bloccata();
        let connessione = connessione.as_ref().ok_or_else(errore_database_bloccato)?;
        crea_cartella(&self.cartella_backup)?;
        let destinazione = self.cartella_backup.join(nome_backup_pre_aggiornamento(
            versione_installata,
            (self.adesso)(),
        ));
        esegui_backup(connessione, &destinazione)?;
        Ok(destinazione)
    }

    /// Salva una copia criptata del database nella cartella scelta dall'utente.
    pub fn esegui_backup_manuale(
        &self,
        cartella_destinazione: &Path,
    ) -> Result<PathBuf, ErroreApp> {
        let connessione = self.connessione_bloccata();
        let connessione = connessione.as_ref().ok_or_else(errore_database_bloccato)?;
        let destinazione = cartella_destinazione.join(nome_backup_manuale((self.adesso)()));
        esegui_backup(connessione, &destinazione)?;
        Ok(destinazione)
    }

    /// Sostituisce i dati attuali con quelli del backup, dopo averne salvato una copia.
    /// La password è quella valida al momento del backup e da qui in poi diventa quella attuale.
    pub fn ripristina_backup(
        &self,
        percorso_backup: &Path,
        password_backup: Zeroizing<String>,
    ) -> Result<(), ErroreApp> {
        verifica_backup(percorso_backup, &password_backup)?;

        let mut connessione = self.connessione_bloccata();
        {
            let connessione_attuale = connessione.as_ref().ok_or_else(errore_database_bloccato)?;
            crea_cartella(&self.cartella_backup)?;
            let copia_sicurezza = self
                .cartella_backup
                .join(nome_backup_pre_ripristino((self.adesso)()));
            esegui_backup(connessione_attuale, &copia_sicurezza)?;
        }
        // La connessione va chiusa prima di sostituire il file
        *connessione = None;
        sostituisci_database_con_backup(percorso_backup, &self.percorso_database)?;
        *connessione = Some(self.apri_e_aggiorna(&password_backup)?);
        Ok(())
    }

    /// Esegue un'operazione sul database; errore se l'archivio è bloccato.
    pub fn con_connessione<T>(
        &self,
        operazione: impl FnOnce(&Connection) -> Result<T, crate::db::ErroreDatabase>,
    ) -> Result<T, ErroreApp> {
        let connessione = self.connessione_bloccata();
        let connessione = connessione.as_ref().ok_or_else(errore_database_bloccato)?;
        Ok(operazione(connessione)?)
    }

    /// Come `con_connessione`, per operazioni che richiedono una transazione.
    pub fn con_connessione_modificabile<T>(
        &self,
        operazione: impl FnOnce(&mut Connection) -> Result<T, crate::db::ErroreDatabase>,
    ) -> Result<T, ErroreApp> {
        let mut connessione = self.connessione_bloccata();
        let connessione = connessione.as_mut().ok_or_else(errore_database_bloccato)?;
        Ok(operazione(connessione)?)
    }

    fn apri_e_aggiorna(&self, password: &str) -> Result<Connection, ErroreApp> {
        let mut connessione = apri_database_criptato(&self.percorso_database, password)?;
        let versione_corrente = versione_schema_corrente(&connessione)?;
        if serve_backup_pre_migrazione(versione_corrente) {
            crea_cartella(&self.cartella_backup)?;
            let destinazione = self.cartella_backup.join(nome_backup_pre_migrazione(
                versione_corrente,
                (self.adesso)(),
            ));
            esegui_backup(&connessione, &destinazione)?;
        }
        applica_migrazioni(&mut connessione)?;
        Ok(connessione)
    }

    /// Un backup giornaliero fallito non deve impedire di lavorare: diventa un avviso.
    fn backup_giornaliero(&self, connessione: &Connection) -> Option<String> {
        let esito = leggi_impostazione_numerica(
            connessione,
            CHIAVE_BACKUP_CONSERVATI,
            BACKUP_CONSERVATI_PREDEFINITI,
        )
        .and_then(|da_conservare| {
            esegui_backup_giornaliero(
                connessione,
                &self.cartella_backup,
                (self.adesso)(),
                da_conservare,
            )
        });
        esito
            .err()
            .map(|errore| format!("Backup automatico non riuscito: {errore}"))
    }

    fn connessione_bloccata(&self) -> MutexGuard<'_, Option<Connection>> {
        self.connessione.lock().expect("lock connessione")
    }

    fn tentativi_bloccati(&self) -> MutexGuard<'_, u32> {
        self.tentativi_falliti.lock().expect("lock tentativi")
    }
}

fn errore_password_diverse() -> ErroreApp {
    ErroreApp::nuovo("password_diverse", "Le due password non coincidono")
}

fn errore_database_bloccato() -> ErroreApp {
    ErroreApp::nuovo(
        "database_bloccato",
        "L'archivio è bloccato: inserire la password",
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const PASSWORD: &str = "password-di-test";

    fn segreto(testo: &str) -> Zeroizing<String> {
        Zeroizing::new(testo.to_string())
    }

    fn orologio_fisso() -> NaiveDateTime {
        NaiveDateTime::parse_from_str("2026-11-10 09:30:00", "%Y-%m-%d %H:%M:%S").unwrap()
    }

    fn stato_in_cartella_temporanea() -> (tempfile::TempDir, StatoApp) {
        let cartella = tempfile::tempdir().unwrap();
        let percorso = cartella.path().join("dati").join("palestra.db");
        (cartella, StatoApp::con_orologio(percorso, orologio_fisso))
    }

    fn stato_creato_e_bloccato() -> (tempfile::TempDir, StatoApp) {
        let (cartella, stato) = stato_in_cartella_temporanea();
        stato
            .crea_database(segreto(PASSWORD), segreto(PASSWORD))
            .unwrap();
        stato.blocca_database();
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
        let (_cartella, stato) = stato_creato_e_bloccato();
        assert_eq!(stato.stato_database(), StatoDatabase::Bloccato);
        stato.sblocca_database(segreto(PASSWORD)).unwrap();
        assert_eq!(stato.stato_database(), StatoDatabase::Sbloccato);
    }

    #[test]
    fn password_errata_resta_bloccato_e_conta_i_tentativi() {
        let (_cartella, stato) = stato_creato_e_bloccato();
        let errore = stato
            .sblocca_database(segreto("password-errata"))
            .unwrap_err();
        assert_eq!(errore.codice, "password_errata");
        assert_eq!(stato.stato_database(), StatoDatabase::Bloccato);
        assert_eq!(stato.tentativi_falliti(), 1);
    }

    #[test]
    fn lo_sblocco_riuscito_azzera_i_tentativi() {
        let (_cartella, stato) = stato_creato_e_bloccato();
        let _ = stato.sblocca_database(segreto("password-errata"));
        stato.sblocca_database(segreto(PASSWORD)).unwrap();
        assert_eq!(stato.tentativi_falliti(), 0);
    }

    #[test]
    fn lo_sblocco_crea_il_backup_giornaliero_una_sola_volta() {
        let (_cartella, stato) = stato_creato_e_bloccato();
        let esito = stato.sblocca_database(segreto(PASSWORD)).unwrap();
        stato.blocca_database();
        stato.sblocca_database(segreto(PASSWORD)).unwrap();

        assert_eq!(esito.avviso_backup, None);
        let backup: Vec<_> = std::fs::read_dir(stato.cartella_backup())
            .unwrap()
            .collect();
        assert_eq!(backup.len(), 1);
    }

    #[test]
    fn il_backup_pre_aggiornamento_finisce_tra_i_backup_automatici() {
        let (_cartella, stato) = stato_creato_e_bloccato();
        assert!(stato.esegui_backup_pre_aggiornamento("0.1.0").is_err());
        stato.sblocca_database(segreto(PASSWORD)).unwrap();
        let percorso = stato.esegui_backup_pre_aggiornamento("0.1.0").unwrap();
        assert!(percorso.starts_with(stato.cartella_backup()));
        assert!(percorso.exists());
        let nome = percorso.file_name().unwrap().to_string_lossy();
        assert!(nome.starts_with("pre-aggiornamento-v0.1.0-"), "{nome}");
        crate::db::backup::verifica_backup(&percorso, PASSWORD).unwrap();
    }

    #[test]
    fn il_backup_manuale_richiede_il_database_sbloccato() {
        let (cartella, stato) = stato_creato_e_bloccato();
        let errore = stato.esegui_backup_manuale(cartella.path()).unwrap_err();
        assert_eq!(errore.codice, "database_bloccato");
    }

    #[test]
    fn le_operazioni_sul_database_richiedono_lo_sblocco() {
        let (_cartella, stato) = stato_creato_e_bloccato();
        let errore = stato.con_connessione(|_| Ok(())).unwrap_err();
        assert_eq!(errore.codice, "database_bloccato");

        stato.sblocca_database(segreto(PASSWORD)).unwrap();
        assert!(stato.con_connessione(|_| Ok(())).is_ok());
    }

    #[test]
    fn un_nuovo_database_richiede_la_configurazione_iniziale() {
        let (_cartella, stato) = stato_in_cartella_temporanea();
        stato
            .crea_database(segreto(PASSWORD), segreto(PASSWORD))
            .unwrap();
        let completata = stato
            .con_connessione(crate::db::impostazioni::configurazione_iniziale_completata)
            .unwrap();
        assert!(!completata);
    }

    #[test]
    fn dopo_il_cambio_serve_la_nuova_password() {
        let (_cartella, stato) = stato_creato_e_bloccato();
        stato.sblocca_database(segreto(PASSWORD)).unwrap();
        stato
            .cambia_password(
                segreto(PASSWORD),
                segreto("nuova-password"),
                segreto("nuova-password"),
            )
            .unwrap();
        stato.blocca_database();
        assert!(stato.sblocca_database(segreto(PASSWORD)).is_err());
        stato.sblocca_database(segreto("nuova-password")).unwrap();
    }

    #[test]
    fn il_cambio_richiede_la_password_attuale_corretta() {
        let (_cartella, stato) = stato_creato_e_bloccato();
        stato.sblocca_database(segreto(PASSWORD)).unwrap();
        let errore = stato
            .cambia_password(
                segreto("password-sbagliata"),
                segreto("nuova-password"),
                segreto("nuova-password"),
            )
            .unwrap_err();
        assert_eq!(errore.codice, "password_errata");
        stato.blocca_database();
        stato.sblocca_database(segreto(PASSWORD)).unwrap();
    }

    #[test]
    fn il_cambio_rifiuta_conferma_diversa_e_password_corta() {
        let (_cartella, stato) = stato_creato_e_bloccato();
        stato.sblocca_database(segreto(PASSWORD)).unwrap();
        let cambia = |nuova: &str, conferma: &str| {
            stato.cambia_password(segreto(PASSWORD), segreto(nuova), segreto(conferma))
        };
        assert_eq!(
            cambia("nuova-password", "altra-password")
                .unwrap_err()
                .codice,
            "password_diverse"
        );
        assert!(cambia("corta", "corta").is_err());
    }

    #[test]
    fn lo_stato_si_serializza_in_snake_case() {
        assert_eq!(
            serde_json::to_string(&StatoDatabase::DaCreare).unwrap(),
            "\"da_creare\""
        );
    }
}
