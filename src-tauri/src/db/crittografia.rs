use std::path::Path;

use rusqlite::functions::FunctionFlags;
use rusqlite::{Connection, ErrorCode};

use super::ErroreDatabase;
use crate::domain::testo::normalizza_per_ricerca;

pub const LUNGHEZZA_MINIMA_PASSWORD: usize = 12;

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
    // Flag predefiniti (con CREATE): servono all'ATTACH dei backup per creare il file di copia
    let connessione = Connection::open(percorso)?;
    imposta_chiave_e_configura(&connessione, password)?;
    Ok(connessione)
}

/// Cambia la password del database aperto, dopo aver verificato quella attuale sul file.
pub fn cambia_password(
    connessione: &Connection,
    percorso: &Path,
    password_attuale: &str,
    nuova_password: &str,
) -> Result<(), ErroreDatabase> {
    valida_nuova_password(nuova_password)?;
    drop(apri_database_criptato(percorso, password_attuale)?);
    connessione.pragma_update(None, "rekey", nuova_password)?;
    Ok(())
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
    registra_funzioni_sql(connessione)?;
    Ok(())
}

/// Funzioni Rust richiamabili da SQL. `normalizza(testo)` serve alla ricerca senza accenti.
fn registra_funzioni_sql(connessione: &Connection) -> rusqlite::Result<()> {
    connessione.create_scalar_function(
        "normalizza",
        1,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        |contesto| {
            let testo: Option<String> = contesto.get(0)?;
            Ok(testo.map(|testo| normalizza_per_ricerca(&testo)))
        },
    )
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
