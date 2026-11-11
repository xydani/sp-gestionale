//! Funzioni condivise dai test d'integrazione.

use std::path::PathBuf;

use rusqlite::Connection;
use sp_gestionale_lib::db::crittografia::crea_database_criptato;
use sp_gestionale_lib::db::migrazioni::applica_migrazioni;
use tempfile::TempDir;

pub const PASSWORD_TEST: &str = "password-di-test";

pub struct DatabaseTemporaneo {
    // Tenuta in vita: la cartella viene cancellata quando il valore esce di scope
    _cartella: TempDir,
    pub percorso: PathBuf,
}

impl DatabaseTemporaneo {
    pub fn nuovo() -> Self {
        let cartella = tempfile::tempdir().expect("cartella temporanea");
        let percorso = cartella.path().join("palestra.db");
        Self {
            _cartella: cartella,
            percorso,
        }
    }

    /// Crea il database criptato con lo schema aggiornato.
    // Ogni file di test compila il modulo per conto suo: chi parte da uno schema vecchio non lo usa
    #[allow(dead_code)]
    pub fn crea_con_schema(&self) -> Connection {
        let mut connessione = crea_database_criptato(&self.percorso, PASSWORD_TEST).unwrap();
        applica_migrazioni(&mut connessione).unwrap();
        connessione
    }
}
