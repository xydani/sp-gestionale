//! Dati della pagina Statistiche: legge periodi e incassi, i calcoli sono nel dominio.

use chrono::{Months, NaiveDate};
use rusqlite::{Connection, OptionalExtension, Row, params};
use serde::Serialize;

use super::ErroreDatabase;
use super::impostazioni::{
    CHIAVE_GIORNI_TOLLERANZA_RIENTRO, CHIAVE_QUOTA_ISCRIZIONE_CENT,
    QUOTA_ISCRIZIONE_PREDEFINITA_CENT, leggi_impostazione_numerica,
};
use super::tipologie::unita_da_testo;
use crate::domain::date::Durata;
use crate::domain::statistiche::{
    AbbonamentoInCorso, AttiviMese, ConteggioRinnovi, GIORNI_TOLLERANZA_RIENTRO_PREDEFINITI,
    Incasso, PeriodoIscritto, PrevisioneFinestra, QuotaInCorso, RiepilogoRinnovi, RinnoviMese,
    TassiRinnovo, ValoreMedio, attivi_per_mese, calcola_valore_medio, prevedi_incassi,
    riepiloga_rinnovi,
};

/// I tassi della previsione si basano sull'ultimo anno di scadenze.
const MESI_STORICO_PREVISIONE: u32 = 12;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rinnovi {
    pub totale: ConteggioRinnovi,
    pub tasso_rinnovo: Option<f64>,
    pub per_mese: Vec<RinnoviMese>,
}

impl From<&RiepilogoRinnovi> for Rinnovi {
    fn from(riepilogo: &RiepilogoRinnovi) -> Self {
        Self {
            totale: riepilogo.totale,
            tasso_rinnovo: riepilogo.totale.tasso_rinnovo(),
            per_mese: riepilogo.per_mese.clone(),
        }
    }
}

/// Chi non ha rinnovato l'abbonamento: da ricontattare.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IscrittoPerso {
    pub iscritto_id: i64,
    pub nome: String,
    pub cognome: String,
    pub telefono: Option<String>,
    pub tipologia_nome: String,
    pub data_scadenza: NaiveDate,
    pub archiviato: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VenditeTipologiaMese {
    pub mese: String,
    pub tipologia_nome: String,
    pub numero: u32,
    pub importo_cent: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TotaleTipologia {
    pub tipologia_nome: String,
    pub numero: u32,
    pub importo_cent: i64,
    /// Lo stesso periodo dell'anno prima, per il confronto.
    pub numero_anno_precedente: u32,
    pub importo_anno_precedente_cent: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VenditeTipologie {
    pub per_mese: Vec<VenditeTipologiaMese>,
    /// Le più vendute prima.
    pub totali: Vec<TotaleTipologia>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistiche {
    pub giorni_tolleranza_rientro: u32,
    pub rinnovi_abbonamenti: Rinnovi,
    pub rinnovi_iscrizioni: Rinnovi,
    pub persi: Vec<IscrittoPerso>,
    pub attivi_per_mese: Vec<AttiviMese>,
    pub valore_medio: ValoreMedio,
    /// Sempre da oggi in avanti, qualunque sia il periodo scelto.
    pub previsione: Vec<PrevisioneFinestra>,
    pub vendite_per_tipologia: VenditeTipologie,
}

pub fn leggi_statistiche(
    connessione: &Connection,
    dal: NaiveDate,
    al: NaiveDate,
    oggi: NaiveDate,
) -> Result<Statistiche, ErroreDatabase> {
    if dal > al {
        return Err(ErroreDatabase::non_valido(
            "La data di inizio è successiva alla data di fine",
        ));
    }
    let tolleranza = leggi_impostazione_numerica(
        connessione,
        CHIAVE_GIORNI_TOLLERANZA_RIENTRO,
        GIORNI_TOLLERANZA_RIENTRO_PREDEFINITI as usize,
    )? as u32;
    let abbonamenti = leggi_periodi(connessione, PERIODI_ABBONAMENTI)?;
    let iscrizioni = leggi_periodi(connessione, PERIODI_ISCRIZIONI)?;

    let rinnovi_abbonamenti = riepiloga_rinnovi(&abbonamenti, dal, al, oggi, tolleranza);
    let rinnovi_iscrizioni = riepiloga_rinnovi(&iscrizioni, dal, al, oggi, tolleranza);
    let previsione = prevedi(connessione, &abbonamenti, &iscrizioni, oggi, tolleranza)?;

    Ok(Statistiche {
        giorni_tolleranza_rientro: tolleranza,
        rinnovi_abbonamenti: Rinnovi::from(&rinnovi_abbonamenti),
        rinnovi_iscrizioni: Rinnovi::from(&rinnovi_iscrizioni),
        persi: descrivi_persi(connessione, &rinnovi_abbonamenti.scadenze_perse)?,
        attivi_per_mese: attivi_per_mese(&abbonamenti, dal, al, oggi),
        valore_medio: calcola_valore_medio(&leggi_incassi(connessione)?, &abbonamenti, dal, al),
        previsione,
        vendite_per_tipologia: vendite_per_tipologia(connessione, dal, al)?,
    })
}

/// Le voci pregresse contano: sono abbonamenti veri, anche se pagati prima del programma.
const PERIODI_ABBONAMENTI: &str = "SELECT iscritto_id, data_inizio, data_scadenza FROM pagamenti";
const PERIODI_ISCRIZIONI: &str = "SELECT iscritto_id, data_inizio, data_fine FROM iscrizioni";

fn leggi_periodi(
    connessione: &Connection,
    sql: &str,
) -> Result<Vec<PeriodoIscritto>, ErroreDatabase> {
    let mut query = connessione.prepare(sql)?;
    let periodi = query
        .query_map([], |riga| {
            Ok(PeriodoIscritto {
                iscritto_id: riga.get(0)?,
                inizio: riga.get(1)?,
                fine: riga.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(periodi)
}

/// Solo incassi veri: le voci pregresse non sono denaro entrato con il programma.
fn leggi_incassi(connessione: &Connection) -> Result<Vec<Incasso>, ErroreDatabase> {
    let mut query = connessione.prepare(
        "SELECT iscritto_id, data_pagamento, importo_cent FROM pagamenti WHERE pregresso = 0
         UNION ALL
         SELECT iscritto_id, data_pagamento, importo_cent FROM iscrizioni WHERE pregresso = 0",
    )?;
    let incassi = query
        .query_map([], |riga| {
            Ok(Incasso {
                iscritto_id: riga.get(0)?,
                data: riga.get(1)?,
                importo_cent: riga.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(incassi)
}

fn descrivi_persi(
    connessione: &Connection,
    scadenze_perse: &[PeriodoIscritto],
) -> Result<Vec<IscrittoPerso>, ErroreDatabase> {
    let mut query = connessione.prepare(
        "SELECT i.id, i.nome, i.cognome, i.telefono, p.tipologia_nome, p.data_scadenza,
                i.archiviato
         FROM pagamenti p JOIN iscritti i ON i.id = p.iscritto_id
         WHERE p.iscritto_id = ?1 AND p.data_scadenza = ?2 AND i.data_eliminazione IS NULL
         LIMIT 1",
    )?;
    let mut persi = Vec::new();
    for scadenza in scadenze_perse {
        let perso = query
            .query_row(params![scadenza.iscritto_id, scadenza.fine], leggi_perso)
            .optional()?;
        persi.extend(perso);
    }
    Ok(persi)
}

fn leggi_perso(riga: &Row<'_>) -> rusqlite::Result<IscrittoPerso> {
    Ok(IscrittoPerso {
        iscritto_id: riga.get(0)?,
        nome: riga.get(1)?,
        cognome: riga.get(2)?,
        telefono: riga.get(3)?,
        tipologia_nome: riga.get(4)?,
        data_scadenza: riga.get(5)?,
        archiviato: riga.get(6)?,
    })
}

fn prevedi(
    connessione: &Connection,
    abbonamenti: &[PeriodoIscritto],
    iscrizioni: &[PeriodoIscritto],
    oggi: NaiveDate,
    tolleranza: u32,
) -> Result<Vec<PrevisioneFinestra>, ErroreDatabase> {
    let inizio_storico = oggi - Months::new(MESI_STORICO_PREVISIONE);
    let storico_abbonamenti =
        riepiloga_rinnovi(abbonamenti, inizio_storico, oggi, oggi, tolleranza);
    let storico_quote = riepiloga_rinnovi(iscrizioni, inizio_storico, oggi, oggi, tolleranza);
    let tassi = componi_tassi(
        storico_abbonamenti.totale.tasso_rinnovo(),
        storico_quote.totale.tasso_rinnovo(),
    );
    let tassi_peggiori = componi_tassi(
        storico_abbonamenti.tasso_peggiore(),
        storico_quote.tasso_peggiore(),
    );
    Ok(prevedi_incassi(
        &abbonamenti_in_corso(connessione)?,
        &quote_in_corso(connessione)?,
        oggi,
        tolleranza,
        tassi,
        tassi_peggiori,
    ))
}

/// Senza storico delle quote si usa il tasso degli abbonamenti, e viceversa.
fn componi_tassi(abbonamenti: Option<f64>, quote: Option<f64>) -> Option<TassiRinnovo> {
    let abbonamenti_o_quote = abbonamenti.or(quote)?;
    Some(TassiRinnovo {
        abbonamenti: abbonamenti_o_quote,
        quote: quote.unwrap_or(abbonamenti_o_quote),
    })
}

/// L'ultimo abbonamento di ogni iscritto non archiviato, con il prezzo del rinnovo:
/// quello del listino, o l'ultimo pagato se la tipologia non è più in vendita.
fn abbonamenti_in_corso(
    connessione: &Connection,
) -> Result<Vec<AbbonamentoInCorso>, ErroreDatabase> {
    let mut query = connessione.prepare(
        "SELECT p.data_scadenza, t.durata_valore, t.durata_unita,
                CASE WHEN t.attiva = 1 OR p.pregresso = 1 THEN t.prezzo_cent
                     ELSE p.importo_cent END
         FROM iscritti i
         JOIN pagamenti p ON p.id = (
             SELECT id FROM pagamenti WHERE iscritto_id = i.id
             ORDER BY data_scadenza DESC, id DESC LIMIT 1)
         JOIN tipologie_abbonamento t ON t.id = p.tipologia_id
         WHERE i.archiviato = 0 AND i.data_eliminazione IS NULL",
    )?;
    let abbonamenti = query
        .query_map([], leggi_abbonamento_in_corso)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(abbonamenti)
}

fn leggi_abbonamento_in_corso(riga: &Row<'_>) -> rusqlite::Result<AbbonamentoInCorso> {
    let testo_unita: String = riga.get(2)?;
    let unita = unita_da_testo(&testo_unita).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            2,
            rusqlite::types::Type::Text,
            format!("unità di durata sconosciuta: {testo_unita}").into(),
        )
    })?;
    Ok(AbbonamentoInCorso {
        data_scadenza: riga.get(0)?,
        durata: Durata {
            valore: riga.get(1)?,
            unita,
        },
        prezzo_cent: riga.get(3)?,
    })
}

fn quote_in_corso(connessione: &Connection) -> Result<Vec<QuotaInCorso>, ErroreDatabase> {
    let quota_cent = leggi_impostazione_numerica(
        connessione,
        CHIAVE_QUOTA_ISCRIZIONE_CENT,
        QUOTA_ISCRIZIONE_PREDEFINITA_CENT,
    )? as i64;
    let mut query = connessione.prepare(
        "SELECT s.ultima_data_fine_iscrizione
         FROM v_ultima_iscrizione_iscritti s JOIN iscritti i ON i.id = s.iscritto_id
         WHERE s.ultima_data_fine_iscrizione IS NOT NULL
           AND i.archiviato = 0 AND i.data_eliminazione IS NULL",
    )?;
    let quote = query
        .query_map([], |riga| {
            Ok(QuotaInCorso {
                data_fine: riga.get(0)?,
                quota_cent,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(quote)
}

fn vendite_per_tipologia(
    connessione: &Connection,
    dal: NaiveDate,
    al: NaiveDate,
) -> Result<VenditeTipologie, ErroreDatabase> {
    let mut query = connessione.prepare(
        "SELECT substr(data_pagamento, 1, 7) AS mese, tipologia_nome, count(*), sum(importo_cent)
         FROM pagamenti
         WHERE pregresso = 0 AND data_pagamento BETWEEN ?1 AND ?2
         GROUP BY mese, tipologia_nome
         ORDER BY mese, tipologia_nome COLLATE NOCASE",
    )?;
    let per_mese = query
        .query_map(params![dal, al], |riga| {
            Ok(VenditeTipologiaMese {
                mese: riga.get(0)?,
                tipologia_nome: riga.get(1)?,
                numero: riga.get(2)?,
                importo_cent: riga.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(VenditeTipologie {
        totali: totali_per_tipologia(connessione, dal, al)?,
        per_mese,
    })
}

/// Totali del periodo per tipologia, con lo stesso periodo dell'anno precedente.
fn totali_per_tipologia(
    connessione: &Connection,
    dal: NaiveDate,
    al: NaiveDate,
) -> Result<Vec<TotaleTipologia>, ErroreDatabase> {
    let anno = Months::new(12);
    let mut query = connessione.prepare(
        "SELECT tipologia_nome,
                count(CASE WHEN data_pagamento BETWEEN ?1 AND ?2 THEN 1 END),
                coalesce(sum(CASE WHEN data_pagamento BETWEEN ?1 AND ?2 THEN importo_cent END), 0),
                count(CASE WHEN data_pagamento BETWEEN ?3 AND ?4 THEN 1 END),
                coalesce(sum(CASE WHEN data_pagamento BETWEEN ?3 AND ?4 THEN importo_cent END), 0)
         FROM pagamenti
         WHERE pregresso = 0 AND (data_pagamento BETWEEN ?1 AND ?2 OR data_pagamento BETWEEN ?3 AND ?4)
         GROUP BY tipologia_nome
         ORDER BY 2 DESC, tipologia_nome COLLATE NOCASE",
    )?;
    let totali = query
        .query_map(params![dal, al, dal - anno, al - anno], |riga| {
            Ok(TotaleTipologia {
                tipologia_nome: riga.get(0)?,
                numero: riga.get(1)?,
                importo_cent: riga.get(2)?,
                numero_anno_precedente: riga.get(3)?,
                importo_anno_precedente_cent: riga.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(totali)
}
