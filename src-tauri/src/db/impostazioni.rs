use rusqlite::{Connection, OptionalExtension};

use super::ErroreDatabase;

pub const CHIAVE_BACKUP_CONSERVATI: &str = "backup_conservati";
pub const BACKUP_CONSERVATI_PREDEFINITI: usize = 30;
pub const CHIAVE_GIORNI_PREAVVISO: &str = "giorni_preavviso";

/// Legge un'impostazione numerica; se manca o non è un numero usa il valore predefinito.
pub fn leggi_impostazione_numerica(
    connessione: &Connection,
    chiave: &str,
    predefinito: usize,
) -> Result<usize, ErroreDatabase> {
    let valore: Option<String> = connessione
        .query_row(
            "SELECT valore FROM impostazioni WHERE chiave = ?1",
            [chiave],
            |riga| riga.get(0),
        )
        .optional()?;
    Ok(valore
        .and_then(|testo| testo.parse().ok())
        .unwrap_or(predefinito))
}
