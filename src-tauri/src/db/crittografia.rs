use std::path::Path;

use rusqlite::{Connection, ErrorCode, OpenFlags};

use super::ErroreDatabase;

pub const LUNGHEZZA_MINIMA_PASSWORD: usize = 8;

/// Crea un nuovo database criptato con la password indicata.
pub fn crea_database_criptato(
    percorso: &Path,
    password: &str,
) -> Result<Connection, ErroreDatabase> {
    valida_nuova_password(password)?;
    if percorso.exists() {
        return Err(ErroreDatabase::DatabaseGiaEsistente);
    }
    let connessione = Connection::open(percorso)?;
    imposta_chiave_e_configura(&connessione, password)?;
    Ok(connessione)
}

/// Apre un database criptato esistente; con la password sbagliata restituisce `PasswordErrata`.
pub fn apri_database_criptato(
    percorso: &Path,
    password: &str,
) -> Result<Connection, ErroreDatabase> {
    if !percorso.exists() {
        return Err(ErroreDatabase::DatabaseNonTrovato);
    }
    let connessione = Connection::open_with_flags(percorso, OpenFlags::SQLITE_OPEN_READ_WRITE)?;
    imposta_chiave_e_configura(&connessione, password)?;
    Ok(connessione)
}

pub fn valida_nuova_password(password: &str) -> Result<(), ErroreDatabase> {
    if password.chars().count() < LUNGHEZZA_MINIMA_PASSWORD {
        return Err(ErroreDatabase::PasswordTroppoCorta {
            minimo: LUNGHEZZA_MINIMA_PASSWORD,
        });
    }
    Ok(())
}

fn imposta_chiave_e_configura(
    connessione: &Connection,
    password: &str,
) -> Result<(), ErroreDatabase> {
    connessione.pragma_update(None, "key", password)?;
    verifica_password(connessione)?;
    connessione.pragma_update(None, "journal_mode", "WAL")?;
    connessione.pragma_update(None, "foreign_keys", "ON")?;
    Ok(())
}

/// SQLCipher non verifica la chiave finché non legge una pagina:
/// con la chiave sbagliata la prima lettura fallisce con "file is not a database".
fn verifica_password(connessione: &Connection) -> Result<(), ErroreDatabase> {
    match connessione.query_row("SELECT count(*) FROM sqlite_master", [], |_| Ok(())) {
        Ok(()) => Ok(()),
        Err(rusqlite::Error::SqliteFailure(errore, _))
            if errore.code == ErrorCode::NotADatabase =>
        {
            Err(ErroreDatabase::PasswordErrata)
        }
        Err(altro) => Err(altro.into()),
    }
}
