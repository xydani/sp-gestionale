use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Giorni di preavviso predefiniti per segnalare un abbonamento in scadenza.
pub const GIORNI_PREAVVISO_PREDEFINITI: u32 = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatoAbbonamento {
    Attivo,
    InScadenza,
    Scaduto,
    Nessuno,
}

/// Stato dell'abbonamento di un iscritto in base alla sua ultima scadenza.
///
/// Il giorno di scadenza è ancora valido: diventa scaduto dal giorno dopo.
pub fn calcola_stato_abbonamento(
    ultima_data_scadenza: Option<NaiveDate>,
    oggi: NaiveDate,
    giorni_preavviso: u32,
) -> StatoAbbonamento {
    let Some(data_scadenza) = ultima_data_scadenza else {
        return StatoAbbonamento::Nessuno;
    };

    let giorni_rimanenti = giorni_alla_scadenza(data_scadenza, oggi);
    if giorni_rimanenti < 0 {
        StatoAbbonamento::Scaduto
    } else if giorni_rimanenti <= i64::from(giorni_preavviso) {
        StatoAbbonamento::InScadenza
    } else {
        StatoAbbonamento::Attivo
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

    fn stato_con_scadenza_tra(giorni: i64) -> StatoAbbonamento {
        let scadenza = OGGI + chrono::Duration::days(giorni);
        calcola_stato_abbonamento(Some(scadenza), OGGI, GIORNI_PREAVVISO_PREDEFINITI)
    }

    #[test]
    fn senza_pagamenti_lo_stato_e_nessuno() {
        assert_eq!(
            calcola_stato_abbonamento(None, OGGI, GIORNI_PREAVVISO_PREDEFINITI),
            StatoAbbonamento::Nessuno
        );
    }

    #[test]
    fn scaduto_ieri_e_scaduto() {
        assert_eq!(stato_con_scadenza_tra(-1), StatoAbbonamento::Scaduto);
    }

    #[test]
    fn scade_oggi_e_in_scadenza() {
        assert_eq!(stato_con_scadenza_tra(0), StatoAbbonamento::InScadenza);
    }

    #[test]
    fn scade_all_ultimo_giorno_di_preavviso_e_in_scadenza() {
        assert_eq!(stato_con_scadenza_tra(7), StatoAbbonamento::InScadenza);
    }

    #[test]
    fn scade_dopo_il_preavviso_e_attivo() {
        assert_eq!(stato_con_scadenza_tra(8), StatoAbbonamento::Attivo);
    }

    #[test]
    fn preavviso_zero_segnala_solo_il_giorno_di_scadenza() {
        let domani = OGGI.succ_opt().unwrap();
        assert_eq!(
            calcola_stato_abbonamento(Some(OGGI), OGGI, 0),
            StatoAbbonamento::InScadenza
        );
        assert_eq!(
            calcola_stato_abbonamento(Some(domani), OGGI, 0),
            StatoAbbonamento::Attivo
        );
    }

    #[test]
    fn giorni_alla_scadenza_e_negativo_se_scaduto() {
        let tre_giorni_fa = OGGI - chrono::Duration::days(3);
        assert_eq!(giorni_alla_scadenza(tre_giorni_fa, OGGI), -3);
        assert_eq!(giorni_alla_scadenza(OGGI, OGGI), 0);
    }

    #[test]
    fn stato_si_serializza_in_snake_case() {
        assert_eq!(
            serde_json::to_string(&StatoAbbonamento::InScadenza).unwrap(),
            "\"in_scadenza\""
        );
    }
}
