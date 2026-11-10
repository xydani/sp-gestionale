//! Listino delle tipologie di abbonamento.

use rusqlite::{Connection, ErrorCode, Row, params};
use serde::{Deserialize, Serialize};

use super::ErroreDatabase;
use crate::domain::date::{Durata, UnitaDurata};
use crate::domain::validazione::{
    DURATA_MASSIMA_ABBONAMENTO_MESI, LUNGHEZZA_MASSIMA_NOME, controlla_importo,
};

/// Lo stesso limite della durata in mesi, espresso in giorni.
const DURATA_MASSIMA_ABBONAMENTO_GIORNI: u32 = 1095;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TipologiaAbbonamento {
    pub id: i64,
    pub nome: String,
    pub durata_valore: u32,
    pub durata_unita: UnitaDurata,
    pub prezzo_cent: i64,
    pub attiva: bool,
}

impl TipologiaAbbonamento {
    pub fn durata(&self) -> Durata {
        Durata {
            valore: self.durata_valore,
            unita: self.durata_unita,
        }
    }
}

/// Dati inseriti dall'utente per creare o modificare una tipologia.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatiTipologia {
    pub nome: String,
    pub durata_valore: u32,
    pub durata_unita: UnitaDurata,
    pub prezzo_cent: i64,
}

const COLONNE: &str = "id, nome, durata_valore, durata_unita, prezzo_cent, attiva";

pub fn elenca_tipologie(
    connessione: &Connection,
    solo_attive: bool,
) -> Result<Vec<TipologiaAbbonamento>, ErroreDatabase> {
    let mut query = connessione.prepare(&format!(
        "SELECT {COLONNE} FROM tipologie_abbonamento
         WHERE attiva = 1 OR ?1 = 0
         ORDER BY attiva DESC, durata_unita, durata_valore, nome"
    ))?;
    let tipologie = query
        .query_map([solo_attive], leggi_tipologia)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(tipologie)
}

pub fn trova_tipologia(
    connessione: &Connection,
    id: i64,
) -> Result<TipologiaAbbonamento, ErroreDatabase> {
    connessione
        .query_row(
            &format!("SELECT {COLONNE} FROM tipologie_abbonamento WHERE id = ?1"),
            [id],
            leggi_tipologia,
        )
        .map_err(|errore| match errore {
            rusqlite::Error::QueryReturnedNoRows => ErroreDatabase::NonTrovato,
            altro => altro.into(),
        })
}

pub fn crea_tipologia(
    connessione: &Connection,
    dati: &DatiTipologia,
) -> Result<TipologiaAbbonamento, ErroreDatabase> {
    let dati = valida_dati_tipologia(dati)?;
    connessione
        .execute(
            "INSERT INTO tipologie_abbonamento (nome, durata_valore, durata_unita, prezzo_cent)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                dati.nome,
                dati.durata_valore,
                testo_unita(dati.durata_unita),
                dati.prezzo_cent
            ],
        )
        .map_err(errore_nome_duplicato)?;
    trova_tipologia(connessione, connessione.last_insert_rowid())
}

/// Le modifiche valgono per i pagamenti futuri: lo storico conserva il nome copiato.
pub fn modifica_tipologia(
    connessione: &Connection,
    id: i64,
    dati: &DatiTipologia,
) -> Result<TipologiaAbbonamento, ErroreDatabase> {
    let dati = valida_dati_tipologia(dati)?;
    let righe_modificate = connessione
        .execute(
            "UPDATE tipologie_abbonamento
             SET nome = ?1, durata_valore = ?2, durata_unita = ?3, prezzo_cent = ?4
             WHERE id = ?5",
            params![
                dati.nome,
                dati.durata_valore,
                testo_unita(dati.durata_unita),
                dati.prezzo_cent,
                id
            ],
        )
        .map_err(errore_nome_duplicato)?;
    if righe_modificate == 0 {
        return Err(ErroreDatabase::NonTrovato);
    }
    trova_tipologia(connessione, id)
}

/// Le tipologie non si cancellano: si disattivano, così lo storico resta coerente.
pub fn imposta_tipologia_attiva(
    connessione: &Connection,
    id: i64,
    attiva: bool,
) -> Result<TipologiaAbbonamento, ErroreDatabase> {
    let righe_modificate = connessione.execute(
        "UPDATE tipologie_abbonamento SET attiva = ?1 WHERE id = ?2",
        params![attiva, id],
    )?;
    if righe_modificate == 0 {
        return Err(ErroreDatabase::NonTrovato);
    }
    trova_tipologia(connessione, id)
}

/// Pulisce i dati (spazi nel nome) e li controlla.
pub fn valida_dati_tipologia(dati: &DatiTipologia) -> Result<DatiTipologia, ErroreDatabase> {
    let nome = dati.nome.trim().to_string();
    if nome.is_empty() {
        return Err(dati_non_validi("Il nome della tipologia è obbligatorio"));
    }
    if nome.chars().count() > LUNGHEZZA_MASSIMA_NOME {
        return Err(dati_non_validi(&format!(
            "Il nome della tipologia può avere al massimo {LUNGHEZZA_MASSIMA_NOME} caratteri"
        )));
    }
    if dati.durata_valore == 0 {
        return Err(dati_non_validi("La durata deve essere maggiore di zero"));
    }
    let durata_massima = match dati.durata_unita {
        UnitaDurata::Giorni => DURATA_MASSIMA_ABBONAMENTO_GIORNI,
        UnitaDurata::Mesi => DURATA_MASSIMA_ABBONAMENTO_MESI,
    };
    if dati.durata_valore > durata_massima {
        return Err(dati_non_validi(&format!(
            "La durata è troppo lunga: al massimo {durata_massima} {}",
            testo_unita(dati.durata_unita)
        )));
    }
    controlla_importo(dati.prezzo_cent, "Il prezzo").map_err(ErroreDatabase::DatiNonValidi)?;
    Ok(DatiTipologia {
        nome,
        ..dati.clone()
    })
}

pub fn testo_unita(unita: UnitaDurata) -> &'static str {
    match unita {
        UnitaDurata::Giorni => "giorni",
        UnitaDurata::Mesi => "mesi",
    }
}

pub fn unita_da_testo(testo: &str) -> Option<UnitaDurata> {
    match testo {
        "giorni" => Some(UnitaDurata::Giorni),
        "mesi" => Some(UnitaDurata::Mesi),
        _ => None,
    }
}

fn leggi_tipologia(riga: &Row<'_>) -> rusqlite::Result<TipologiaAbbonamento> {
    let testo: String = riga.get(3)?;
    let durata_unita = unita_da_testo(&testo).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            3,
            rusqlite::types::Type::Text,
            format!("unità di durata sconosciuta: {testo}").into(),
        )
    })?;
    Ok(TipologiaAbbonamento {
        id: riga.get(0)?,
        nome: riga.get(1)?,
        durata_valore: riga.get(2)?,
        durata_unita,
        prezzo_cent: riga.get(4)?,
        attiva: riga.get(5)?,
    })
}

fn errore_nome_duplicato(errore: rusqlite::Error) -> ErroreDatabase {
    match &errore {
        rusqlite::Error::SqliteFailure(dettaglio, _)
            if dettaglio.code == ErrorCode::ConstraintViolation =>
        {
            dati_non_validi("Esiste già una tipologia con questo nome")
        }
        _ => errore.into(),
    }
}

fn dati_non_validi(messaggio: &str) -> ErroreDatabase {
    ErroreDatabase::DatiNonValidi(messaggio.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn dati(nome: &str, durata_valore: u32, prezzo_cent: i64) -> DatiTipologia {
        DatiTipologia {
            nome: nome.into(),
            durata_valore,
            durata_unita: UnitaDurata::Mesi,
            prezzo_cent,
        }
    }

    #[test]
    fn il_nome_viene_ripulito_dagli_spazi() {
        assert_eq!(
            valida_dati_tipologia(&dati("  Mensile  ", 1, 5000))
                .unwrap()
                .nome,
            "Mensile"
        );
    }

    #[test]
    fn nome_vuoto_durata_zero_e_prezzo_negativo_sono_rifiutati() {
        assert!(valida_dati_tipologia(&dati("   ", 1, 5000)).is_err());
        assert!(valida_dati_tipologia(&dati("Mensile", 0, 5000)).is_err());
        assert!(valida_dati_tipologia(&dati("Mensile", 1, -1)).is_err());
    }

    #[test]
    fn prezzo_zero_e_ammesso() {
        assert!(valida_dati_tipologia(&dati("Prova gratuita", 7, 0)).is_ok());
    }

    #[test]
    fn unita_testo_andata_e_ritorno() {
        for unita in [UnitaDurata::Giorni, UnitaDurata::Mesi] {
            assert_eq!(unita_da_testo(testo_unita(unita)), Some(unita));
        }
        assert_eq!(unita_da_testo("settimane"), None);
    }
}
