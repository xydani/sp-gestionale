use chrono::{Days, NaiveDate};
use serde::{Deserialize, Serialize};

/// Giorni di preavviso predefiniti per segnalare un abbonamento in scadenza.
pub const GIORNI_PREAVVISO_PREDEFINITI: u32 = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatoValidita {
    Attivo,
    InScadenza,
    Scaduto,
    Nessuno,
}

/// Stato di validità (abbonamento o iscrizione) in base alla sua ultima scadenza.
///
/// Il giorno di scadenza è ancora valido: diventa scaduto dal giorno dopo.
pub fn calcola_stato_validita(
    ultima_data_scadenza: Option<NaiveDate>,
    oggi: NaiveDate,
    giorni_preavviso: u32,
) -> StatoValidita {
    let Some(data_scadenza) = ultima_data_scadenza else {
        return StatoValidita::Nessuno;
    };

    let giorni_rimanenti = giorni_alla_scadenza(data_scadenza, oggi);
    if giorni_rimanenti < 0 {
        StatoValidita::Scaduto
    } else if giorni_rimanenti <= i64::from(giorni_preavviso) {
        StatoValidita::InScadenza
    } else {
        StatoValidita::Attivo
    }
}

/// Intervallo di date di scadenza che corrisponde a uno stato: permette di filtrare
/// per stato direttamente nel database, con la stessa regola di `calcola_stato_validita`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CondizioneScadenza {
    /// Nessun pagamento: scadenza assente.
    Assente,
    /// Scadenza compresa tra le due date (estremi inclusi, `None` = illimitato).
    Tra {
        dal: Option<NaiveDate>,
        al: Option<NaiveDate>,
    },
}

impl CondizioneScadenza {
    pub fn comprende(self, ultima_data_scadenza: Option<NaiveDate>) -> bool {
        match (self, ultima_data_scadenza) {
            (Self::Assente, scadenza) => scadenza.is_none(),
            (Self::Tra { .. }, None) => false,
            (Self::Tra { dal, al }, Some(scadenza)) => {
                dal.is_none_or(|dal| scadenza >= dal) && al.is_none_or(|al| scadenza <= al)
            }
        }
    }
}

pub fn condizione_scadenza_per_stato(
    stato: StatoValidita,
    oggi: NaiveDate,
    giorni_preavviso: u32,
) -> CondizioneScadenza {
    let ultimo_giorno_di_preavviso = oggi + Days::new(u64::from(giorni_preavviso));
    match stato {
        StatoValidita::Nessuno => CondizioneScadenza::Assente,
        StatoValidita::Scaduto => CondizioneScadenza::Tra {
            dal: None,
            al: oggi.pred_opt(),
        },
        StatoValidita::InScadenza => CondizioneScadenza::Tra {
            dal: Some(oggi),
            al: Some(ultimo_giorno_di_preavviso),
        },
        StatoValidita::Attivo => CondizioneScadenza::Tra {
            dal: ultimo_giorno_di_preavviso.succ_opt(),
            al: None,
        },
    }
}

/// Giorni da oggi alla scadenza: 0 se scade oggi, negativo se già scaduto.
pub fn giorni_alla_scadenza(data_scadenza: NaiveDate, oggi: NaiveDate) -> i64 {
    (data_scadenza - oggi).num_days()
}

#[cfg(test)]
mod test {
    use super::*;

    const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();

    fn stato_con_scadenza_tra(giorni: i64) -> StatoValidita {
        let scadenza = OGGI + chrono::Duration::days(giorni);
        calcola_stato_validita(Some(scadenza), OGGI, GIORNI_PREAVVISO_PREDEFINITI)
    }

    #[test]
    fn senza_pagamenti_lo_stato_e_nessuno() {
        assert_eq!(
            calcola_stato_validita(None, OGGI, GIORNI_PREAVVISO_PREDEFINITI),
            StatoValidita::Nessuno
        );
    }

    #[test]
    fn scaduto_ieri_e_scaduto() {
        assert_eq!(stato_con_scadenza_tra(-1), StatoValidita::Scaduto);
    }

    #[test]
    fn scade_oggi_e_in_scadenza() {
        assert_eq!(stato_con_scadenza_tra(0), StatoValidita::InScadenza);
    }

    #[test]
    fn scade_all_ultimo_giorno_di_preavviso_e_in_scadenza() {
        assert_eq!(stato_con_scadenza_tra(7), StatoValidita::InScadenza);
    }

    #[test]
    fn scade_dopo_il_preavviso_e_attivo() {
        assert_eq!(stato_con_scadenza_tra(8), StatoValidita::Attivo);
    }

    #[test]
    fn preavviso_zero_segnala_solo_il_giorno_di_scadenza() {
        let domani = OGGI.succ_opt().unwrap();
        assert_eq!(
            calcola_stato_validita(Some(OGGI), OGGI, 0),
            StatoValidita::InScadenza
        );
        assert_eq!(
            calcola_stato_validita(Some(domani), OGGI, 0),
            StatoValidita::Attivo
        );
    }

    #[test]
    fn giorni_alla_scadenza_e_negativo_se_scaduto() {
        let tre_giorni_fa = OGGI - chrono::Duration::days(3);
        assert_eq!(giorni_alla_scadenza(tre_giorni_fa, OGGI), -3);
        assert_eq!(giorni_alla_scadenza(OGGI, OGGI), 0);
    }

    #[test]
    fn la_condizione_di_ogni_stato_coincide_con_il_calcolo_dello_stato() {
        let stati = [
            StatoValidita::Attivo,
            StatoValidita::InScadenza,
            StatoValidita::Scaduto,
            StatoValidita::Nessuno,
        ];
        let scadenze = std::iter::once(None)
            .chain((-40..=40).map(|giorni| Some(OGGI + chrono::Duration::days(giorni))));
        for scadenza in scadenze {
            for giorni_preavviso in [0, 7, 14] {
                let stato_calcolato = calcola_stato_validita(scadenza, OGGI, giorni_preavviso);
                for stato in stati {
                    let condizione = condizione_scadenza_per_stato(stato, OGGI, giorni_preavviso);
                    assert_eq!(
                        condizione.comprende(scadenza),
                        stato == stato_calcolato,
                        "scadenza {scadenza:?}, preavviso {giorni_preavviso}, stato {stato:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn stato_si_serializza_in_snake_case() {
        assert_eq!(
            serde_json::to_string(&StatoValidita::InScadenza).unwrap(),
            "\"in_scadenza\""
        );
    }
}
