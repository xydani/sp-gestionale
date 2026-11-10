//! Anagrafica degli iscritti.

use chrono::NaiveDate;
use rusqlite::types::Value;
use rusqlite::{Connection, OptionalExtension, Row, params, params_from_iter};
use serde::{Deserialize, Serialize};

use super::ErroreDatabase;
use super::impostazioni::{CHIAVE_GIORNI_PREAVVISO, leggi_impostazione_numerica};
use crate::domain::stato::{
    GIORNI_PREAVVISO_PREDEFINITI, StatoAbbonamento, calcola_stato_abbonamento,
};
use crate::domain::testo::parole_di_ricerca;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Iscritto {
    pub id: i64,
    pub nome: String,
    pub cognome: String,
    pub telefono: Option<String>,
    pub data_iscrizione: NaiveDate,
    pub note: Option<String>,
    pub archiviato: bool,
    pub ultima_data_scadenza: Option<NaiveDate>,
    pub stato: StatoAbbonamento,
}

/// Dati inseriti dall'utente per creare o modificare un iscritto.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatiIscritto {
    pub nome: String,
    pub cognome: String,
    pub telefono: Option<String>,
    pub data_iscrizione: NaiveDate,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiltroIscritti {
    /// Parole cercate in nome e cognome, senza distinzione di maiuscole e accenti.
    pub testo: String,
    pub includi_archiviati: bool,
    pub stato: Option<StatoAbbonamento>,
}

/// Data di oggi e preavviso: servono a calcolare lo stato dell'abbonamento.
#[derive(Debug, Clone, Copy)]
pub struct Riferimento {
    pub oggi: NaiveDate,
    pub giorni_preavviso: u32,
}

/// Costruisce il riferimento leggendo i giorni di preavviso dalle impostazioni.
pub fn riferimento_per_oggi(
    connessione: &Connection,
    oggi: NaiveDate,
) -> Result<Riferimento, ErroreDatabase> {
    let giorni_preavviso = leggi_impostazione_numerica(
        connessione,
        CHIAVE_GIORNI_PREAVVISO,
        GIORNI_PREAVVISO_PREDEFINITI as usize,
    )?;
    Ok(Riferimento {
        oggi,
        giorni_preavviso: giorni_preavviso as u32,
    })
}

const SELEZIONE_ISCRITTI: &str = "
    SELECT i.id, i.nome, i.cognome, i.telefono, i.data_iscrizione, i.note, i.archiviato,
           u.ultima_data_scadenza
    FROM iscritti i
    JOIN v_ultima_scadenza_iscritti u ON u.iscritto_id = i.id";

pub fn elenca_iscritti(
    connessione: &Connection,
    filtro: &FiltroIscritti,
    riferimento: Riferimento,
) -> Result<Vec<Iscritto>, ErroreDatabase> {
    let mut condizioni = vec!["(i.archiviato = 0 OR ?1 = 1)".to_string()];
    let mut parametri: Vec<Value> = vec![Value::from(filtro.includi_archiviati)];

    // Ogni parola deve comparire nel nome o nel cognome: "ross mar" trova "Mario Rossi"
    for parola in parole_di_ricerca(&filtro.testo) {
        parametri.push(Value::from(format!("%{}%", escape_like(&parola))));
        let indice = parametri.len();
        condizioni.push(format!(
            "(normalizza(i.nome) LIKE ?{indice} ESCAPE '\\' OR normalizza(i.cognome) LIKE ?{indice} ESCAPE '\\')"
        ));
    }

    let sql = format!(
        "{SELEZIONE_ISCRITTI} WHERE {} ORDER BY i.cognome COLLATE NOCASE, i.nome COLLATE NOCASE, i.id",
        condizioni.join(" AND ")
    );
    let mut query = connessione.prepare(&sql)?;
    let iscritti = query
        .query_map(params_from_iter(parametri), |riga| {
            leggi_iscritto(riga, riferimento)
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(match filtro.stato {
        Some(stato) => iscritti
            .into_iter()
            .filter(|iscritto| iscritto.stato == stato)
            .collect(),
        None => iscritti,
    })
}

pub fn trova_iscritto(
    connessione: &Connection,
    id: i64,
    riferimento: Riferimento,
) -> Result<Iscritto, ErroreDatabase> {
    connessione
        .query_row(
            &format!("{SELEZIONE_ISCRITTI} WHERE i.id = ?1"),
            [id],
            |riga| leggi_iscritto(riga, riferimento),
        )
        .optional()?
        .ok_or(ErroreDatabase::NonTrovato)
}

pub fn crea_iscritto(
    connessione: &Connection,
    dati: &DatiIscritto,
    riferimento: Riferimento,
) -> Result<Iscritto, ErroreDatabase> {
    let dati = valida_dati_iscritto(dati)?;
    connessione.execute(
        "INSERT INTO iscritti (nome, cognome, telefono, data_iscrizione, note)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            dati.nome,
            dati.cognome,
            dati.telefono,
            dati.data_iscrizione.to_string(),
            dati.note
        ],
    )?;
    trova_iscritto(connessione, connessione.last_insert_rowid(), riferimento)
}

pub fn modifica_iscritto(
    connessione: &Connection,
    id: i64,
    dati: &DatiIscritto,
    riferimento: Riferimento,
) -> Result<Iscritto, ErroreDatabase> {
    let dati = valida_dati_iscritto(dati)?;
    let righe_modificate = connessione.execute(
        "UPDATE iscritti
         SET nome = ?1, cognome = ?2, telefono = ?3, data_iscrizione = ?4, note = ?5,
             modificato_il = datetime('now')
         WHERE id = ?6",
        params![
            dati.nome,
            dati.cognome,
            dati.telefono,
            dati.data_iscrizione.to_string(),
            dati.note,
            id
        ],
    )?;
    if righe_modificate == 0 {
        return Err(ErroreDatabase::NonTrovato);
    }
    trova_iscritto(connessione, id, riferimento)
}

/// Gli iscritti non si cancellano: si archiviano, così lo storico dei pagamenti resta.
pub fn imposta_iscritto_archiviato(
    connessione: &Connection,
    id: i64,
    archiviato: bool,
    riferimento: Riferimento,
) -> Result<Iscritto, ErroreDatabase> {
    let righe_modificate = connessione.execute(
        "UPDATE iscritti SET archiviato = ?1, modificato_il = datetime('now') WHERE id = ?2",
        params![archiviato, id],
    )?;
    if righe_modificate == 0 {
        return Err(ErroreDatabase::NonTrovato);
    }
    trova_iscritto(connessione, id, riferimento)
}

/// Pulisce i dati (spazi, campi facoltativi vuoti) e controlla quelli obbligatori.
pub fn valida_dati_iscritto(dati: &DatiIscritto) -> Result<DatiIscritto, ErroreDatabase> {
    let nome = dati.nome.trim().to_string();
    let cognome = dati.cognome.trim().to_string();
    if nome.is_empty() {
        return Err(ErroreDatabase::DatiNonValidi(
            "Il nome è obbligatorio".into(),
        ));
    }
    if cognome.is_empty() {
        return Err(ErroreDatabase::DatiNonValidi(
            "Il cognome è obbligatorio".into(),
        ));
    }
    Ok(DatiIscritto {
        nome,
        cognome,
        telefono: testo_facoltativo(dati.telefono.as_deref()),
        data_iscrizione: dati.data_iscrizione,
        note: testo_facoltativo(dati.note.as_deref()),
    })
}

fn testo_facoltativo(testo: Option<&str>) -> Option<String> {
    testo
        .map(str::trim)
        .filter(|testo| !testo.is_empty())
        .map(str::to_string)
}

/// I caratteri % e _ sono jolly in LIKE: vanno protetti se l'utente li scrive.
fn escape_like(testo: &str) -> String {
    testo
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

fn leggi_iscritto(riga: &Row<'_>, riferimento: Riferimento) -> rusqlite::Result<Iscritto> {
    let ultima_data_scadenza: Option<NaiveDate> = riga.get(7)?;
    Ok(Iscritto {
        id: riga.get(0)?,
        nome: riga.get(1)?,
        cognome: riga.get(2)?,
        telefono: riga.get(3)?,
        data_iscrizione: riga.get(4)?,
        note: riga.get(5)?,
        archiviato: riga.get(6)?,
        ultima_data_scadenza,
        stato: calcola_stato_abbonamento(
            ultima_data_scadenza,
            riferimento.oggi,
            riferimento.giorni_preavviso,
        ),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    fn dati(nome: &str, cognome: &str) -> DatiIscritto {
        DatiIscritto {
            nome: nome.into(),
            cognome: cognome.into(),
            telefono: Some("   ".into()),
            data_iscrizione: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            note: Some(" nota ".into()),
        }
    }

    #[test]
    fn la_validazione_ripulisce_spazi_e_campi_vuoti() {
        let puliti = valida_dati_iscritto(&dati("  Mario ", " Rossi ")).unwrap();
        assert_eq!(puliti.nome, "Mario");
        assert_eq!(puliti.cognome, "Rossi");
        assert_eq!(puliti.telefono, None);
        assert_eq!(puliti.note.as_deref(), Some("nota"));
    }

    #[test]
    fn nome_e_cognome_sono_obbligatori() {
        assert!(valida_dati_iscritto(&dati(" ", "Rossi")).is_err());
        assert!(valida_dati_iscritto(&dati("Mario", "")).is_err());
    }

    #[test]
    fn i_jolly_di_like_sono_protetti() {
        assert_eq!(escape_like("50%_a\\b"), "50\\%\\_a\\\\b");
    }
}
