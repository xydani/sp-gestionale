//! Pagamenti e rinnovi degli abbonamenti.

use chrono::NaiveDate;
use rusqlite::types::Value;
use rusqlite::{Connection, OptionalExtension, Row, params, params_from_iter};
use serde::{Deserialize, Serialize};

use super::paginazione::{ELEMENTI_PER_PAGINA, Pagina, RichiestaPagina};
use super::tipologie::{TipologiaAbbonamento, trova_tipologia};
use super::{ErroreDatabase, errore_inserimento_per_iscritto};
use crate::domain::date::calcola_data_scadenza;
use crate::domain::pagamento::MetodoPagamento;
use crate::domain::rinnovo::calcola_data_inizio_rinnovo;
use crate::domain::testo::testo_facoltativo;
use crate::domain::validazione::{
    DURATA_MASSIMA_ABBONAMENTO_MESI, EsitoControllo, controlla_data_pagamento, controlla_importo,
    controlla_note, controlla_periodo,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagamento {
    pub id: i64,
    pub iscritto_id: i64,
    pub iscritto_nome: String,
    pub iscritto_cognome: String,
    pub tipologia_id: Option<i64>,
    pub tipologia_nome: String,
    pub importo_cent: i64,
    pub data_pagamento: NaiveDate,
    pub metodo: MetodoPagamento,
    pub data_inizio: NaiveDate,
    pub data_scadenza: NaiveDate,
    pub note: Option<String>,
    pub rinnovo_di_id: Option<i64>,
    /// I dati dell'iscritto sono stati cancellati su richiesta: resta solo il pagamento.
    pub iscritto_eliminato: bool,
}

/// Dati inseriti dall'utente per registrare un pagamento.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatiPagamento {
    pub iscritto_id: i64,
    pub tipologia_id: i64,
    pub importo_cent: i64,
    pub data_pagamento: NaiveDate,
    pub metodo: MetodoPagamento,
    pub data_inizio: NaiveDate,
    pub data_scadenza: NaiveDate,
    pub note: Option<String>,
    pub rinnovo_di_id: Option<i64>,
}

/// Valori proposti nel modulo di pagamento: tutti modificabili dall'utente.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropostaPagamento {
    pub importo_cent: i64,
    pub data_inizio: NaiveDate,
    pub data_scadenza: NaiveDate,
    /// Pagamento con la scadenza più recente: il nuovo pagamento ne è il rinnovo.
    pub rinnovo_di_id: Option<i64>,
}

/// Una pagina di pagamenti con il totale incassato di tutti quelli filtrati.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginaPagamenti {
    #[serde(flatten)]
    pub pagina: Pagina<Pagamento>,
    pub totale_importo_cent: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiltroPagamenti {
    /// Date di pagamento comprese, estremi inclusi.
    pub dal: Option<NaiveDate>,
    pub al: Option<NaiveDate>,
    pub metodo: Option<MetodoPagamento>,
    pub iscritto_id: Option<i64>,
}

const SELEZIONE_PAGAMENTI: &str = "
    SELECT p.id, p.iscritto_id, i.nome, i.cognome, p.tipologia_id, p.tipologia_nome,
           p.importo_cent, p.data_pagamento, p.metodo, p.data_inizio, p.data_scadenza,
           p.note, p.rinnovo_di_id, i.data_eliminazione IS NOT NULL
    FROM pagamenti p
    JOIN iscritti i ON i.id = p.iscritto_id";

/// Propone importo e date per un nuovo pagamento dell'iscritto con la tipologia scelta.
/// Se `data_inizio` non è indicata, il nuovo abbonamento parte dopo l'ultima scadenza
/// (se ancora valida) oppure da oggi.
/// Senza `iscritto_id` (iscritto non ancora creato) l'abbonamento parte da oggi.
pub fn proponi_pagamento(
    connessione: &Connection,
    iscritto_id: Option<i64>,
    tipologia_id: i64,
    data_inizio: Option<NaiveDate>,
    oggi: NaiveDate,
) -> Result<PropostaPagamento, ErroreDatabase> {
    let tipologia = trova_tipologia(connessione, tipologia_id)?;
    let ultimo = ultimo_pagamento(connessione, iscritto_id)?;
    let data_inizio = data_inizio.unwrap_or_else(|| {
        calcola_data_inizio_rinnovo(ultimo.map(|(_, data_scadenza)| data_scadenza), oggi)
    });
    let data_scadenza = calcola_data_scadenza(data_inizio, tipologia.durata())
        .map_err(|errore| ErroreDatabase::non_valido(errore.to_string()))?;
    Ok(PropostaPagamento {
        importo_cent: tipologia.prezzo_cent,
        data_inizio,
        data_scadenza,
        rinnovo_di_id: ultimo.map(|(id, _)| id),
    })
}

/// `oggi` serve a rifiutare pagamenti con date non plausibili (es. nel futuro).
pub fn registra_pagamento(
    connessione: &Connection,
    dati: &DatiPagamento,
    oggi: NaiveDate,
) -> Result<Pagamento, ErroreDatabase> {
    valida_dati_pagamento(dati, oggi)?;
    // Il nome della tipologia viene copiato: lo storico non cambia se il listino cambia
    let tipologia = trova_tipologia_in_vendita(connessione, dati.tipologia_id)?;
    let note = testo_facoltativo(dati.note.as_deref());
    connessione
        .execute(
            "INSERT INTO pagamenti (iscritto_id, tipologia_id, tipologia_nome, importo_cent,
                 data_pagamento, metodo, data_inizio, data_scadenza, note, rinnovo_di_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                dati.iscritto_id,
                tipologia.id,
                tipologia.nome,
                dati.importo_cent,
                dati.data_pagamento,
                dati.metodo.come_testo(),
                dati.data_inizio,
                dati.data_scadenza,
                note,
                dati.rinnovo_di_id,
            ],
        )
        .map_err(errore_inserimento_per_iscritto)?;
    trova_pagamento(connessione, connessione.last_insert_rowid())
}

pub fn trova_pagamento(connessione: &Connection, id: i64) -> Result<Pagamento, ErroreDatabase> {
    connessione
        .query_row(
            &format!("{SELEZIONE_PAGAMENTI} WHERE p.id = ?1"),
            [id],
            leggi_pagamento,
        )
        .optional()?
        .ok_or(ErroreDatabase::NonTrovato)
}

/// Pagamenti più recenti per primi.
pub fn elenca_pagamenti(
    connessione: &Connection,
    filtro: &FiltroPagamenti,
) -> Result<Vec<Pagamento>, ErroreDatabase> {
    let (clausola_where, parametri) = condizioni_filtro(filtro);
    let mut query = connessione.prepare(&format!(
        "{SELEZIONE_PAGAMENTI} {clausola_where} ORDER BY p.data_pagamento DESC, p.id DESC"
    ))?;
    let pagamenti = query
        .query_map(params_from_iter(parametri), leggi_pagamento)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(pagamenti)
}

/// Una pagina di pagamenti, più recenti per primi; conteggio e totale valgono per tutto il filtro.
pub fn elenca_pagamenti_paginati(
    connessione: &Connection,
    filtro: &FiltroPagamenti,
    richiesta: RichiestaPagina,
) -> Result<PaginaPagamenti, ErroreDatabase> {
    let (clausola_where, parametri) = condizioni_filtro(filtro);

    let (totale_elementi, totale_importo_cent): (u32, i64) = connessione.query_row(
        &format!(
            "SELECT count(*), coalesce(sum(p.importo_cent), 0) FROM pagamenti p {clausola_where}"
        ),
        params_from_iter(parametri.iter()),
        |riga| Ok((riga.get(0)?, riga.get(1)?)),
    )?;

    let mut parametri_pagina = parametri;
    parametri_pagina.push(Value::from(i64::from(ELEMENTI_PER_PAGINA)));
    parametri_pagina.push(Value::from(i64::from(richiesta.scarto())));
    let mut query = connessione.prepare(&format!(
        "{SELEZIONE_PAGAMENTI} {clausola_where}
         ORDER BY p.data_pagamento DESC, p.id DESC LIMIT ? OFFSET ?"
    ))?;
    let elementi = query
        .query_map(params_from_iter(parametri_pagina), leggi_pagamento)?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(PaginaPagamenti {
        pagina: Pagina {
            elementi,
            numero: richiesta.numero.max(1),
            elementi_per_pagina: ELEMENTI_PER_PAGINA,
            totale_elementi,
        },
        totale_importo_cent,
    })
}

/// Clausola WHERE e parametri corrispondenti al filtro.
fn condizioni_filtro(filtro: &FiltroPagamenti) -> (String, Vec<Value>) {
    let mut condizioni: Vec<&str> = Vec::new();
    let mut parametri: Vec<Value> = Vec::new();
    if let Some(dal) = filtro.dal {
        condizioni.push("p.data_pagamento >= ?");
        parametri.push(Value::from(dal.to_string()));
    }
    if let Some(al) = filtro.al {
        condizioni.push("p.data_pagamento <= ?");
        parametri.push(Value::from(al.to_string()));
    }
    if let Some(metodo) = filtro.metodo {
        condizioni.push("p.metodo = ?");
        parametri.push(Value::from(metodo.come_testo().to_string()));
    }
    if let Some(iscritto_id) = filtro.iscritto_id {
        condizioni.push("p.iscritto_id = ?");
        parametri.push(Value::from(iscritto_id));
    }
    let clausola_where = if condizioni.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", condizioni.join(" AND "))
    };
    (clausola_where, parametri)
}

/// Elimina un pagamento registrato per errore. Gli eventuali rinnovi collegati restano.
pub fn elimina_pagamento(connessione: &Connection, id: i64) -> Result<(), ErroreDatabase> {
    let righe_eliminate = connessione.execute("DELETE FROM pagamenti WHERE id = ?1", [id])?;
    if righe_eliminate == 0 {
        return Err(ErroreDatabase::NonTrovato);
    }
    Ok(())
}

pub fn valida_dati_pagamento(dati: &DatiPagamento, oggi: NaiveDate) -> Result<(), ErroreDatabase> {
    controlla_pagamento(dati, oggi).map_err(ErroreDatabase::non_valido)
}

fn controlla_pagamento(dati: &DatiPagamento, oggi: NaiveDate) -> EsitoControllo {
    controlla_importo(dati.importo_cent, "L'importo")?;
    controlla_data_pagamento(dati.data_pagamento, oggi)?;
    controlla_periodo(
        dati.data_inizio,
        dati.data_scadenza,
        dati.data_pagamento,
        DURATA_MASSIMA_ABBONAMENTO_MESI,
    )?;
    if let Some(note) = &dati.note {
        controlla_note(note)?;
    }
    Ok(())
}

/// Lo storico tiene i tipi disattivati, ma i pagamenti nuovi usano solo il listino in vigore.
fn trova_tipologia_in_vendita(
    connessione: &Connection,
    tipologia_id: i64,
) -> Result<TipologiaAbbonamento, ErroreDatabase> {
    let tipologia = trova_tipologia(connessione, tipologia_id)?;
    if !tipologia.attiva {
        return Err(ErroreDatabase::non_valido(format!(
            "L'abbonamento «{}» non è più nel listino: sceglierne uno attivo",
            tipologia.nome
        )));
    }
    Ok(tipologia)
}

/// Id e scadenza del pagamento con la scadenza più lontana dell'iscritto.
fn ultimo_pagamento(
    connessione: &Connection,
    iscritto_id: Option<i64>,
) -> Result<Option<(i64, NaiveDate)>, ErroreDatabase> {
    Ok(connessione
        .query_row(
            "SELECT id, data_scadenza FROM pagamenti WHERE iscritto_id = ?1
             ORDER BY data_scadenza DESC, id DESC LIMIT 1",
            [iscritto_id],
            |riga| Ok((riga.get(0)?, riga.get(1)?)),
        )
        .optional()?)
}

fn leggi_pagamento(riga: &Row<'_>) -> rusqlite::Result<Pagamento> {
    let testo_metodo: String = riga.get(8)?;
    let metodo = MetodoPagamento::da_testo(&testo_metodo).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            8,
            rusqlite::types::Type::Text,
            format!("metodo di pagamento sconosciuto: {testo_metodo}").into(),
        )
    })?;
    Ok(Pagamento {
        id: riga.get(0)?,
        iscritto_id: riga.get(1)?,
        iscritto_nome: riga.get(2)?,
        iscritto_cognome: riga.get(3)?,
        tipologia_id: riga.get(4)?,
        tipologia_nome: riga.get(5)?,
        importo_cent: riga.get(6)?,
        data_pagamento: riga.get(7)?,
        metodo,
        data_inizio: riga.get(9)?,
        data_scadenza: riga.get(10)?,
        note: riga.get(11)?,
        rinnovo_di_id: riga.get(12)?,
        iscritto_eliminato: riga.get(13)?,
    })
}
