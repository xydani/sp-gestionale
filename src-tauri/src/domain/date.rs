use chrono::{Datelike, Days, Months, NaiveDate};
use serde::{Deserialize, Serialize};

use super::ErroreDominio;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UnitaDurata {
    Giorni,
    Mesi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Durata {
    pub valore: u32,
    pub unita: UnitaDurata,
}

/// Ultimo giorno valido di un abbonamento che inizia il giorno `data_inizio`.
///
/// Regola: inizio + durata − 1 giorno (mensile dal 10/01 scade il 09/02).
/// Se il mese di arrivo è più corto del giorno di inizio (31/01 + 1 mese),
/// la scadenza è l'ultimo giorno di quel mese (28/02), così il periodo
/// successivo inizia il primo del mese dopo senza perdere giorni.
pub fn calcola_data_scadenza(
    data_inizio: NaiveDate,
    durata: Durata,
) -> Result<NaiveDate, ErroreDominio> {
    if durata.valore == 0 {
        return Err(ErroreDominio::DurataNulla);
    }

    match durata.unita {
        UnitaDurata::Giorni => data_inizio
            .checked_add_days(Days::new(u64::from(durata.valore) - 1))
            .ok_or(ErroreDominio::DataFuoriIntervallo),
        UnitaDurata::Mesi => {
            let stesso_giorno_dopo_n_mesi = data_inizio
                .checked_add_months(Months::new(durata.valore))
                .ok_or(ErroreDominio::DataFuoriIntervallo)?;

            let giorno_troncato_a_fine_mese = stesso_giorno_dopo_n_mesi.day() < data_inizio.day();
            if giorno_troncato_a_fine_mese {
                Ok(stesso_giorno_dopo_n_mesi)
            } else {
                stesso_giorno_dopo_n_mesi
                    .pred_opt()
                    .ok_or(ErroreDominio::DataFuoriIntervallo)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn data(anno: i32, mese: u32, giorno: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(anno, mese, giorno).unwrap()
    }

    fn mesi(valore: u32) -> Durata {
        Durata {
            valore,
            unita: UnitaDurata::Mesi,
        }
    }

    fn giorni(valore: u32) -> Durata {
        Durata {
            valore,
            unita: UnitaDurata::Giorni,
        }
    }

    #[test]
    fn mensile_scade_il_giorno_prima_dello_stesso_giorno_del_mese_dopo() {
        assert_eq!(
            calcola_data_scadenza(data(2026, 1, 10), mesi(1)),
            Ok(data(2026, 2, 9))
        );
    }

    #[test]
    fn mensile_dal_primo_scade_a_fine_mese() {
        assert_eq!(
            calcola_data_scadenza(data(2026, 1, 1), mesi(1)),
            Ok(data(2026, 1, 31))
        );
    }

    #[test]
    fn mensile_dal_31_gennaio_scade_a_fine_febbraio() {
        assert_eq!(
            calcola_data_scadenza(data(2026, 1, 31), mesi(1)),
            Ok(data(2026, 2, 28))
        );
    }

    #[test]
    fn mensile_dal_31_gennaio_in_anno_bisestile_scade_il_29_febbraio() {
        assert_eq!(
            calcola_data_scadenza(data(2028, 1, 31), mesi(1)),
            Ok(data(2028, 2, 29))
        );
    }

    #[test]
    fn mensile_dal_29_febbraio_bisestile_scade_il_28_marzo() {
        assert_eq!(
            calcola_data_scadenza(data(2028, 2, 29), mesi(1)),
            Ok(data(2028, 3, 28))
        );
    }

    #[test]
    fn trimestrale_attraversa_il_cambio_anno() {
        assert_eq!(
            calcola_data_scadenza(data(2026, 11, 15), mesi(3)),
            Ok(data(2027, 2, 14))
        );
    }

    #[test]
    fn annuale_scade_il_giorno_prima_dell_anno_dopo() {
        assert_eq!(
            calcola_data_scadenza(data(2026, 3, 1), mesi(12)),
            Ok(data(2027, 2, 28))
        );
    }

    #[test]
    fn durata_in_giorni_include_il_giorno_di_inizio() {
        assert_eq!(
            calcola_data_scadenza(data(2026, 1, 1), giorni(1)),
            Ok(data(2026, 1, 1))
        );
        assert_eq!(
            calcola_data_scadenza(data(2026, 1, 1), giorni(30)),
            Ok(data(2026, 1, 30))
        );
    }

    #[test]
    fn durata_in_giorni_attraversa_il_29_febbraio() {
        assert_eq!(
            calcola_data_scadenza(data(2028, 2, 28), giorni(2)),
            Ok(data(2028, 2, 29))
        );
    }

    #[test]
    fn durata_nulla_e_un_errore() {
        assert_eq!(
            calcola_data_scadenza(data(2026, 1, 1), mesi(0)),
            Err(ErroreDominio::DurataNulla)
        );
        assert_eq!(
            calcola_data_scadenza(data(2026, 1, 1), giorni(0)),
            Err(ErroreDominio::DurataNulla)
        );
    }

    #[test]
    fn data_oltre_il_limite_supportato_e_un_errore() {
        assert_eq!(
            calcola_data_scadenza(NaiveDate::MAX, giorni(2)),
            Err(ErroreDominio::DataFuoriIntervallo)
        );
    }
}
