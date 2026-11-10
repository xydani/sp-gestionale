//! Iscrizione di una persona nuova: anagrafica, quota annuale e primo abbonamento insieme.

use rusqlite::Connection;
use serde::Deserialize;

use super::ErroreDatabase;
use super::iscritti::{DatiIscritto, Iscritto, Riferimento, crea_iscritto, trova_iscritto};
use super::iscrizioni::{DatiIscrizione, registra_iscrizione};
use super::pagamenti::{DatiPagamento, registra_pagamento};

/// Tutto ciò che serve per iscrivere una persona nuova.
/// `iscritto_id` di iscrizione e pagamento viene ignorato: si usa quello dell'iscritto appena creato.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatiNuovoIscritto {
    pub anagrafica: DatiIscritto,
    pub iscrizione: DatiIscrizione,
    pub pagamento: DatiPagamento,
}

/// Crea l'iscritto con iscrizione annuale e primo abbonamento: se un dato non è valido
/// non viene salvato nulla, così non restano iscritti senza pagamenti.
pub fn iscrivi_nuovo_iscritto(
    connessione: &mut Connection,
    dati: &DatiNuovoIscritto,
    riferimento: Riferimento,
) -> Result<Iscritto, ErroreDatabase> {
    let transazione = connessione.transaction()?;
    let iscritto_id = crea_iscritto(&transazione, &dati.anagrafica, riferimento)?.id;
    registra_iscrizione(
        &transazione,
        &DatiIscrizione {
            iscritto_id,
            ..dati.iscrizione.clone()
        },
        riferimento.oggi,
    )?;
    registra_pagamento(
        &transazione,
        &DatiPagamento {
            iscritto_id,
            rinnovo_di_id: None,
            ..dati.pagamento.clone()
        },
        riferimento.oggi,
    )?;
    let iscritto = trova_iscritto(&transazione, iscritto_id, riferimento)?;
    transazione.commit()?;
    Ok(iscritto)
}
