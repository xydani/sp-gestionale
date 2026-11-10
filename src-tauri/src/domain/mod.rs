//! Regole di dominio pure: nessun accesso a database o Tauri.

pub mod date;
pub mod rinnovo;
pub mod stato;
pub mod testo;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ErroreDominio {
    #[error("La durata dell'abbonamento deve essere maggiore di zero")]
    DurataNulla,
    #[error("La data di scadenza è fuori dall'intervallo supportato")]
    DataFuoriIntervallo,
}
