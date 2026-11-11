//! Statistiche: rinnovi e abbandoni, iscritti attivi, valore medio e previsione degli incassi.
//! Solo calcoli su dati già letti: niente database.

use std::collections::{HashMap, HashSet};

use chrono::{Datelike, Days, Months, NaiveDate};
use serde::Serialize;

use super::date::{Durata, calcola_data_scadenza};
use super::rinnovo::calcola_data_inizio_rinnovo;

/// Dopo la scadenza si aspetta questo tempo prima di considerare perso chi non ha rinnovato.
pub const GIORNI_TOLLERANZA_RIENTRO_PREDEFINITI: u32 = 30;
/// Finestre della previsione degli incassi, in giorni da oggi.
pub const FINESTRE_PREVISIONE_GIORNI: [u32; 3] = [30, 60, 90];

/// Periodo coperto da un abbonamento o da un'iscrizione annuale, estremi inclusi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeriodoIscritto {
    pub iscritto_id: i64,
    pub inizio: NaiveDate,
    pub fine: NaiveDate,
}

/// Cosa è successo dopo una scadenza.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EsitoScadenza {
    Rinnovato,
    Perso,
    /// Scaduto da poco: c'è ancora tempo per rinnovare, non conta nei tassi.
    InAttesa,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConteggioRinnovi {
    pub rinnovati: u32,
    pub persi: u32,
    pub in_attesa: u32,
}

impl ConteggioRinnovi {
    fn aggiungi(&mut self, esito: EsitoScadenza) {
        match esito {
            EsitoScadenza::Rinnovato => self.rinnovati += 1,
            EsitoScadenza::Perso => self.persi += 1,
            EsitoScadenza::InAttesa => self.in_attesa += 1,
        }
    }

    /// Quota di rinnovi tra le scadenze già decise (0–1); assente se non ce ne sono.
    pub fn tasso_rinnovo(&self) -> Option<f64> {
        let decise = self.rinnovati + self.persi;
        if decise == 0 {
            return None;
        }
        Some(f64::from(self.rinnovati) / f64::from(decise))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RinnoviMese {
    /// `AAAA-MM`: il mese della scadenza.
    pub mese: String,
    pub conteggio: ConteggioRinnovi,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiepilogoRinnovi {
    pub totale: ConteggioRinnovi,
    pub per_mese: Vec<RinnoviMese>,
    /// Le scadenze di chi non ha rinnovato, le più recenti prima.
    #[serde(skip)]
    pub scadenze_perse: Vec<PeriodoIscritto>,
}

impl RiepilogoRinnovi {
    /// Il tasso del mese peggiore tra quelli con scadenze già decise.
    pub fn tasso_peggiore(&self) -> Option<f64> {
        self.per_mese
            .iter()
            .filter_map(|mese| mese.conteggio.tasso_rinnovo())
            .min_by(f64::total_cmp)
    }
}

/// Esito della scadenza di `periodo`: rinnova chi ha un periodo che va oltre
/// e inizia al più tardi `giorni_tolleranza` giorni dopo la scadenza.
pub fn esito_scadenza(
    periodo: &PeriodoIscritto,
    periodi_dello_stesso_iscritto: &[PeriodoIscritto],
    oggi: NaiveDate,
    giorni_tolleranza: u32,
) -> EsitoScadenza {
    let limite_rientro = periodo.fine + Days::new(u64::from(giorni_tolleranza));
    let rinnovato = periodi_dello_stesso_iscritto
        .iter()
        .any(|altro| altro.fine > periodo.fine && altro.inizio <= limite_rientro);
    if rinnovato {
        EsitoScadenza::Rinnovato
    } else if oggi <= limite_rientro {
        EsitoScadenza::InAttesa
    } else {
        EsitoScadenza::Perso
    }
}

/// Rinnovi e abbandoni delle scadenze comprese tra `dal` e `al`, anche mese per mese.
pub fn riepiloga_rinnovi(
    periodi: &[PeriodoIscritto],
    dal: NaiveDate,
    al: NaiveDate,
    oggi: NaiveDate,
    giorni_tolleranza: u32,
) -> RiepilogoRinnovi {
    let per_iscritto = raggruppa_per_iscritto(periodi);
    let mut totale = ConteggioRinnovi::default();
    let mut per_mese: Vec<RinnoviMese> = mesi_tra(dal, al)
        .into_iter()
        .map(|mese| RinnoviMese {
            mese: chiave_mese(mese),
            conteggio: ConteggioRinnovi::default(),
        })
        .collect();
    let mut scadenze_perse = Vec::new();

    for periodo in periodi
        .iter()
        .filter(|periodo| periodo.fine >= dal && periodo.fine <= al)
    {
        let stesso_iscritto = &per_iscritto[&periodo.iscritto_id];
        let esito = esito_scadenza(periodo, stesso_iscritto, oggi, giorni_tolleranza);
        totale.aggiungi(esito);
        let chiave = chiave_mese(periodo.fine);
        if let Some(mese) = per_mese.iter_mut().find(|mese| mese.mese == chiave) {
            mese.conteggio.aggiungi(esito);
        }
        if esito == EsitoScadenza::Perso {
            scadenze_perse.push(*periodo);
        }
    }

    scadenze_perse.sort_by(|a, b| b.fine.cmp(&a.fine));
    RiepilogoRinnovi {
        totale,
        per_mese,
        scadenze_perse,
    }
}

fn raggruppa_per_iscritto(periodi: &[PeriodoIscritto]) -> HashMap<i64, Vec<PeriodoIscritto>> {
    let mut per_iscritto: HashMap<i64, Vec<PeriodoIscritto>> = HashMap::new();
    for periodo in periodi {
        per_iscritto
            .entry(periodo.iscritto_id)
            .or_default()
            .push(*periodo);
    }
    per_iscritto
}

/// Persone diverse con un periodo che comprende la data.
pub fn conta_attivi_al(periodi: &[PeriodoIscritto], data: NaiveDate) -> u32 {
    let attivi: HashSet<i64> = periodi
        .iter()
        .filter(|periodo| periodo.inizio <= data && periodo.fine >= data)
        .map(|periodo| periodo.iscritto_id)
        .collect();
    attivi.len() as u32
}

/// Persone diverse con un periodo valido almeno un giorno del mese.
pub fn conta_attivi_nel_mese(periodi: &[PeriodoIscritto], primo_del_mese: NaiveDate) -> u32 {
    let ultimo_del_mese = ultimo_giorno_del_mese(primo_del_mese);
    let attivi: HashSet<i64> = periodi
        .iter()
        .filter(|periodo| periodo.inizio <= ultimo_del_mese && periodo.fine >= primo_del_mese)
        .map(|periodo| periodo.iscritto_id)
        .collect();
    attivi.len() as u32
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttiviMese {
    pub mese: String,
    /// Iscritti con abbonamento valido l'ultimo giorno del mese (oggi per il mese in corso).
    pub attivi: u32,
}

pub fn attivi_per_mese(
    periodi: &[PeriodoIscritto],
    dal: NaiveDate,
    al: NaiveDate,
    oggi: NaiveDate,
) -> Vec<AttiviMese> {
    mesi_tra(dal, al.min(oggi))
        .into_iter()
        .map(|mese| AttiviMese {
            mese: chiave_mese(mese),
            attivi: conta_attivi_al(periodi, ultimo_giorno_del_mese(mese).min(oggi)),
        })
        .collect()
}

/// Un incasso vero (non le voci pregresse), per il valore medio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Incasso {
    pub iscritto_id: i64,
    pub data: NaiveDate,
    pub importo_cent: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValoreMese {
    pub mese: String,
    pub incassi_cent: i64,
    /// Iscritti con abbonamento valido almeno un giorno del mese.
    pub attivi: u32,
    pub spesa_media_cent: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValoreMedio {
    pub incassi_cent: i64,
    pub iscritti_paganti: u32,
    /// Quanto ha speso in media chi ha pagato almeno una volta nel periodo.
    pub valore_medio_cent: Option<i64>,
    /// Incasso medio al mese per ogni iscritto attivo.
    pub spesa_media_mensile_cent: Option<i64>,
    pub per_mese: Vec<ValoreMese>,
}

pub fn calcola_valore_medio(
    incassi: &[Incasso],
    abbonamenti: &[PeriodoIscritto],
    dal: NaiveDate,
    al: NaiveDate,
) -> ValoreMedio {
    let nel_periodo: Vec<&Incasso> = incassi
        .iter()
        .filter(|incasso| incasso.data >= dal && incasso.data <= al)
        .collect();
    let incassi_cent: i64 = nel_periodo.iter().map(|incasso| incasso.importo_cent).sum();
    let paganti: HashSet<i64> = nel_periodo
        .iter()
        .map(|incasso| incasso.iscritto_id)
        .collect();
    let iscritti_paganti = paganti.len() as u32;

    let per_mese: Vec<ValoreMese> = mesi_tra(dal, al)
        .into_iter()
        .map(|mese| valore_del_mese(&nel_periodo, abbonamenti, mese))
        .collect();
    let incassi_mesi: i64 = per_mese.iter().map(|mese| mese.incassi_cent).sum();
    let attivi_mesi: u32 = per_mese.iter().map(|mese| mese.attivi).sum();

    ValoreMedio {
        incassi_cent,
        iscritti_paganti,
        valore_medio_cent: dividi_arrotondando(incassi_cent, iscritti_paganti),
        spesa_media_mensile_cent: dividi_arrotondando(incassi_mesi, attivi_mesi),
        per_mese,
    }
}

fn valore_del_mese(
    incassi: &[&Incasso],
    abbonamenti: &[PeriodoIscritto],
    primo_del_mese: NaiveDate,
) -> ValoreMese {
    let chiave = chiave_mese(primo_del_mese);
    let incassi_cent = incassi
        .iter()
        .filter(|incasso| chiave_mese(incasso.data) == chiave)
        .map(|incasso| incasso.importo_cent)
        .sum();
    let attivi = conta_attivi_nel_mese(abbonamenti, primo_del_mese);
    ValoreMese {
        mese: chiave,
        incassi_cent,
        attivi,
        spesa_media_cent: dividi_arrotondando(incassi_cent, attivi),
    }
}

fn dividi_arrotondando(totale_cent: i64, quanti: u32) -> Option<i64> {
    if quanti == 0 {
        return None;
    }
    Some((totale_cent as f64 / f64::from(quanti)).round() as i64)
}

/// Ultimo abbonamento di un iscritto attivo, con il prezzo del prossimo rinnovo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbbonamentoInCorso {
    pub data_scadenza: NaiveDate,
    pub durata: Durata,
    pub prezzo_cent: i64,
}

/// Ultima iscrizione annuale di un iscritto attivo, con la quota del prossimo rinnovo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuotaInCorso {
    pub data_fine: NaiveDate,
    pub quota_cent: i64,
}

/// Probabilità di rinnovo usate per stimare quanto entrerà.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TassiRinnovo {
    pub abbonamenti: f64,
    pub quote: f64,
}

impl TassiRinnovo {
    /// Rinnovano tutti: il massimo che può entrare.
    pub const TUTTI: Self = Self {
        abbonamenti: 1.0,
        quote: 1.0,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrevisioneFinestra {
    pub giorni: u32,
    /// Con i tassi di rinnovo dell'ultimo anno; assente se non c'è ancora storico.
    pub stima_cent: Option<i64>,
    /// Con il tasso del mese peggiore dell'ultimo anno.
    pub minimo_cent: Option<i64>,
    /// Se rinnovassero tutti.
    pub massimo_cent: i64,
}

/// Quanto dovrebbe entrare nei prossimi `giorni` (oggi compreso) dai rinnovi di abbonamenti e quote.
/// I rinnovi successivi (un mensile si rinnova più volte) valgono sempre meno.
pub fn incasso_atteso(
    abbonamenti: &[AbbonamentoInCorso],
    quote: &[QuotaInCorso],
    oggi: NaiveDate,
    giorni: u32,
    giorni_tolleranza: u32,
    tassi: TassiRinnovo,
) -> i64 {
    let limite = oggi + Days::new(u64::from(giorni));
    let mut totale = 0.0;
    for abbonamento in abbonamenti {
        totale += rinnovi_attesi(
            abbonamento,
            oggi,
            limite,
            giorni_tolleranza,
            tassi.abbonamenti,
        );
    }
    for quota in quote {
        let rinnovo = calcola_data_inizio_rinnovo(Some(quota.data_fine), oggi);
        if ancora_recuperabile(quota.data_fine, oggi, giorni_tolleranza) && rinnovo < limite {
            totale += quota.quota_cent as f64 * tassi.quote;
        }
    }
    totale.round() as i64
}

fn rinnovi_attesi(
    abbonamento: &AbbonamentoInCorso,
    oggi: NaiveDate,
    limite: NaiveDate,
    giorni_tolleranza: u32,
    tasso: f64,
) -> f64 {
    if !ancora_recuperabile(abbonamento.data_scadenza, oggi, giorni_tolleranza) {
        return 0.0;
    }
    let mut totale = 0.0;
    let mut probabilita = 1.0;
    let mut scadenza = abbonamento.data_scadenza;
    loop {
        let inizio = calcola_data_inizio_rinnovo(Some(scadenza), oggi);
        if inizio >= limite {
            return totale;
        }
        probabilita *= tasso;
        totale += abbonamento.prezzo_cent as f64 * probabilita;
        let Ok(prossima_scadenza) = calcola_data_scadenza(inizio, abbonamento.durata) else {
            return totale;
        };
        scadenza = prossima_scadenza;
    }
}

/// Chi è scaduto oltre la tolleranza è già perso: non porta incassi.
fn ancora_recuperabile(scadenza: NaiveDate, oggi: NaiveDate, giorni_tolleranza: u32) -> bool {
    scadenza + Days::new(u64::from(giorni_tolleranza)) >= oggi
}

/// Previsione per ogni finestra: stima, minimo e massimo.
pub fn prevedi_incassi(
    abbonamenti: &[AbbonamentoInCorso],
    quote: &[QuotaInCorso],
    oggi: NaiveDate,
    giorni_tolleranza: u32,
    tassi: Option<TassiRinnovo>,
    tassi_peggiori: Option<TassiRinnovo>,
) -> Vec<PrevisioneFinestra> {
    let incasso_con =
        |giorni, tassi| incasso_atteso(abbonamenti, quote, oggi, giorni, giorni_tolleranza, tassi);
    FINESTRE_PREVISIONE_GIORNI
        .iter()
        .map(|&giorni| PrevisioneFinestra {
            giorni,
            stima_cent: tassi.map(|tassi| incasso_con(giorni, tassi)),
            minimo_cent: tassi_peggiori.map(|tassi| incasso_con(giorni, tassi)),
            massimo_cent: incasso_con(giorni, TassiRinnovo::TUTTI),
        })
        .collect()
}

/// Il primo giorno di ogni mese da quello di `dal` a quello di `al`.
pub fn mesi_tra(dal: NaiveDate, al: NaiveDate) -> Vec<NaiveDate> {
    let mut mesi = Vec::new();
    let mut mese = primo_giorno_del_mese(dal);
    while mese <= al {
        mesi.push(mese);
        mese = mese + Months::new(1);
    }
    mesi
}

pub fn chiave_mese(data: NaiveDate) -> String {
    data.format("%Y-%m").to_string()
}

fn primo_giorno_del_mese(data: NaiveDate) -> NaiveDate {
    data.with_day(1)
        .expect("il primo giorno del mese esiste sempre")
}

fn ultimo_giorno_del_mese(primo_del_mese: NaiveDate) -> NaiveDate {
    (primo_del_mese + Months::new(1))
        .pred_opt()
        .expect("data nell'intervallo supportato")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::date::UnitaDurata;

    const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
    const TOLLERANZA: u32 = 30;

    fn data(testo: &str) -> NaiveDate {
        testo.parse().unwrap()
    }

    fn periodo(iscritto_id: i64, inizio: &str, fine: &str) -> PeriodoIscritto {
        PeriodoIscritto {
            iscritto_id,
            inizio: data(inizio),
            fine: data(fine),
        }
    }

    fn mensile(scadenza: &str, prezzo_cent: i64) -> AbbonamentoInCorso {
        AbbonamentoInCorso {
            data_scadenza: data(scadenza),
            durata: Durata {
                valore: 1,
                unita: UnitaDurata::Mesi,
            },
            prezzo_cent,
        }
    }

    #[test]
    fn rinnova_chi_riparte_entro_la_tolleranza() {
        let scaduto = periodo(1, "2026-08-01", "2026-08-31");
        for (inizio_nuovo, atteso) in [
            ("2026-09-01", EsitoScadenza::Rinnovato),
            ("2026-09-30", EsitoScadenza::Rinnovato),
            ("2026-10-01", EsitoScadenza::Perso),
        ] {
            let nuovo = periodo(1, inizio_nuovo, "2026-12-31");
            let esito = esito_scadenza(&scaduto, &[scaduto, nuovo], OGGI, TOLLERANZA);
            assert_eq!(esito, atteso, "nuovo abbonamento dal {inizio_nuovo}");
        }
    }

    #[test]
    fn il_rinnovo_anticipato_conta_come_rinnovo() {
        let attuale = periodo(1, "2026-10-01", "2026-10-31");
        let anticipato = periodo(1, "2026-11-01", "2026-11-30");
        assert_eq!(
            esito_scadenza(&attuale, &[attuale, anticipato], OGGI, TOLLERANZA),
            EsitoScadenza::Rinnovato
        );
    }

    #[test]
    fn chi_e_scaduto_da_poco_e_in_attesa() {
        let scaduto = periodo(1, "2026-10-01", "2026-10-31");
        assert_eq!(
            esito_scadenza(&scaduto, &[scaduto], OGGI, TOLLERANZA),
            EsitoScadenza::InAttesa
        );
        let oltre = periodo(1, "2026-09-01", "2026-10-10");
        assert_eq!(
            esito_scadenza(&oltre, &[oltre], OGGI, TOLLERANZA),
            EsitoScadenza::Perso
        );
    }

    #[test]
    fn il_riepilogo_conta_solo_le_scadenze_del_periodo_e_le_divide_per_mese() {
        let periodi = [
            periodo(1, "2026-07-01", "2026-07-31"),
            periodo(1, "2026-08-01", "2026-08-31"),
            periodo(2, "2026-07-01", "2026-07-31"),
            periodo(3, "2026-05-01", "2026-05-31"),
        ];
        let riepilogo = riepiloga_rinnovi(
            &periodi,
            data("2026-07-01"),
            data("2026-08-31"),
            OGGI,
            TOLLERANZA,
        );

        assert_eq!(
            riepilogo.totale,
            ConteggioRinnovi {
                rinnovati: 1,
                persi: 2,
                in_attesa: 0
            }
        );
        assert_eq!(riepilogo.per_mese.len(), 2);
        assert_eq!(riepilogo.per_mese[0].conteggio.rinnovati, 1);
        assert_eq!(riepilogo.per_mese[0].conteggio.persi, 1);
        assert_eq!(riepilogo.scadenze_perse[0].iscritto_id, 1);
        assert_eq!(riepilogo.tasso_peggiore(), Some(0.0));
    }

    #[test]
    fn il_tasso_ignora_le_scadenze_in_attesa() {
        let conteggio = ConteggioRinnovi {
            rinnovati: 3,
            persi: 1,
            in_attesa: 10,
        };
        assert_eq!(conteggio.tasso_rinnovo(), Some(0.75));
        assert_eq!(ConteggioRinnovi::default().tasso_rinnovo(), None);
    }

    #[test]
    fn conta_una_volta_chi_ha_periodi_sovrapposti() {
        let periodi = [
            periodo(1, "2026-10-01", "2026-12-31"),
            periodo(1, "2026-11-01", "2026-11-30"),
            periodo(2, "2026-01-01", "2026-01-31"),
        ];
        assert_eq!(conta_attivi_al(&periodi, OGGI), 1);
        assert_eq!(conta_attivi_nel_mese(&periodi, data("2026-01-01")), 1);
    }

    #[test]
    fn gli_attivi_del_mese_in_corso_si_contano_a_oggi() {
        let periodi = [
            periodo(1, "2026-11-01", "2026-11-12"),
            periodo(2, "2026-10-01", "2026-10-31"),
        ];
        let attivi = attivi_per_mese(&periodi, data("2026-10-01"), data("2026-12-31"), OGGI);
        assert_eq!(
            attivi,
            vec![
                AttiviMese {
                    mese: "2026-10".into(),
                    attivi: 1
                },
                AttiviMese {
                    mese: "2026-11".into(),
                    attivi: 1
                },
            ]
        );
    }

    #[test]
    fn il_valore_medio_divide_per_chi_ha_pagato() {
        let incassi = [
            Incasso {
                iscritto_id: 1,
                data: data("2026-10-05"),
                importo_cent: 5000,
            },
            Incasso {
                iscritto_id: 1,
                data: data("2026-11-05"),
                importo_cent: 5000,
            },
            Incasso {
                iscritto_id: 2,
                data: data("2026-11-06"),
                importo_cent: 2000,
            },
        ];
        let abbonamenti = [
            periodo(1, "2026-10-05", "2026-12-04"),
            periodo(2, "2026-11-06", "2026-12-05"),
        ];
        let valore = calcola_valore_medio(
            &incassi,
            &abbonamenti,
            data("2026-10-01"),
            data("2026-11-30"),
        );

        assert_eq!(valore.incassi_cent, 12000);
        assert_eq!(valore.iscritti_paganti, 2);
        assert_eq!(valore.valore_medio_cent, Some(6000));
        // Ottobre 5000 su 1 attivo, novembre 7000 su 2: 12000 / 3
        assert_eq!(valore.spesa_media_mensile_cent, Some(4000));
        assert_eq!(valore.per_mese[1].spesa_media_cent, Some(3500));
    }

    #[test]
    fn senza_incassi_il_valore_medio_non_c_e() {
        let valore = calcola_valore_medio(&[], &[], data("2026-10-01"), data("2026-10-31"));
        assert_eq!(valore.valore_medio_cent, None);
        assert_eq!(valore.spesa_media_mensile_cent, None);
    }

    #[test]
    fn un_mensile_in_novanta_giorni_si_rinnova_piu_volte_con_probabilita_decrescente() {
        let abbonamenti = [mensile("2026-11-20", 5000)];
        let tassi = TassiRinnovo {
            abbonamenti: 0.5,
            quote: 1.0,
        };
        // Rinnovi il 21/11, il 21/12 e il 21/01: 2500 + 1250 + 625
        assert_eq!(
            incasso_atteso(&abbonamenti, &[], OGGI, 90, TOLLERANZA, tassi),
            4375
        );
        assert_eq!(
            incasso_atteso(&abbonamenti, &[], OGGI, 30, TOLLERANZA, tassi),
            2500
        );
        assert_eq!(
            incasso_atteso(&abbonamenti, &[], OGGI, 90, TOLLERANZA, TassiRinnovo::TUTTI),
            15000
        );
    }

    #[test]
    fn chi_e_gia_perso_non_porta_incassi_e_chi_e_scaduto_da_poco_rinnova_oggi() {
        let perso = mensile("2026-09-01", 5000);
        let scaduto_da_poco = mensile("2026-11-01", 5000);
        assert_eq!(
            incasso_atteso(&[perso], &[], OGGI, 30, TOLLERANZA, TassiRinnovo::TUTTI),
            0
        );
        assert_eq!(
            incasso_atteso(
                &[scaduto_da_poco],
                &[],
                OGGI,
                30,
                TOLLERANZA,
                TassiRinnovo::TUTTI
            ),
            5000
        );
    }

    #[test]
    fn la_quota_annuale_si_rinnova_una_volta_nella_finestra() {
        let quote = [QuotaInCorso {
            data_fine: data("2026-12-31"),
            quota_cent: 3000,
        }];
        let tassi = TassiRinnovo {
            abbonamenti: 1.0,
            quote: 0.8,
        };
        assert_eq!(incasso_atteso(&[], &quote, OGGI, 30, TOLLERANZA, tassi), 0);
        assert_eq!(
            incasso_atteso(&[], &quote, OGGI, 60, TOLLERANZA, tassi),
            2400
        );
    }

    #[test]
    fn senza_storico_la_previsione_da_solo_il_massimo() {
        let previsione = prevedi_incassi(
            &[mensile("2026-11-20", 5000)],
            &[],
            OGGI,
            TOLLERANZA,
            None,
            None,
        );
        assert_eq!(previsione.len(), 3);
        assert_eq!(previsione[0].stima_cent, None);
        assert_eq!(previsione[0].massimo_cent, 5000);
    }

    #[test]
    fn i_mesi_comprendono_quello_di_inizio_e_di_fine() {
        assert_eq!(
            mesi_tra(data("2026-01-15"), data("2026-03-01")),
            vec![data("2026-01-01"), data("2026-02-01"), data("2026-03-01")]
        );
    }
}
