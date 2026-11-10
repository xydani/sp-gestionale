//! Accesso ai dati: database SQLite criptato con SQLCipher.

pub mod backup;
pub mod crittografia;
pub mod impostazioni;
pub mod iscritti;
pub mod iscrizioni;
pub mod migrazioni;
pub mod pagamenti;
pub mod paginazione;
pub mod tipologie;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErroreDatabase {
    #[error("Password errata")]
    PasswordErrata,
    #[error("La password deve avere almeno {minimo} caratteri")]
    PasswordTroppoCorta { minimo: usize },
    #[error("Esiste già un database in questo percorso")]
    DatabaseGiaEsistente,
    #[error("Database non trovato")]
    DatabaseNonTrovato,
    #[error(
        "Il database è stato creato da una versione più recente del programma \
         (schema {versione_database}, supportato fino a {versione_supportata}). Aggiornare il programma."
    )]
    VersioneDatabaseSuccessiva {
        versione_database: u32,
        versione_supportata: u32,
    },
    #[error("{0}")]
    DatiNonValidi(String),
    #[error("Elemento non trovato")]
    NonTrovato,
    #[error("Errore sul file: {0}")]
    File(String),
    #[error("Errore del database: {0}")]
    Sqlite(#[from] rusqlite::Error),
}
