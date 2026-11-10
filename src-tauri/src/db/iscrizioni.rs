//! Iscrizioni annuali (anno sportivo) con la quota pagata.

use chrono::NaiveDate;
use rusqlite::{Connection, OptionalExtension, Row, params};
use serde::{Deserialize, Serialize};

use super::ErroreDatabase;
use super::impostazioni::{
    CHIAVE_MESE_INIZIO_ANNO_SPORTIVO, CHIAVE_QUOTA_ISCRIZIONE_CENT,
    QUOTA_ISCRIZIONE_PREDEFINITA_CENT, leggi_impostazione_numerica,
};
use crate::domain::anno_sportivo::{
    MESE_INIZIO_ANNO_SPORTIVO_PREDEFINITO, proponi_periodo_iscrizione,
};
use crate::domain::pagamento::MetodoPagamento;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Iscrizione {
    pub id: i64,
    pub iscritto_id: i64,
    pub importo_cent: i64,
    pub data_pagamento: NaiveDate,
    pub metodo: MetodoPagamento,
    pub data_inizio: NaiveDate,
    pub data_fine: NaiveDate,
    pub note: Option<String>,
}

/// Dati inseriti dall'utente per registrare un'iscrizione.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatiIscrizione {
    pub iscritto_id: i64,
    pub importo_cent: i64,
    pub data_pagamento: NaiveDate,
    pub metodo: MetodoPagamento,
    pub data_inizio: NaiveDate,
    pub data_fine: NaiveDate,
    pub note: Option<String>,
}

/// Quota e periodo proposti per una nuova iscrizione, tutti modificabili.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropostaIscrizione {
    pub importo_cent: i64,
    pub data_inizio: NaiveDate,
    pub data_fine: NaiveDate,
}

const SELEZIONE_ISCRIZIONI: &str = "
    SELECT id, iscritto_id, importo_cent, data_pagamento, metodo, data_inizio, data_fine, note
    FROM iscrizioni";

/// Propone la quota dalle impostazioni e il periodo dell'anno sportivo:
/// il successivo se l'iscrizione è ancora valida, altrimenti quello in corso.
/// Senza `iscritto_id` (iscritto non ancora creato) si propone l'anno sportivo in corso.
pub fn proponi_iscrizione(
    connessione: &Connection,
    iscritto_id: Option<i64>,
    oggi: NaiveDate,
) -> Result<PropostaIscrizione, ErroreDatabase> {
    let mese_inizio = leggi_impostazione_numerica(
        connessione,
        CHIAVE_MESE_INIZIO_ANNO_SPORTIVO,
        MESE_INIZIO_ANNO_SPORTIVO_PREDEFINITO as usize,
    )?;
    let quota_cent = leggi_impostazione_numerica(
        connessione,
        CHIAVE_QUOTA_ISCRIZIONE_CENT,
        QUOTA_ISCRIZIONE_PREDEFINITA_CENT,
    )?;
    let ultima_data_fine: Option<NaiveDate> = connessione.query_row(
        "SELECT max(data_fine) FROM iscrizioni WHERE iscritto_id = ?1",
        [iscritto_id],
        |riga| riga.get(0),
    )?;
    let periodo = proponi_periodo_iscrizione(ultima_data_fine, oggi, mese_inizio as u32);
    Ok(PropostaIscrizione {
        importo_cent: quota_cent as i64,
        data_inizio: periodo.inizio,
        data_fine: periodo.fine,
    })
}

pub fn registra_iscrizione(
    connessione: &Connection,
    dati: &DatiIscrizione,
) -> Result<Iscrizione, ErroreDatabase> {
    if dati.importo_cent < 0 {
        return Err(dati_non_validi("La quota non può essere negativa"));
    }
    if dati.data_fine < dati.data_inizio {
        return Err(dati_non_validi(
            "La fine non può essere precedente all'inizio",
        ));
    }
    let note = dati
        .note
        .as_deref()
        .map(str::trim)
        .filter(|nota| !nota.is_empty());
    connessione
        .execute(
            "INSERT INTO iscrizioni (iscritto_id, importo_cent, data_pagamento, metodo,
                 data_inizio, data_fine, note)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                dati.iscritto_id,
                dati.importo_cent,
                dati.data_pagamento,
                dati.metodo.come_testo(),
                dati.data_inizio,
                dati.data_fine,
                note,
            ],
        )
        .map_err(|errore| match &errore {
            rusqlite::Error::SqliteFailure(dettaglio, _)
                if dettaglio.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_FOREIGNKEY =>
            {
                dati_non_validi("Iscritto non trovato")
            }
            _ => errore.into(),
        })?;
    trova_iscrizione(connessione, connessione.last_insert_rowid())
}

pub fn trova_iscrizione(connessione: &Connection, id: i64) -> Result<Iscrizione, ErroreDatabase> {
    connessione
        .query_row(
            &format!("{SELEZIONE_ISCRIZIONI} WHERE id = ?1"),
            [id],
            leggi_iscrizione,
        )
        .optional()?
        .ok_or(ErroreDatabase::NonTrovato)
}

/// Storico delle iscrizioni di un iscritto, la più recente per prima.
pub fn elenca_iscrizioni(
    connessione: &Connection,
    iscritto_id: i64,
) -> Result<Vec<Iscrizione>, ErroreDatabase> {
    let mut query = connessione.prepare(&format!(
        "{SELEZIONE_ISCRIZIONI} WHERE iscritto_id = ?1 ORDER BY data_inizio DESC, id DESC"
    ))?;
    let iscrizioni = query
        .query_map([iscritto_id], leggi_iscrizione)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(iscrizioni)
}

/// Elimina un'iscrizione registrata per errore.
pub fn elimina_iscrizione(connessione: &Connection, id: i64) -> Result<(), ErroreDatabase> {
    let righe_eliminate = connessione.execute("DELETE FROM iscrizioni WHERE id = ?1", [id])?;
    if righe_eliminate == 0 {
        return Err(ErroreDatabase::NonTrovato);
    }
    Ok(())
}

fn dati_non_validi(messaggio: &str) -> ErroreDatabase {
    ErroreDatabase::DatiNonValidi(messaggio.to_string())
}

fn leggi_iscrizione(riga: &Row<'_>) -> rusqlite::Result<Iscrizione> {
    let testo_metodo: String = riga.get(4)?;
    let metodo = MetodoPagamento::da_testo(&testo_metodo).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            4,
            rusqlite::types::Type::Text,
            format!("metodo di pagamento sconosciuto: {testo_metodo}").into(),
        )
    })?;
    Ok(Iscrizione {
        id: riga.get(0)?,
        iscritto_id: riga.get(1)?,
        importo_cent: riga.get(2)?,
        data_pagamento: riga.get(3)?,
        metodo,
        data_inizio: riga.get(5)?,
        data_fine: riga.get(6)?,
        note: riga.get(7)?,
    })
}
