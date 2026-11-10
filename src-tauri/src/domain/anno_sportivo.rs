use chrono::{Datelike, Months, NaiveDate};

/// Mese di inizio predefinito dell'anno sportivo (settembre).
pub const MESE_INIZIO_ANNO_SPORTIVO_PREDEFINITO: u32 = 9;

/// Periodo di validità di un'iscrizione, estremi inclusi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Periodo {
    pub inizio: NaiveDate,
    pub fine: NaiveDate,
}

/// Anno sportivo che contiene la data: dal primo giorno del mese di inizio
/// all'ultimo giorno del mese precedente dell'anno dopo (es. 01/09/2026 – 31/08/2027).
pub fn anno_sportivo_contenente(data: NaiveDate, mese_inizio: u32) -> Periodo {
    let mese_inizio = mese_inizio.clamp(1, 12);
    let anno_inizio = if data.month() >= mese_inizio {
        data.year()
    } else {
        data.year() - 1
    };
    let inizio = NaiveDate::from_ymd_opt(anno_inizio, mese_inizio, 1)
        .expect("il primo giorno del mese esiste sempre");
    let fine = inizio
        .checked_add_months(Months::new(12))
        .and_then(|inizio_anno_dopo| inizio_anno_dopo.pred_opt())
        .expect("l'anno sportivo è nell'intervallo di date supportato");
    Periodo { inizio, fine }
}

/// Periodo proposto per una nuova iscrizione: se quella attuale è ancora valida,
/// l'anno sportivo successivo; altrimenti l'anno sportivo in corso.
pub fn proponi_periodo_iscrizione(
    ultima_data_fine: Option<NaiveDate>,
    oggi: NaiveDate,
    mese_inizio: u32,
) -> Periodo {
    if let Some(fine) = ultima_data_fine
        && fine >= oggi
    {
        let giorno_dopo_la_fine = fine.succ_opt().expect("data nell'intervallo supportato");
        return anno_sportivo_contenente(giorno_dopo_la_fine, mese_inizio);
    }
    anno_sportivo_contenente(oggi, mese_inizio)
}

#[cfg(test)]
mod test {
    use super::*;

    fn data(anno: i32, mese: u32, giorno: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(anno, mese, giorno).unwrap()
    }

    #[test]
    fn da_settembre_l_anno_sportivo_e_quello_che_inizia() {
        let periodo = anno_sportivo_contenente(data(2026, 11, 10), 9);
        assert_eq!(
            periodo,
            Periodo {
                inizio: data(2026, 9, 1),
                fine: data(2027, 8, 31)
            }
        );
    }

    #[test]
    fn prima_di_settembre_e_quello_iniziato_l_anno_prima() {
        let periodo = anno_sportivo_contenente(data(2026, 8, 31), 9);
        assert_eq!(
            periodo,
            Periodo {
                inizio: data(2025, 9, 1),
                fine: data(2026, 8, 31)
            }
        );
    }

    #[test]
    fn il_primo_giorno_dell_anno_sportivo_appartiene_al_nuovo() {
        assert_eq!(
            anno_sportivo_contenente(data(2026, 9, 1), 9).inizio,
            data(2026, 9, 1)
        );
    }

    #[test]
    fn con_inizio_a_gennaio_coincide_con_l_anno_solare() {
        let periodo = anno_sportivo_contenente(data(2028, 2, 29), 1);
        assert_eq!(
            periodo,
            Periodo {
                inizio: data(2028, 1, 1),
                fine: data(2028, 12, 31)
            }
        );
    }

    #[test]
    fn un_mese_non_valido_viene_corretto() {
        assert_eq!(
            anno_sportivo_contenente(data(2026, 5, 1), 0).inizio,
            data(2026, 1, 1)
        );
        assert_eq!(
            anno_sportivo_contenente(data(2026, 5, 1), 13).inizio,
            data(2025, 12, 1)
        );
    }

    #[test]
    fn prima_iscrizione_nell_anno_in_corso() {
        let periodo = proponi_periodo_iscrizione(None, data(2026, 11, 10), 9);
        assert_eq!(periodo.inizio, data(2026, 9, 1));
    }

    #[test]
    fn rinnovo_con_iscrizione_valida_propone_l_anno_successivo() {
        let periodo = proponi_periodo_iscrizione(Some(data(2026, 8, 31)), data(2026, 7, 20), 9);
        assert_eq!(
            periodo,
            Periodo {
                inizio: data(2026, 9, 1),
                fine: data(2027, 8, 31)
            }
        );
    }

    #[test]
    fn rinnovo_con_iscrizione_scaduta_propone_l_anno_in_corso() {
        let periodo = proponi_periodo_iscrizione(Some(data(2025, 8, 31)), data(2026, 11, 10), 9);
        assert_eq!(periodo.inizio, data(2026, 9, 1));
    }

    #[test]
    fn rinnovo_l_ultimo_giorno_di_validita_propone_l_anno_successivo() {
        let periodo = proponi_periodo_iscrizione(Some(data(2026, 8, 31)), data(2026, 8, 31), 9);
        assert_eq!(periodo.inizio, data(2026, 9, 1));
    }
}
