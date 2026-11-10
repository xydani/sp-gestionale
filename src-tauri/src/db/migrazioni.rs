use rusqlite::Connection;

use super::ErroreDatabase;

/// Migrazioni in ordine: la posizione + 1 è la versione dello schema.
/// Una migrazione rilasciata non va mai modificata: se ne aggiunge una nuova.
const MIGRAZIONI: &[&str] = &[include_str!("../../migrations/0001_init.sql")];

pub fn versione_schema_supportata() -> u32 {
    MIGRAZIONI.len() as u32
}

pub fn versione_schema_corrente(connessione: &Connection) -> Result<u32, ErroreDatabase> {
    Ok(connessione.pragma_query_value(None, "user_version", |riga| riga.get(0))?)
}

/// Porta lo schema all'ultima versione; ogni migrazione gira nella propria transazione.
/// Restituisce il numero di migrazioni applicate.
pub fn applica_migrazioni(connessione: &mut Connection) -> Result<u32, ErroreDatabase> {
    let versione_database = versione_schema_corrente(connessione)?;
    let versione_supportata = versione_schema_supportata();

    if versione_database > versione_supportata {
        return Err(ErroreDatabase::VersioneDatabaseSuccessiva {
            versione_database,
            versione_supportata,
        });
    }

    for (indice, sql) in MIGRAZIONI
        .iter()
        .enumerate()
        .skip(versione_database as usize)
    {
        let nuova_versione = indice as u32 + 1;
        let transazione = connessione.transaction()?;
        transazione.execute_batch(sql)?;
        transazione.pragma_update(None, "user_version", nuova_versione)?;
        transazione.commit()?;
    }

    Ok(versione_supportata - versione_database)
}

/// Vero se ci sono migrazioni da applicare (serve per fare il backup prima).
pub fn ci_sono_migrazioni_da_applicare(connessione: &Connection) -> Result<bool, ErroreDatabase> {
    Ok(versione_schema_corrente(connessione)? < versione_schema_supportata())
}
