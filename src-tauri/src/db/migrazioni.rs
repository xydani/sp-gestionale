use rusqlite::Connection;

use super::ErroreDatabase;

/// Migrazioni in ordine: la posizione + 1 è la versione dello schema.
/// Una migrazione rilasciata non va mai modificata: se ne aggiunge una nuova.
const MIGRAZIONI: &[&str] = &[
    include_str!("../../migrations/0001_init.sql"),
    include_str!("../../migrations/0002_iscrizioni_e_anagrafica.sql"),
    include_str!("../../migrations/0003_promemoria.sql"),
    include_str!("../../migrations/0004_configurazione_iniziale.sql"),
    include_str!("../../migrations/0005_certificato_medico.sql"),
    include_str!("../../migrations/0006_eliminazione_iscritti.sql"),
];

pub fn versione_schema_supportata() -> u32 {
    MIGRAZIONI.len() as u32
}

pub fn versione_schema_corrente(connessione: &Connection) -> Result<u32, ErroreDatabase> {
    Ok(connessione.pragma_query_value(None, "user_version", |riga| riga.get(0))?)
}

/// Porta lo schema all'ultima versione; ogni migrazione gira nella propria transazione.
/// Restituisce il numero di migrazioni applicate.
pub fn applica_migrazioni(connessione: &mut Connection) -> Result<u32, ErroreDatabase> {
    applica_migrazioni_fino_a(connessione, versione_schema_supportata())
}

/// Come `applica_migrazioni`, ma si ferma alla versione indicata: serve ai test
/// per creare database "vecchi" e verificarne l'aggiornamento.
pub fn applica_migrazioni_fino_a(
    connessione: &mut Connection,
    versione_finale: u32,
) -> Result<u32, ErroreDatabase> {
    let versione_database = versione_schema_corrente(connessione)?;
    let versione_supportata = versione_schema_supportata();

    if versione_database > versione_supportata {
        return Err(ErroreDatabase::VersioneDatabaseSuccessiva {
            versione_database,
            versione_supportata,
        });
    }

    let versione_finale = versione_finale.min(versione_supportata);
    for (indice, sql) in MIGRAZIONI
        .iter()
        .enumerate()
        .take(versione_finale as usize)
        .skip(versione_database as usize)
    {
        let nuova_versione = indice as u32 + 1;
        let transazione = connessione.transaction()?;
        transazione.execute_batch(sql)?;
        transazione.pragma_update(None, "user_version", nuova_versione)?;
        transazione.commit()?;
    }

    Ok(versione_finale.saturating_sub(versione_database))
}

/// Il backup prima delle migrazioni serve solo se il database contiene già dati:
/// un database appena creato (versione 0) non ha nulla da proteggere.
pub fn serve_backup_pre_migrazione(versione_corrente: u32) -> bool {
    versione_corrente > 0 && versione_corrente < versione_schema_supportata()
}

/// Vero se ci sono migrazioni da applicare (serve per fare il backup prima).
pub fn ci_sono_migrazioni_da_applicare(connessione: &Connection) -> Result<bool, ErroreDatabase> {
    Ok(versione_schema_corrente(connessione)? < versione_schema_supportata())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn database_nuovo_non_richiede_backup_pre_migrazione() {
        assert!(!serve_backup_pre_migrazione(0));
    }

    #[test]
    fn database_aggiornato_non_richiede_backup_pre_migrazione() {
        assert!(!serve_backup_pre_migrazione(versione_schema_supportata()));
    }

    #[test]
    fn database_con_dati_e_schema_vecchio_richiede_backup() {
        // Vero solo quando esisteranno almeno due migrazioni
        let versione_supportata = versione_schema_supportata();
        if versione_supportata > 1 {
            assert!(serve_backup_pre_migrazione(versione_supportata - 1));
        }
    }
}
