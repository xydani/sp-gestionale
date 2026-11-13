//! Eliminazione definitiva dei dati di un iscritto su sua richiesta (diritto all'oblio, GDPR).

use chrono::NaiveDate;
use rusqlite::{Connection, OptionalExtension, params};

use super::ErroreDatabase;

/// Nome che sostituisce quello vero: i pagamenti dell'iscritto restano, ma anonimi.
pub const NOME_ISCRITTO_ELIMINATO: &str = "Iscritto";
pub const COGNOME_ISCRITTO_ELIMINATO: &str = "eliminato";

/// Cancella l'anagrafica e le note di un iscritto archiviato. Pagamenti e iscrizioni restano
/// (senza note) così i totali degli incassi non cambiano; l'iscritto sparisce da elenchi e ricerche.
pub fn elimina_dati_iscritto(
    connessione: &mut Connection,
    id: i64,
    oggi: NaiveDate,
) -> Result<(), ErroreDatabase> {
    let archiviato: bool = connessione
        .query_row(
            "SELECT archiviato FROM iscritti WHERE id = ?1 AND data_eliminazione IS NULL",
            [id],
            |riga| riga.get(0),
        )
        .optional()?
        .ok_or(ErroreDatabase::NonTrovato)?;
    if !archiviato {
        return Err(ErroreDatabase::non_valido(
            "Prima di eliminarlo definitivamente l'iscritto va archiviato",
        ));
    }

    // Le pagine liberate vengono azzerate: nel file non restano copie dei dati cancellati
    connessione.pragma_update(None, "secure_delete", true)?;
    let transazione = connessione.transaction()?;
    transazione.execute(
        "UPDATE iscritti
         SET nome = ?1, cognome = ?2, data_nascita = NULL, luogo_nascita = NULL, telefono = NULL,
             email = NULL, note = NULL, data_scadenza_certificato = NULL,
             certificato_senza_scadenza = 0,
             data_eliminazione = ?3, modificato_il = datetime('now')
         WHERE id = ?4",
        params![
            NOME_ISCRITTO_ELIMINATO,
            COGNOME_ISCRITTO_ELIMINATO,
            oggi,
            id
        ],
    )?;
    transazione.execute(
        "UPDATE pagamenti SET note = NULL WHERE iscritto_id = ?1",
        [id],
    )?;
    transazione.execute(
        "UPDATE iscrizioni SET note = NULL WHERE iscritto_id = ?1",
        [id],
    )?;
    transazione.execute(
        "DELETE FROM promemoria_inviati WHERE iscritto_id = ?1",
        [id],
    )?;
    transazione.commit()?;
    // Anche il registro WAL conteneva le versioni precedenti delle pagine
    connessione.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |_| Ok(()))?;
    Ok(())
}
