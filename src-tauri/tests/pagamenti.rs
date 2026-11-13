mod comune;

use chrono::NaiveDate;
use comune::DatabaseTemporaneo;
use rusqlite::Connection;
use sp_gestionale_lib::dati_prova::genera_dati_prova;
use sp_gestionale_lib::db::ErroreDatabase;
use sp_gestionale_lib::db::iscritti::{DatiIscritto, Riferimento, crea_iscritto, trova_iscritto};
use sp_gestionale_lib::db::pagamenti::{
    DatiPagamento, FiltroPagamenti, elenca_pagamenti, elenca_pagamenti_paginati, elimina_pagamento,
    proponi_pagamento, registra_pagamento,
};
use sp_gestionale_lib::db::paginazione::{ELEMENTI_PER_PAGINA, RichiestaPagina};
use sp_gestionale_lib::db::tipologie::{DatiTipologia, modifica_tipologia};
use sp_gestionale_lib::domain::date::UnitaDurata;
use sp_gestionale_lib::domain::pagamento::MetodoPagamento;
use sp_gestionale_lib::domain::stato::StatoValidita;

const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const RIFERIMENTO: Riferimento = Riferimento {
    oggi: OGGI,
    giorni_preavviso: 7,
};
const MENSILE: i64 = 1;
const TRIMESTRALE: i64 = 2;

fn data(testo: &str) -> NaiveDate {
    testo.parse().unwrap()
}

fn nuovo_iscritto(connessione: &Connection) -> i64 {
    let dati = DatiIscritto {
        nome: "Mario".into(),
        cognome: "Rossi".into(),
        data_nascita: data("1990-05-20"),
        luogo_nascita: "Roma".into(),
        telefono: None,
        email: None,
        note: None,
        data_scadenza_certificato: None,
        certificato_presentato: false,
    };
    crea_iscritto(connessione, &dati, RIFERIMENTO).unwrap().id
}

/// Registra il pagamento esattamente come proposto dal programma.
fn paga_come_proposto(
    connessione: &Connection,
    iscritto_id: i64,
    tipologia_id: i64,
    oggi: NaiveDate,
) -> i64 {
    let proposta =
        proponi_pagamento(connessione, Some(iscritto_id), tipologia_id, None, oggi).unwrap();
    let dati = DatiPagamento {
        iscritto_id,
        tipologia_id,
        importo_cent: proposta.importo_cent,
        data_pagamento: oggi,
        metodo: MetodoPagamento::Contanti,
        data_inizio: proposta.data_inizio,
        data_scadenza: proposta.data_scadenza,
        note: None,
        rinnovo_di_id: proposta.rinnovo_di_id,
    };
    registra_pagamento(connessione, &dati, oggi).unwrap().id
}

#[test]
fn il_primo_pagamento_parte_da_oggi_con_il_prezzo_di_listino() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = nuovo_iscritto(&connessione);

    let proposta = proponi_pagamento(&connessione, Some(iscritto), MENSILE, None, OGGI).unwrap();

    assert_eq!(proposta.importo_cent, 5000);
    assert_eq!(proposta.data_inizio, OGGI);
    assert_eq!(proposta.data_scadenza, data("2026-12-09"));
    assert_eq!(proposta.rinnovo_di_id, None);
}

#[test]
fn il_rinnovo_anticipato_parte_dal_giorno_dopo_la_scadenza() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = nuovo_iscritto(&connessione);
    let primo = paga_come_proposto(&connessione, iscritto, MENSILE, OGGI);

    let proposta = proponi_pagamento(
        &connessione,
        Some(iscritto),
        TRIMESTRALE,
        None,
        data("2026-12-05"),
    )
    .unwrap();

    assert_eq!(proposta.data_inizio, data("2026-12-10"));
    assert_eq!(proposta.data_scadenza, data("2027-03-09"));
    assert_eq!(proposta.importo_cent, 13500);
    assert_eq!(proposta.rinnovo_di_id, Some(primo));
}

#[test]
fn il_rinnovo_dopo_la_scadenza_parte_da_oggi() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = nuovo_iscritto(&connessione);
    paga_come_proposto(&connessione, iscritto, MENSILE, data("2026-08-01"));

    let proposta = proponi_pagamento(&connessione, Some(iscritto), MENSILE, None, OGGI).unwrap();

    assert_eq!(proposta.data_inizio, OGGI);
}

#[test]
fn una_data_di_inizio_scelta_ricalcola_la_scadenza() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = nuovo_iscritto(&connessione);

    let proposta = proponi_pagamento(
        &connessione,
        Some(iscritto),
        MENSILE,
        Some(data("2026-11-20")),
        OGGI,
    )
    .unwrap();

    assert_eq!(proposta.data_inizio, data("2026-11-20"));
    assert_eq!(proposta.data_scadenza, data("2026-12-19"));
}

#[test]
fn il_pagamento_aggiorna_lo_stato_dell_iscritto() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = nuovo_iscritto(&connessione);
    assert_eq!(
        trova_iscritto(&connessione, iscritto, RIFERIMENTO)
            .unwrap()
            .stato_abbonamento,
        StatoValidita::Nessuno
    );

    paga_come_proposto(&connessione, iscritto, MENSILE, OGGI);

    let aggiornato = trova_iscritto(&connessione, iscritto, RIFERIMENTO).unwrap();
    assert_eq!(aggiornato.stato_abbonamento, StatoValidita::Attivo);
    assert_eq!(
        aggiornato.ultima_data_scadenza_abbonamento,
        Some(data("2026-12-09"))
    );
}

#[test]
fn lo_storico_conserva_il_nome_della_tipologia_al_momento_del_pagamento() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = nuovo_iscritto(&connessione);
    paga_come_proposto(&connessione, iscritto, MENSILE, OGGI);

    let rinominata = DatiTipologia {
        nome: "Mensile Plus".into(),
        durata_valore: 1,
        durata_unita: UnitaDurata::Mesi,
        prezzo_cent: 6000,
    };
    modifica_tipologia(&connessione, MENSILE, &rinominata).unwrap();

    let filtro = FiltroPagamenti {
        iscritto_id: Some(iscritto),
        ..Default::default()
    };
    let storico = elenca_pagamenti(&connessione, &filtro).unwrap();
    assert_eq!(storico[0].tipologia_nome, "Mensile");
    assert_eq!(storico[0].importo_cent, 5000);
}

#[test]
fn dati_non_validi_sono_rifiutati() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = nuovo_iscritto(&connessione);
    let base = DatiPagamento {
        iscritto_id: iscritto,
        tipologia_id: MENSILE,
        importo_cent: 5000,
        data_pagamento: OGGI,
        metodo: MetodoPagamento::Carta,
        data_inizio: OGGI,
        data_scadenza: data("2026-12-09"),
        note: None,
        rinnovo_di_id: None,
    };

    let importo_negativo = DatiPagamento {
        importo_cent: -1,
        ..base.clone()
    };
    let scadenza_prima = DatiPagamento {
        data_scadenza: data("2026-11-09"),
        ..base.clone()
    };
    let iscritto_inesistente = DatiPagamento {
        iscritto_id: 999,
        ..base.clone()
    };
    let tipologia_inesistente = DatiPagamento {
        tipologia_id: 999,
        ..base
    };

    for dati in [importo_negativo, scadenza_prima, iscritto_inesistente] {
        assert!(matches!(
            registra_pagamento(&connessione, &dati, OGGI),
            Err(ErroreDatabase::DatiNonValidi(_))
        ));
    }
    assert!(matches!(
        registra_pagamento(&connessione, &tipologia_inesistente, OGGI),
        Err(ErroreDatabase::NonTrovato)
    ));
}

#[test]
fn i_filtri_per_periodo_metodo_e_iscritto_funzionano() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    genera_dati_prova(&mut connessione, 250, OGGI, 5).unwrap();

    let tutti = elenca_pagamenti(&connessione, &FiltroPagamenti::default()).unwrap();
    let filtro_ottobre = FiltroPagamenti {
        dal: Some(data("2026-10-01")),
        al: Some(data("2026-10-31")),
        metodo: Some(MetodoPagamento::Carta),
        ..Default::default()
    };
    let ottobre_carta = elenca_pagamenti(&connessione, &filtro_ottobre).unwrap();

    assert!(!ottobre_carta.is_empty());
    assert!(ottobre_carta.len() < tutti.len());
    assert!(ottobre_carta.iter().all(|pagamento| {
        pagamento.metodo == MetodoPagamento::Carta
            && pagamento.data_pagamento >= data("2026-10-01")
            && pagamento.data_pagamento <= data("2026-10-31")
    }));
    // Più recenti per primi
    assert!(
        tutti
            .windows(2)
            .all(|coppia| coppia[0].data_pagamento >= coppia[1].data_pagamento)
    );
}

#[test]
fn si_elimina_un_pagamento_registrato_per_errore() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = nuovo_iscritto(&connessione);
    let primo = paga_come_proposto(&connessione, iscritto, MENSILE, OGGI);
    let rinnovo = paga_come_proposto(&connessione, iscritto, MENSILE, OGGI);

    elimina_pagamento(&connessione, primo).unwrap();

    let filtro = FiltroPagamenti {
        iscritto_id: Some(iscritto),
        ..Default::default()
    };
    let rimasti = elenca_pagamenti(&connessione, &filtro).unwrap();
    assert_eq!(rimasti.len(), 1);
    assert_eq!(rimasti[0].id, rinnovo);
    assert_eq!(rimasti[0].rinnovo_di_id, None);
    assert!(matches!(
        elimina_pagamento(&connessione, primo),
        Err(ErroreDatabase::NonTrovato)
    ));
}

#[test]
fn le_pagine_contengono_al_massimo_50_pagamenti_e_il_totale_e_su_tutto_il_filtro() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    genera_dati_prova(&mut connessione, 250, OGGI, 5).unwrap();
    let filtro = FiltroPagamenti::default();
    let tutti = elenca_pagamenti(&connessione, &filtro).unwrap();
    let totale_atteso: i64 = tutti.iter().map(|pagamento| pagamento.importo_cent).sum();

    let prima =
        elenca_pagamenti_paginati(&connessione, &filtro, RichiestaPagina { numero: 1 }).unwrap();
    let seconda =
        elenca_pagamenti_paginati(&connessione, &filtro, RichiestaPagina { numero: 2 }).unwrap();

    assert_eq!(prima.pagina.elementi.len(), ELEMENTI_PER_PAGINA as usize);
    assert_eq!(prima.pagina.totale_elementi as usize, tutti.len());
    assert_eq!(prima.totale_importo_cent, totale_atteso);
    assert_eq!(prima.pagina.elementi[..], tutti[..50]);
    assert_eq!(seconda.pagina.elementi[..], tutti[50..100]);
}

#[test]
fn senza_pagamenti_la_pagina_e_vuota_con_totale_zero() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let pagina = elenca_pagamenti_paginati(
        &connessione,
        &FiltroPagamenti::default(),
        RichiestaPagina::default(),
    )
    .unwrap();
    assert!(pagina.pagina.elementi.is_empty());
    assert_eq!(pagina.pagina.totale_elementi, 0);
    assert_eq!(pagina.totale_importo_cent, 0);
}
