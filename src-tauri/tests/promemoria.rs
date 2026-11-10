mod comune;

use chrono::{Days, NaiveDate};
use comune::DatabaseTemporaneo;
use rusqlite::Connection;
use sp_gestionale_lib::dati_prova::genera_dati_prova;
use sp_gestionale_lib::db::iscritti::{
    DatiIscritto, FiltroIscritti, OrdinamentoIscritti, Riferimento, crea_iscritto, elenca_iscritti,
    imposta_iscritto_archiviato,
};
use sp_gestionale_lib::db::promemoria::{conta_in_scadenza, controlla_promemoria};
use sp_gestionale_lib::domain::stato::StatoValidita;

const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const RIFERIMENTO: Riferimento = Riferimento {
    oggi: OGGI,
    giorni_preavviso: 7,
};

fn nuovo_iscritto(connessione: &Connection, cognome: &str) -> i64 {
    let dati = DatiIscritto {
        nome: "Prova".into(),
        cognome: cognome.into(),
        data_nascita: "1990-01-01".parse().unwrap(),
        luogo_nascita: "Roma".into(),
        telefono: None,
        email: None,
        note: None,
    };
    crea_iscritto(connessione, &dati, RIFERIMENTO).unwrap().id
}

/// Abbonamento che scade tra `giorni` giorni (negativo = già scaduto).
fn abbonamento_che_scade_tra(connessione: &Connection, iscritto_id: i64, giorni: i64) {
    let scadenza = OGGI + chrono::Duration::days(giorni);
    let inizio = scadenza - Days::new(29);
    connessione
        .execute(
            "INSERT INTO pagamenti (iscritto_id, tipologia_nome, importo_cent, data_pagamento, metodo, data_inizio, data_scadenza)
             VALUES (?1, 'Mensile', 5000, ?2, 'contanti', ?2, ?3)",
            rusqlite::params![iscritto_id, inizio, scadenza],
        )
        .unwrap();
}

fn iscrizione_che_finisce_tra(connessione: &Connection, iscritto_id: i64, giorni: i64) {
    let fine = OGGI + chrono::Duration::days(giorni);
    let inizio = fine - Days::new(364);
    connessione
        .execute(
            "INSERT INTO iscrizioni (iscritto_id, importo_cent, data_pagamento, metodo, data_inizio, data_fine)
             VALUES (?1, 3000, ?2, 'contanti', ?2, ?3)",
            rusqlite::params![iscritto_id, inizio, fine],
        )
        .unwrap();
}

#[test]
fn le_scadenze_nel_preavviso_vengono_notificate_una_sola_volta() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let anna = nuovo_iscritto(&connessione, "Anna");
    let bruno = nuovo_iscritto(&connessione, "Bruno");
    let carla = nuovo_iscritto(&connessione, "Carla");
    abbonamento_che_scade_tra(&connessione, anna, 0);
    abbonamento_che_scade_tra(&connessione, bruno, 7);
    abbonamento_che_scade_tra(&connessione, carla, 8);
    iscrizione_che_finisce_tra(&connessione, carla, 3);

    let primo = controlla_promemoria(&mut connessione, RIFERIMENTO).unwrap();
    let secondo = controlla_promemoria(&mut connessione, RIFERIMENTO).unwrap();

    assert_eq!((primo.nuovi.abbonamenti, primo.nuovi.iscrizioni), (2, 1));
    assert_eq!(
        (secondo.nuovi.abbonamenti, secondo.nuovi.iscrizioni),
        (0, 0)
    );
    assert_eq!(
        (
            secondo.in_scadenza.abbonamenti,
            secondo.in_scadenza.iscrizioni
        ),
        (2, 1)
    );
}

#[test]
fn gli_scaduti_e_gli_archiviati_non_generano_promemoria() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let scaduto = nuovo_iscritto(&connessione, "Scaduto");
    let archiviato = nuovo_iscritto(&connessione, "Archiviato");
    abbonamento_che_scade_tra(&connessione, scaduto, -1);
    abbonamento_che_scade_tra(&connessione, archiviato, 2);
    imposta_iscritto_archiviato(&connessione, archiviato, true, RIFERIMENTO).unwrap();

    let esito = controlla_promemoria(&mut connessione, RIFERIMENTO).unwrap();

    assert_eq!(esito.nuovi.abbonamenti, 0);
    assert_eq!(
        conta_in_scadenza(&connessione, RIFERIMENTO)
            .unwrap()
            .abbonamenti,
        0
    );
}

#[test]
fn dopo_il_rinnovo_la_nuova_scadenza_sara_notificata_di_nuovo() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let iscritto = nuovo_iscritto(&connessione, "Rinnova");
    abbonamento_che_scade_tra(&connessione, iscritto, 2);
    controlla_promemoria(&mut connessione, RIFERIMENTO).unwrap();

    // Rinnovo: la nuova scadenza è tra 32 giorni, fuori dal preavviso
    abbonamento_che_scade_tra(&connessione, iscritto, 32);
    assert_eq!(
        conta_in_scadenza(&connessione, RIFERIMENTO)
            .unwrap()
            .abbonamenti,
        0
    );

    // Un mese dopo la nuova scadenza entra nel preavviso e viene notificata
    let un_mese_dopo = Riferimento {
        oggi: OGGI + Days::new(30),
        giorni_preavviso: 7,
    };
    let esito = controlla_promemoria(&mut connessione, un_mese_dopo).unwrap();
    assert_eq!(esito.nuovi.abbonamenti, 1);
}

#[test]
fn l_elenco_si_ordina_per_scadenza() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    genera_dati_prova(&mut connessione, 250, OGGI, 9).unwrap();

    let filtro = FiltroIscritti {
        stato_abbonamento: Some(StatoValidita::InScadenza),
        ordinamento: OrdinamentoIscritti::ScadenzaAbbonamentoCrescente,
        ..Default::default()
    };
    let in_scadenza = elenca_iscritti(&connessione, &filtro, RIFERIMENTO).unwrap();
    let filtro_scaduti = FiltroIscritti {
        stato_abbonamento: Some(StatoValidita::Scaduto),
        ordinamento: OrdinamentoIscritti::ScadenzaAbbonamentoDecrescente,
        ..Default::default()
    };
    let scaduti = elenca_iscritti(&connessione, &filtro_scaduti, RIFERIMENTO).unwrap();

    let date = |elenco: &[sp_gestionale_lib::db::iscritti::Iscritto]| -> Vec<NaiveDate> {
        elenco
            .iter()
            .map(|i| i.ultima_data_scadenza_abbonamento.unwrap())
            .collect()
    };
    assert!(!in_scadenza.is_empty() && !scaduti.is_empty());
    assert!(date(&in_scadenza).is_sorted());
    assert!(date(&scaduti).is_sorted_by(|a, b| a >= b));
    assert_eq!(
        conta_in_scadenza(&connessione, RIFERIMENTO)
            .unwrap()
            .abbonamenti as usize,
        in_scadenza.len()
    );
}
