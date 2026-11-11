mod comune;

use std::time::{Duration, Instant};

use chrono::NaiveDate;
use comune::DatabaseTemporaneo;
use rusqlite::Connection;
use sp_gestionale_lib::dati_prova::genera_dati_prova;
use sp_gestionale_lib::db::iscritti::{DatiIscritto, Riferimento, crea_iscritto};
use sp_gestionale_lib::db::iscritto_gia_attivo::{
    DatiIscrittoGiaAttivo, registra_iscritto_gia_attivo,
};
use sp_gestionale_lib::db::pagamenti::{DatiPagamento, registra_pagamento};
use sp_gestionale_lib::db::report::{Raggruppamento, riepilogo_incassi};
use sp_gestionale_lib::db::statistiche::leggi_statistiche;
use sp_gestionale_lib::domain::pagamento::MetodoPagamento;

const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const RIFERIMENTO: Riferimento = Riferimento {
    oggi: OGGI,
    giorni_preavviso: 7,
};
const MENSILE: i64 = 1;
const TEMPO_MASSIMO: Duration = Duration::from_millis(300);

fn data(testo: &str) -> NaiveDate {
    testo.parse().unwrap()
}

fn iscritto(connessione: &Connection, nome: &str) -> i64 {
    let dati = DatiIscritto {
        nome: nome.into(),
        cognome: "Prova".into(),
        data_nascita: data("1990-01-01"),
        luogo_nascita: "Roma".into(),
        telefono: None,
        email: None,
        note: None,
        data_scadenza_certificato: None,
    };
    crea_iscritto(connessione, &dati, RIFERIMENTO).unwrap().id
}

fn mensile(connessione: &Connection, iscritto_id: i64, inizio: &str, scadenza: &str) {
    let dati = DatiPagamento {
        iscritto_id,
        tipologia_id: MENSILE,
        importo_cent: 5000,
        data_pagamento: data(inizio),
        metodo: MetodoPagamento::Contanti,
        data_inizio: data(inizio),
        data_scadenza: data(scadenza),
        note: None,
        rinnovo_di_id: None,
    };
    registra_pagamento(connessione, &dati, OGGI).unwrap();
}

#[test]
fn rinnovi_abbandoni_e_valore_medio_su_un_caso_noto() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let fedele = iscritto(&connessione, "Fedele");
    mensile(&connessione, fedele, "2026-08-01", "2026-08-31");
    mensile(&connessione, fedele, "2026-09-01", "2026-09-30");
    let perso = iscritto(&connessione, "Perso");
    mensile(&connessione, perso, "2026-08-01", "2026-08-31");
    // Già iscritto prima del programma: conta per i rinnovi, non per gli incassi
    let gia_attivo = DatiIscrittoGiaAttivo {
        nome: "Pregresso".into(),
        cognome: "Prova".into(),
        telefono: None,
        data_scadenza_certificato: None,
        tipologia_id: MENSILE,
        data_scadenza_abbonamento: data("2026-11-20"),
        quota_pagata: false,
    };
    registra_iscritto_gia_attivo(&mut connessione, &gia_attivo, RIFERIMENTO).unwrap();

    let statistiche =
        leggi_statistiche(&connessione, data("2026-08-01"), data("2026-08-31"), OGGI).unwrap();

    let rinnovi = &statistiche.rinnovi_abbonamenti;
    assert_eq!((rinnovi.totale.rinnovati, rinnovi.totale.persi), (1, 1));
    assert_eq!(rinnovi.tasso_rinnovo, Some(0.5));
    assert_eq!(statistiche.persi.len(), 1);
    assert_eq!(statistiche.persi[0].nome, "Perso");
    assert_eq!(statistiche.persi[0].tipologia_nome, "Mensile");
    assert_eq!(statistiche.valore_medio.incassi_cent, 10000);
    assert_eq!(statistiche.valore_medio.iscritti_paganti, 2);
    assert_eq!(statistiche.valore_medio.valore_medio_cent, Some(5000));
    assert_eq!(statistiche.vendite_per_tipologia.totali[0].numero, 2);
    assert_eq!(statistiche.giorni_tolleranza_rientro, 30);
}

#[test]
fn un_periodo_invertito_e_rifiutato() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    assert!(leggi_statistiche(&connessione, OGGI, data("2026-01-01"), OGGI).is_err());
}

#[test]
fn sui_dati_di_prova_i_numeri_sono_coerenti_e_veloci() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    genera_dati_prova(&mut connessione, 250, OGGI, 42).unwrap();
    let (dal, al) = (data("2025-11-11"), OGGI);

    let inizio = Instant::now();
    let statistiche = leggi_statistiche(&connessione, dal, al, OGGI).unwrap();
    let durata = inizio.elapsed();
    assert!(durata < TEMPO_MASSIMO, "statistiche in {durata:?}");

    let rinnovi = &statistiche.rinnovi_abbonamenti;
    let somma_mesi: u32 = rinnovi
        .per_mese
        .iter()
        .map(|mese| mese.conteggio.rinnovati + mese.conteggio.persi)
        .sum();
    assert_eq!(somma_mesi, rinnovi.totale.rinnovati + rinnovi.totale.persi);
    assert!(rinnovi.totale.rinnovati > 0 && rinnovi.totale.persi > 0);
    assert!(statistiche.persi.len() as u32 <= rinnovi.totale.persi);

    let incassi = riepilogo_incassi(&connessione, dal, al, Raggruppamento::Anno).unwrap();
    assert_eq!(
        statistiche.valore_medio.incassi_cent,
        incassi.totale.totale_cent
    );
    let venduto: i64 = statistiche
        .vendite_per_tipologia
        .totali
        .iter()
        .map(|tipologia| tipologia.importo_cent)
        .sum();
    assert_eq!(venduto, incassi.totale.abbonamenti_cent);

    let mut precedente = 0;
    for finestra in &statistiche.previsione {
        let stima = finestra.stima_cent.unwrap();
        assert!(finestra.minimo_cent.unwrap() <= stima);
        assert!(stima <= finestra.massimo_cent);
        assert!(stima >= precedente, "la stima cresce con la finestra");
        precedente = stima;
    }
    assert_eq!(statistiche.attivi_per_mese.len(), 13);
}
