//! Riepilogo per la dashboard: conteggi per stato, incassi del mese e prossime scadenze.

use chrono::{Datelike, Days, Months, NaiveDate};
use rusqlite::{Connection, params};
use serde::Serialize;

use super::ErroreDatabase;
use super::iscritti::Riferimento;
use crate::domain::stato::{CondizioneScadenza, StatoValidita, condizione_scadenza_per_stato};

/// Quanti giorni avanti guardare per l'elenco delle prossime scadenze.
pub const GIORNI_PROSSIME_SCADENZE: u64 = 30;
/// Massimo numero di scadenze mostrate nell'elenco della dashboard.
pub const PROSSIME_SCADENZE_MASSIME: u32 = 10;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConteggioStati {
    pub attivi: u32,
    pub in_scadenza: u32,
    pub scaduti: u32,
    pub assenti: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Incassi {
    pub abbonamenti_cent: i64,
    pub iscrizioni_cent: i64,
    pub totale_cent: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProssimaScadenza {
    /// "abbonamento", "iscrizione" oppure "certificato"
    pub tipo: String,
    pub iscritto_id: i64,
    pub nome: String,
    pub cognome: String,
    pub data_scadenza: NaiveDate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiepilogoDashboard {
    /// Iscritti non archiviati.
    pub iscritti: u32,
    pub abbonamenti: ConteggioStati,
    pub iscrizioni: ConteggioStati,
    /// "assenti" sono gli iscritti che non hanno presentato il certificato medico.
    pub certificati: ConteggioStati,
    pub incassi_mese: Incassi,
    pub incassi_mese_precedente: Incassi,
    pub prossime_scadenze: Vec<ProssimaScadenza>,
}

pub fn riepilogo_dashboard(
    connessione: &Connection,
    riferimento: Riferimento,
) -> Result<RiepilogoDashboard, ErroreDatabase> {
    let iscritti = connessione.query_row(
        "SELECT count(*) FROM iscritti WHERE archiviato = 0",
        [],
        |riga| riga.get(0),
    )?;
    let (inizio_mese, fine_mese) = mese_di(riferimento.oggi);
    let inizio_mese_precedente = inizio_mese - Months::new(1);
    let fine_mese_precedente = inizio_mese
        .pred_opt()
        .expect("data nell'intervallo supportato");

    Ok(RiepilogoDashboard {
        iscritti,
        abbonamenti: conta_per_stato(
            connessione,
            "v_ultima_scadenza_iscritti",
            "ultima_data_scadenza",
            riferimento,
        )?,
        iscrizioni: conta_per_stato(
            connessione,
            "v_ultima_iscrizione_iscritti",
            "ultima_data_fine_iscrizione",
            riferimento,
        )?,
        certificati: conta_per_stato(
            connessione,
            "v_certificato_iscritti",
            "data_scadenza_certificato",
            riferimento,
        )?,
        incassi_mese: incassi_tra(connessione, inizio_mese, fine_mese)?,
        incassi_mese_precedente: incassi_tra(
            connessione,
            inizio_mese_precedente,
            fine_mese_precedente,
        )?,
        prossime_scadenze: prossime_scadenze(connessione, riferimento.oggi)?,
    })
}

/// Primo e ultimo giorno del mese della data.
pub fn mese_di(data: NaiveDate) -> (NaiveDate, NaiveDate) {
    let primo = data.with_day(1).expect("il primo del mese esiste sempre");
    let ultimo = (primo + Months::new(1))
        .pred_opt()
        .expect("data nell'intervallo supportato");
    (primo, ultimo)
}

/// Conta gli iscritti non archiviati per stato, con la stessa regola di `calcola_stato_validita`.
fn conta_per_stato(
    connessione: &Connection,
    vista: &str,
    colonna: &str,
    riferimento: Riferimento,
) -> Result<ConteggioStati, ErroreDatabase> {
    let conta = |stato| -> Result<u32, ErroreDatabase> {
        let condizione =
            condizione_scadenza_per_stato(stato, riferimento.oggi, riferimento.giorni_preavviso);
        let (filtro, dal, al) = match condizione {
            CondizioneScadenza::Assente => (format!("v.{colonna} IS NULL"), None, None),
            CondizioneScadenza::Tra { dal, al } => (
                format!(
                    "v.{colonna} IS NOT NULL
                     AND (?1 IS NULL OR v.{colonna} >= ?1)
                     AND (?2 IS NULL OR v.{colonna} <= ?2)"
                ),
                dal,
                al,
            ),
        };
        let sql = format!(
            "SELECT count(*) FROM {vista} v JOIN iscritti i ON i.id = v.iscritto_id
             WHERE i.archiviato = 0 AND {filtro}"
        );
        let parametri = if filtro.contains("?1") {
            params![dal, al]
        } else {
            params![]
        };
        Ok(connessione.query_row(&sql, parametri, |riga| riga.get(0))?)
    };
    Ok(ConteggioStati {
        attivi: conta(StatoValidita::Attivo)?,
        in_scadenza: conta(StatoValidita::InScadenza)?,
        scaduti: conta(StatoValidita::Scaduto)?,
        assenti: conta(StatoValidita::Nessuno)?,
    })
}

/// Incassi con data di pagamento tra le due date, estremi inclusi.
pub fn incassi_tra(
    connessione: &Connection,
    dal: NaiveDate,
    al: NaiveDate,
) -> Result<Incassi, ErroreDatabase> {
    let somma = |tabella: &str| -> Result<i64, ErroreDatabase> {
        Ok(connessione.query_row(
            &format!(
                "SELECT coalesce(sum(importo_cent), 0) FROM {tabella}
                 WHERE data_pagamento BETWEEN ?1 AND ?2"
            ),
            params![dal, al],
            |riga| riga.get(0),
        )?)
    };
    let abbonamenti_cent = somma("pagamenti")?;
    let iscrizioni_cent = somma("iscrizioni")?;
    Ok(Incassi {
        abbonamenti_cent,
        iscrizioni_cent,
        totale_cent: abbonamenti_cent + iscrizioni_cent,
    })
}

/// Abbonamenti e iscrizioni che scadono da oggi ai prossimi giorni, i più vicini per primi.
fn prossime_scadenze(
    connessione: &Connection,
    oggi: NaiveDate,
) -> Result<Vec<ProssimaScadenza>, ErroreDatabase> {
    let limite = oggi + Days::new(GIORNI_PROSSIME_SCADENZE);
    let mut query = connessione.prepare(
        "SELECT 'abbonamento', i.id, i.nome, i.cognome, v.ultima_data_scadenza
         FROM v_ultima_scadenza_iscritti v JOIN iscritti i ON i.id = v.iscritto_id
         WHERE i.archiviato = 0 AND v.ultima_data_scadenza BETWEEN ?1 AND ?2
         UNION ALL
         SELECT 'iscrizione', i.id, i.nome, i.cognome, v.ultima_data_fine_iscrizione
         FROM v_ultima_iscrizione_iscritti v JOIN iscritti i ON i.id = v.iscritto_id
         WHERE i.archiviato = 0 AND v.ultima_data_fine_iscrizione BETWEEN ?1 AND ?2
         UNION ALL
         SELECT 'certificato', i.id, i.nome, i.cognome, i.data_scadenza_certificato
         FROM iscritti i
         WHERE i.archiviato = 0 AND i.data_scadenza_certificato BETWEEN ?1 AND ?2
         ORDER BY 5, 4 COLLATE NOCASE
         LIMIT ?3",
    )?;
    let scadenze = query
        .query_map(params![oggi, limite, PROSSIME_SCADENZE_MASSIME], |riga| {
            Ok(ProssimaScadenza {
                tipo: riga.get(0)?,
                iscritto_id: riga.get(1)?,
                nome: riga.get(2)?,
                cognome: riga.get(3)?,
                data_scadenza: riga.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(scadenze)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn il_mese_va_dal_primo_all_ultimo_giorno() {
        let data = |testo: &str| testo.parse::<NaiveDate>().unwrap();
        assert_eq!(
            mese_di(data("2026-11-10")),
            (data("2026-11-01"), data("2026-11-30"))
        );
        assert_eq!(
            mese_di(data("2028-02-15")),
            (data("2028-02-01"), data("2028-02-29"))
        );
        assert_eq!(
            mese_di(data("2026-12-31")),
            (data("2026-12-01"), data("2026-12-31"))
        );
    }
}
