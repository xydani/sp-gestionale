mod comune;

use chrono::NaiveDate;
use comune::DatabaseTemporaneo;
use rusqlite::Connection;
use sp_gestionale_lib::db::iscritti::{
    DatiIscritto, FiltroIscritti, Riferimento, elenca_iscritti, modifica_iscritto,
};
use sp_gestionale_lib::db::iscritto_gia_attivo::{
    DatiIscrittoGiaAttivo, registra_iscritto_gia_attivo,
};
use sp_gestionale_lib::db::iscrizioni::elenca_iscrizioni;
use sp_gestionale_lib::db::pagamenti::{FiltroPagamenti, elenca_pagamenti, proponi_pagamento};
use sp_gestionale_lib::db::report::{Raggruppamento, riepilogo_incassi};
use sp_gestionale_lib::domain::stato::StatoValidita;

const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const RIFERIMENTO: Riferimento = Riferimento {
    oggi: OGGI,
    giorni_preavviso: 7,
};
const MENSILE: i64 = 1;

fn data(testo: &str) -> NaiveDate {
    testo.parse().unwrap()
}

fn conta(connessione: &Connection, tabella: &str) -> i64 {
    connessione
        .query_row(&format!("SELECT count(*) FROM {tabella}"), [], |riga| {
            riga.get(0)
        })
        .unwrap()
}

fn dati_minimi() -> DatiIscrittoGiaAttivo {
    DatiIscrittoGiaAttivo {
        nome: " Luca ".into(),
        cognome: "Ferri".into(),
        telefono: Some(" ".into()),
        data_scadenza_certificato: None,
        tipologia_id: MENSILE,
        data_scadenza_abbonamento: data("2026-11-25"),
        quota_pagata: true,
    }
}

#[test]
fn l_iscritto_gia_attivo_ha_abbonamento_e_quota_validi_ma_da_completare() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();

    let iscritto =
        registra_iscritto_gia_attivo(&mut connessione, &dati_minimi(), RIFERIMENTO).unwrap();

    assert_eq!(iscritto.nome, "Luca");
    assert_eq!(iscritto.telefono, None);
    assert!(iscritto.da_completare);
    assert_eq!(iscritto.stato_abbonamento, StatoValidita::Attivo);
    assert_eq!(
        iscritto.ultima_data_scadenza_abbonamento,
        Some(data("2026-11-25"))
    );
    assert_eq!(iscritto.stato_iscrizione, StatoValidita::Attivo);
    assert_eq!(
        iscritto.ultima_data_fine_iscrizione,
        Some(data("2027-08-31"))
    );
}

#[test]
fn l_abbonamento_pregresso_inizia_una_durata_prima_della_scadenza() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let iscritto =
        registra_iscritto_gia_attivo(&mut connessione, &dati_minimi(), RIFERIMENTO).unwrap();

    let filtro = FiltroPagamenti {
        iscritto_id: Some(iscritto.id),
        ..Default::default()
    };
    let pagamenti = elenca_pagamenti(&connessione, &filtro).unwrap();

    assert_eq!(pagamenti.len(), 1);
    assert!(pagamenti[0].pregresso);
    assert_eq!(pagamenti[0].importo_cent, 0);
    assert_eq!(pagamenti[0].data_inizio, data("2026-10-26"));
    let iscrizioni = elenca_iscrizioni(&connessione, iscritto.id).unwrap();
    assert!(iscrizioni[0].pregresso);
}

#[test]
fn il_rinnovo_parte_dal_giorno_dopo_la_scadenza_pregressa() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let iscritto =
        registra_iscritto_gia_attivo(&mut connessione, &dati_minimi(), RIFERIMENTO).unwrap();

    let proposta = proponi_pagamento(&connessione, Some(iscritto.id), MENSILE, None, OGGI).unwrap();

    assert_eq!(proposta.data_inizio, data("2026-11-26"));
    assert!(proposta.rinnovo_di_id.is_some());
}

#[test]
fn le_voci_pregresse_non_sono_incassi() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    registra_iscritto_gia_attivo(&mut connessione, &dati_minimi(), RIFERIMENTO).unwrap();

    let riepilogo =
        riepilogo_incassi(&connessione, data("2025-01-01"), OGGI, Raggruppamento::Mese).unwrap();
    let pagamenti = elenca_pagamenti(&connessione, &FiltroPagamenti::default()).unwrap();

    assert_eq!(riepilogo.numero_movimenti, 0);
    assert!(pagamenti.is_empty());
}

#[test]
fn senza_quota_pagata_non_si_registra_l_iscrizione() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let dati = DatiIscrittoGiaAttivo {
        quota_pagata: false,
        ..dati_minimi()
    };

    let iscritto = registra_iscritto_gia_attivo(&mut connessione, &dati, RIFERIMENTO).unwrap();

    assert_eq!(iscritto.stato_iscrizione, StatoValidita::Nessuno);
    assert_eq!(conta(&connessione, "iscrizioni"), 0);
}

#[test]
fn una_scadenza_oltre_la_durata_non_salva_nulla() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let dati = DatiIscrittoGiaAttivo {
        data_scadenza_abbonamento: data("2027-03-01"),
        ..dati_minimi()
    };

    let errore = registra_iscritto_gia_attivo(&mut connessione, &dati, RIFERIMENTO).unwrap_err();

    assert!(errore.to_string().contains("durata"));
    assert_eq!(conta(&connessione, "iscritti"), 0);
}

#[test]
fn nome_non_valido_rifiutato() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let dati = DatiIscrittoGiaAttivo {
        cognome: "123".into(),
        ..dati_minimi()
    };

    assert!(registra_iscritto_gia_attivo(&mut connessione, &dati, RIFERIMENTO).is_err());
    assert_eq!(conta(&connessione, "iscritti"), 0);
}

#[test]
fn il_filtro_trova_gli_iscritti_da_completare_finche_non_si_completano() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let iscritto =
        registra_iscritto_gia_attivo(&mut connessione, &dati_minimi(), RIFERIMENTO).unwrap();
    let filtro = FiltroIscritti {
        solo_da_completare: true,
        ..Default::default()
    };
    assert_eq!(
        elenca_iscritti(&connessione, &filtro, RIFERIMENTO)
            .unwrap()
            .len(),
        1
    );

    let completo = DatiIscritto {
        nome: iscritto.nome.clone(),
        cognome: iscritto.cognome.clone(),
        data_nascita: data("1990-02-03"),
        luogo_nascita: "Pisa".into(),
        telefono: None,
        email: None,
        note: None,
        data_scadenza_certificato: None,
    };
    let completato = modifica_iscritto(&connessione, iscritto.id, &completo, RIFERIMENTO).unwrap();

    assert!(!completato.da_completare);
    assert!(
        elenca_iscritti(&connessione, &filtro, RIFERIMENTO)
            .unwrap()
            .is_empty()
    );
}
