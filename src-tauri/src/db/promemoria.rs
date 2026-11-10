//! Promemoria allo staff per abbonamenti e iscrizioni in scadenza.

use chrono::NaiveDate;
use rusqlite::{Connection, params};
use serde::Serialize;

use super::ErroreDatabase;
use super::iscritti::Riferimento;
use crate::domain::stato::{CondizioneScadenza, StatoValidita, condizione_scadenza_per_stato};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TipoScadenza {
    Abbonamento,
    Iscrizione,
    Certificato,
}

impl TipoScadenza {
    pub const TUTTI: [TipoScadenza; 3] = [Self::Abbonamento, Self::Iscrizione, Self::Certificato];

    fn come_testo(self) -> &'static str {
        match self {
            Self::Abbonamento => "abbonamento",
            Self::Iscrizione => "iscrizione",
            Self::Certificato => "certificato",
        }
    }

    /// Vista e colonna con l'ultima scadenza di ogni iscritto.
    fn vista_e_colonna(self) -> (&'static str, &'static str) {
        match self {
            Self::Abbonamento => ("v_ultima_scadenza_iscritti", "ultima_data_scadenza"),
            Self::Iscrizione => (
                "v_ultima_iscrizione_iscritti",
                "ultima_data_fine_iscrizione",
            ),
            Self::Certificato => ("v_certificato_iscritti", "data_scadenza_certificato"),
        }
    }
}

/// Quanti iscritti (non archiviati) hanno abbonamento o iscrizione in scadenza.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConteggioInScadenza {
    pub abbonamenti: u32,
    pub iscrizioni: u32,
    pub certificati: u32,
}

/// Esito del controllo periodico: le scadenze notificate ora per la prima volta e il totale.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EsitoPromemoria {
    pub nuovi: ConteggioInScadenza,
    pub in_scadenza: ConteggioInScadenza,
}

pub fn conta_in_scadenza(
    connessione: &Connection,
    riferimento: Riferimento,
) -> Result<ConteggioInScadenza, ErroreDatabase> {
    Ok(ConteggioInScadenza {
        abbonamenti: scadenze_in_arrivo(connessione, TipoScadenza::Abbonamento, riferimento)?.len()
            as u32,
        iscrizioni: scadenze_in_arrivo(connessione, TipoScadenza::Iscrizione, riferimento)?.len()
            as u32,
        certificati: scadenze_in_arrivo(connessione, TipoScadenza::Certificato, riferimento)?.len()
            as u32,
    })
}

/// Trova le scadenze entrate nel periodo di preavviso non ancora notificate,
/// le segna come notificate e restituisce quante sono, insieme al totale in scadenza.
pub fn controlla_promemoria(
    connessione: &mut Connection,
    riferimento: Riferimento,
) -> Result<EsitoPromemoria, ErroreDatabase> {
    let transazione = connessione.transaction()?;
    let mut esito = EsitoPromemoria::default();

    for tipo in TipoScadenza::TUTTI {
        let in_arrivo = scadenze_in_arrivo(&transazione, tipo, riferimento)?;
        let mut nuovi = 0;
        for (iscritto_id, data_scadenza) in &in_arrivo {
            nuovi += transazione.execute(
                "INSERT OR IGNORE INTO promemoria_inviati (tipo, iscritto_id, data_scadenza)
                 VALUES (?1, ?2, ?3)",
                params![tipo.come_testo(), iscritto_id, data_scadenza],
            )? as u32;
        }
        match tipo {
            TipoScadenza::Abbonamento => {
                esito.nuovi.abbonamenti = nuovi;
                esito.in_scadenza.abbonamenti = in_arrivo.len() as u32;
            }
            TipoScadenza::Iscrizione => {
                esito.nuovi.iscrizioni = nuovi;
                esito.in_scadenza.iscrizioni = in_arrivo.len() as u32;
            }
            TipoScadenza::Certificato => {
                esito.nuovi.certificati = nuovi;
                esito.in_scadenza.certificati = in_arrivo.len() as u32;
            }
        }
    }

    transazione.commit()?;
    Ok(esito)
}

/// Iscritti non archiviati la cui ultima scadenza cade nel periodo di preavviso.
fn scadenze_in_arrivo(
    connessione: &Connection,
    tipo: TipoScadenza,
    riferimento: Riferimento,
) -> Result<Vec<(i64, NaiveDate)>, ErroreDatabase> {
    let CondizioneScadenza::Tra {
        dal: Some(dal),
        al: Some(al),
    } = condizione_scadenza_per_stato(
        StatoValidita::InScadenza,
        riferimento.oggi,
        riferimento.giorni_preavviso,
    )
    else {
        unreachable!("lo stato in scadenza ha sempre un intervallo chiuso");
    };
    let (vista, colonna) = tipo.vista_e_colonna();
    let mut query = connessione.prepare(&format!(
        "SELECT v.iscritto_id, v.{colonna}
         FROM {vista} v JOIN iscritti i ON i.id = v.iscritto_id
         WHERE i.archiviato = 0 AND v.{colonna} BETWEEN ?1 AND ?2"
    ))?;
    let scadenze = query
        .query_map(params![dal, al], |riga| Ok((riga.get(0)?, riga.get(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(scadenze)
}
