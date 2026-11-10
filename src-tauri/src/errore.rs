use serde::Serialize;

use crate::db::ErroreDatabase;

/// Errore inviato all'interfaccia: un codice stabile per la logica e un messaggio in italiano.
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct ErroreApp {
    pub codice: &'static str,
    pub messaggio: String,
}

impl ErroreApp {
    pub fn nuovo(codice: &'static str, messaggio: impl Into<String>) -> Self {
        Self {
            codice,
            messaggio: messaggio.into(),
        }
    }
}

impl From<ErroreDatabase> for ErroreApp {
    fn from(errore: ErroreDatabase) -> Self {
        let codice = match &errore {
            ErroreDatabase::PasswordErrata => "password_errata",
            ErroreDatabase::PasswordTroppoCorta { .. } => "password_troppo_corta",
            ErroreDatabase::DatabaseGiaEsistente => "database_gia_esistente",
            ErroreDatabase::DatabaseNonTrovato => "database_non_trovato",
            ErroreDatabase::VersioneDatabaseSuccessiva { .. } => "versione_database_successiva",
            ErroreDatabase::File(_) => "errore_file",
            ErroreDatabase::Sqlite(_) => "errore_database",
        };
        Self::nuovo(codice, errore.to_string())
    }
}
