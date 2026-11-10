use chrono::NaiveDate;

/// Data di inizio del nuovo abbonamento quando un iscritto rinnova.
///
/// Se l'abbonamento precedente è ancora valido, il nuovo parte dal giorno
/// dopo la scadenza (i giorni residui non si perdono); altrimenti da oggi.
pub fn calcola_data_inizio_rinnovo(
    data_scadenza_precedente: Option<NaiveDate>,
    oggi: NaiveDate,
) -> NaiveDate {
    let Some(data_scadenza) = data_scadenza_precedente else {
        return oggi;
    };
    match data_scadenza.succ_opt() {
        Some(giorno_dopo_scadenza) => giorno_dopo_scadenza.max(oggi),
        None => oggi,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();

    fn data(anno: i32, mese: u32, giorno: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(anno, mese, giorno).unwrap()
    }

    #[test]
    fn primo_abbonamento_inizia_oggi() {
        assert_eq!(calcola_data_inizio_rinnovo(None, OGGI), OGGI);
    }

    #[test]
    fn rinnovo_prima_della_scadenza_parte_dal_giorno_dopo_la_scadenza() {
        assert_eq!(
            calcola_data_inizio_rinnovo(Some(data(2026, 11, 20)), OGGI),
            data(2026, 11, 21)
        );
    }

    #[test]
    fn rinnovo_il_giorno_della_scadenza_parte_da_domani() {
        assert_eq!(
            calcola_data_inizio_rinnovo(Some(OGGI), OGGI),
            data(2026, 11, 11)
        );
    }

    #[test]
    fn rinnovo_il_giorno_dopo_la_scadenza_parte_da_oggi() {
        assert_eq!(
            calcola_data_inizio_rinnovo(Some(data(2026, 11, 9)), OGGI),
            OGGI
        );
    }

    #[test]
    fn rinnovo_dopo_scadenza_lontana_parte_da_oggi() {
        assert_eq!(
            calcola_data_inizio_rinnovo(Some(data(2025, 1, 31)), OGGI),
            OGGI
        );
    }

    #[test]
    fn rinnovo_a_fine_anno_parte_dal_primo_gennaio() {
        let fine_anno = data(2026, 12, 31);
        assert_eq!(
            calcola_data_inizio_rinnovo(Some(fine_anno), OGGI),
            data(2027, 1, 1)
        );
    }
}
