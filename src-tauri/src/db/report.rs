//! Riepilogo economico: incassi di abbonamenti e iscrizioni per periodo e per metodo.

use chrono::NaiveDate;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

use super::ErroreDatabase;
use super::dashboard::Incassi;
use crate::domain::csv::{componi_csv, importo_per_csv};
use crate::domain::pagamento::MetodoPagamento;

/// Tutti gli incassi in un'unica sequenza: pagamenti degli abbonamenti e quote di iscrizione.
/// Le voci pregresse sono escluse: il denaro è stato incassato prima del programma.
const MOVIMENTI: &str = "
    SELECT p.data_pagamento AS data, 'abbonamento' AS tipo, i.cognome, i.nome,
           p.tipologia_nome AS descrizione, p.metodo, p.importo_cent
    FROM pagamenti p JOIN iscritti i ON i.id = p.iscritto_id
    WHERE p.data_pagamento BETWEEN ?1 AND ?2 AND p.pregresso = 0
    UNION ALL
    SELECT s.data_pagamento, 'iscrizione', i.cognome, i.nome,
           'Iscrizione annuale', s.metodo, s.importo_cent
    FROM iscrizioni s JOIN iscritti i ON i.id = s.iscritto_id
    WHERE s.data_pagamento BETWEEN ?1 AND ?2 AND s.pregresso = 0";

const SOMME_PER_TIPO: &str = "
    coalesce(sum(CASE WHEN tipo = 'abbonamento' THEN importo_cent END), 0),
    coalesce(sum(CASE WHEN tipo = 'iscrizione' THEN importo_cent END), 0)";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Raggruppamento {
    Giorno,
    Mese,
    Anno,
}

impl Raggruppamento {
    /// Caratteri iniziali della data ISO che identificano il periodo (`2026-11-10` → `2026-11`).
    fn lunghezza_chiave(self) -> u32 {
        match self {
            Self::Giorno => 10,
            Self::Mese => 7,
            Self::Anno => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncassiPeriodo {
    /// `AAAA`, `AAAA-MM` o `AAAA-MM-GG` secondo il raggruppamento.
    pub periodo: String,
    pub incassi: Incassi,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncassiMetodo {
    pub metodo: MetodoPagamento,
    pub incassi: Incassi,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiepilogoIncassi {
    pub totale: Incassi,
    pub numero_movimenti: u32,
    pub per_periodo: Vec<IncassiPeriodo>,
    pub per_metodo: Vec<IncassiMetodo>,
}

pub fn riepilogo_incassi(
    connessione: &Connection,
    dal: NaiveDate,
    al: NaiveDate,
    raggruppamento: Raggruppamento,
) -> Result<RiepilogoIncassi, ErroreDatabase> {
    valida_periodo(dal, al)?;
    let chiave_periodo = format!("substr(data, 1, {})", raggruppamento.lunghezza_chiave());
    let mut per_periodo = Vec::new();
    for (periodo, incassi) in raggruppa(connessione, dal, al, &chiave_periodo)? {
        per_periodo.push(IncassiPeriodo { periodo, incassi });
    }
    let mut per_metodo = Vec::new();
    for (testo_metodo, incassi) in raggruppa(connessione, dal, al, "metodo")? {
        // Il vincolo della tabella ammette solo metodi noti: un testo diverso non può esserci
        if let Some(metodo) = MetodoPagamento::da_testo(&testo_metodo) {
            per_metodo.push(IncassiMetodo { metodo, incassi });
        }
    }
    let (numero_movimenti, abbonamenti_cent, iscrizioni_cent) = connessione.query_row(
        &format!("SELECT count(*), {SOMME_PER_TIPO} FROM ({MOVIMENTI})"),
        params![dal, al],
        |riga| Ok((riga.get(0)?, riga.get(1)?, riga.get(2)?)),
    )?;
    Ok(RiepilogoIncassi {
        totale: componi_incassi(abbonamenti_cent, iscrizioni_cent),
        numero_movimenti,
        per_periodo,
        per_metodo,
    })
}

/// Somme per tipo raggruppate secondo l'espressione SQL, in ordine di chiave.
fn raggruppa(
    connessione: &Connection,
    dal: NaiveDate,
    al: NaiveDate,
    chiave: &str,
) -> Result<Vec<(String, Incassi)>, ErroreDatabase> {
    let mut query = connessione.prepare(&format!(
        "SELECT {chiave} AS chiave, {SOMME_PER_TIPO} FROM ({MOVIMENTI})
         GROUP BY chiave ORDER BY chiave"
    ))?;
    let righe = query
        .query_map(params![dal, al], |riga| {
            Ok((riga.get(0)?, componi_incassi(riga.get(1)?, riga.get(2)?)))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(righe)
}

fn componi_incassi(abbonamenti_cent: i64, iscrizioni_cent: i64) -> Incassi {
    Incassi {
        abbonamenti_cent,
        iscrizioni_cent,
        totale_cent: abbonamenti_cent + iscrizioni_cent,
    }
}

fn valida_periodo(dal: NaiveDate, al: NaiveDate) -> Result<(), ErroreDatabase> {
    if dal > al {
        return Err(ErroreDatabase::non_valido(
            "La data di inizio è successiva alla data di fine",
        ));
    }
    Ok(())
}

const INTESTAZIONE_CSV: [&str; 7] = [
    "Data",
    "Tipo",
    "Cognome",
    "Nome",
    "Descrizione",
    "Metodo",
    "Importo (€)",
];

/// Tutti i movimenti del periodo in formato CSV, in ordine di data.
pub fn esporta_incassi_csv(
    connessione: &Connection,
    dal: NaiveDate,
    al: NaiveDate,
) -> Result<String, ErroreDatabase> {
    valida_periodo(dal, al)?;
    let mut query = connessione.prepare(&format!(
        "SELECT * FROM ({MOVIMENTI}) ORDER BY data, cognome COLLATE NOCASE, nome COLLATE NOCASE"
    ))?;
    let righe = query
        .query_map(params![dal, al], |riga| {
            let data: NaiveDate = riga.get(0)?;
            let tipo: String = riga.get(1)?;
            Ok(vec![
                data.format("%d/%m/%Y").to_string(),
                if tipo == "abbonamento" {
                    "Abbonamento"
                } else {
                    "Iscrizione"
                }
                .to_string(),
                riga.get(2)?,
                riga.get(3)?,
                riga.get(4)?,
                riga.get(5)?,
                importo_per_csv(riga.get(6)?),
            ])
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(componi_csv(&INTESTAZIONE_CSV, &righe))
}
