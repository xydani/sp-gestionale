mod comune;

use chrono::NaiveDate;
use comune::{DatabaseTemporaneo, PASSWORD_TEST};
use rusqlite::Connection;
use sp_gestionale_lib::dati_prova::genera_dati_prova;
use sp_gestionale_lib::db::ErroreDatabase;
use sp_gestionale_lib::db::crittografia::{apri_database_criptato, crea_database_criptato};
use sp_gestionale_lib::db::iscritti::{
    DatiIscritto, FiltroIscritti, Riferimento, crea_iscritto, elenca_iscritti,
    elenca_iscritti_paginati, trova_iscritto,
};
use sp_gestionale_lib::db::iscrizioni::{
    DatiIscrizione, elenca_iscrizioni, elimina_iscrizione, proponi_iscrizione, registra_iscrizione,
};
use sp_gestionale_lib::db::migrazioni::{
    applica_migrazioni_fino_a, versione_schema_corrente, versione_schema_supportata,
};
use sp_gestionale_lib::db::paginazione::RichiestaPagina;
use sp_gestionale_lib::domain::pagamento::MetodoPagamento;
use sp_gestionale_lib::domain::stato::StatoValidita;
use sp_gestionale_lib::stato_app::StatoApp;
use zeroize::Zeroizing;

const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const RIFERIMENTO: Riferimento = Riferimento {
    oggi: OGGI,
    giorni_preavviso: 7,
};

fn data(testo: &str) -> NaiveDate {
    testo.parse().unwrap()
}

fn nuovo_iscritto(connessione: &Connection) -> i64 {
    let dati = DatiIscritto {
        nome: "Giulia".into(),
        cognome: "Bianchi".into(),
        data_nascita: data("1995-03-12"),
        luogo_nascita: "Milano".into(),
        telefono: None,
        email: Some("giulia@esempio.it".into()),
        note: None,
    };
    crea_iscritto(connessione, &dati, RIFERIMENTO).unwrap().id
}

fn iscrivi_come_proposto(connessione: &Connection, iscritto_id: i64, oggi: NaiveDate) -> i64 {
    let proposta = proponi_iscrizione(connessione, Some(iscritto_id), oggi).unwrap();
    let dati = DatiIscrizione {
        iscritto_id,
        importo_cent: proposta.importo_cent,
        data_pagamento: oggi,
        metodo: MetodoPagamento::Contanti,
        data_inizio: proposta.data_inizio,
        data_fine: proposta.data_fine,
        note: None,
    };
    registra_iscrizione(connessione, &dati).unwrap().id
}

#[test]
fn la_prima_iscrizione_copre_l_anno_sportivo_in_corso_con_la_quota_predefinita() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = nuovo_iscritto(&connessione);

    let proposta = proponi_iscrizione(&connessione, Some(iscritto), OGGI).unwrap();

    assert_eq!(proposta.importo_cent, 3000);
    assert_eq!(proposta.data_inizio, data("2026-09-01"));
    assert_eq!(proposta.data_fine, data("2027-08-31"));
}

#[test]
fn il_rinnovo_con_iscrizione_valida_propone_l_anno_sportivo_successivo() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = nuovo_iscritto(&connessione);
    iscrivi_come_proposto(&connessione, iscritto, OGGI);

    let proposta = proponi_iscrizione(&connessione, Some(iscritto), data("2027-08-20")).unwrap();

    assert_eq!(proposta.data_inizio, data("2027-09-01"));
    assert_eq!(proposta.data_fine, data("2028-08-31"));
}

#[test]
fn quota_e_mese_di_inizio_vengono_dalle_impostazioni() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    connessione
        .execute_batch(
            "UPDATE impostazioni SET valore = '1' WHERE chiave = 'mese_inizio_anno_sportivo';
             UPDATE impostazioni SET valore = '2500' WHERE chiave = 'quota_iscrizione_cent';",
        )
        .unwrap();
    let iscritto = nuovo_iscritto(&connessione);

    let proposta = proponi_iscrizione(&connessione, Some(iscritto), OGGI).unwrap();

    assert_eq!(proposta.importo_cent, 2500);
    assert_eq!(proposta.data_inizio, data("2026-01-01"));
    assert_eq!(proposta.data_fine, data("2026-12-31"));
}

#[test]
fn iscrizione_e_abbonamento_hanno_stati_separati() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = nuovo_iscritto(&connessione);

    iscrivi_come_proposto(&connessione, iscritto, OGGI);

    let aggiornato = trova_iscritto(&connessione, iscritto, RIFERIMENTO).unwrap();
    assert_eq!(aggiornato.stato_iscrizione, StatoValidita::Attivo);
    assert_eq!(
        aggiornato.ultima_data_fine_iscrizione,
        Some(data("2027-08-31"))
    );
    assert_eq!(aggiornato.stato_abbonamento, StatoValidita::Nessuno);
}

#[test]
fn lo_storico_delle_iscrizioni_e_dal_piu_recente() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = nuovo_iscritto(&connessione);
    iscrivi_come_proposto(&connessione, iscritto, data("2025-10-01"));
    iscrivi_come_proposto(&connessione, iscritto, OGGI);

    let storico = elenca_iscrizioni(&connessione, iscritto).unwrap();

    assert_eq!(storico.len(), 2);
    assert_eq!(storico[0].data_inizio, data("2026-09-01"));
    assert_eq!(storico[1].data_inizio, data("2025-09-01"));
}

#[test]
fn dati_non_validi_e_iscritto_inesistente_sono_rifiutati() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = nuovo_iscritto(&connessione);
    let base = DatiIscrizione {
        iscritto_id: iscritto,
        importo_cent: 3000,
        data_pagamento: OGGI,
        metodo: MetodoPagamento::Carta,
        data_inizio: data("2026-09-01"),
        data_fine: data("2027-08-31"),
        note: None,
    };
    for dati in [
        DatiIscrizione {
            importo_cent: -1,
            ..base.clone()
        },
        DatiIscrizione {
            data_fine: data("2026-08-31"),
            ..base.clone()
        },
        DatiIscrizione {
            iscritto_id: 999,
            ..base
        },
    ] {
        assert!(matches!(
            registra_iscrizione(&connessione, &dati),
            Err(ErroreDatabase::DatiNonValidi(_))
        ));
    }
}

#[test]
fn si_elimina_un_iscrizione_registrata_per_errore() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = nuovo_iscritto(&connessione);
    let id = iscrivi_come_proposto(&connessione, iscritto, OGGI);

    elimina_iscrizione(&connessione, id).unwrap();

    assert!(
        elenca_iscrizioni(&connessione, iscritto)
            .unwrap()
            .is_empty()
    );
    assert!(matches!(
        elimina_iscrizione(&connessione, id),
        Err(ErroreDatabase::NonTrovato)
    ));
}

#[test]
fn il_filtro_per_stato_iscrizione_coincide_con_lo_stato_calcolato() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    genera_dati_prova(&mut connessione, 250, OGGI, 21).unwrap();
    let tutti = elenca_iscritti(&connessione, &FiltroIscritti::default(), RIFERIMENTO).unwrap();

    for stato in [
        StatoValidita::Attivo,
        StatoValidita::Scaduto,
        StatoValidita::Nessuno,
    ] {
        let filtro = FiltroIscritti {
            stato_iscrizione: Some(stato),
            ..Default::default()
        };
        let pagina = elenca_iscritti_paginati(
            &connessione,
            &filtro,
            RIFERIMENTO,
            RichiestaPagina::default(),
        )
        .unwrap();
        let attesi = tutti.iter().filter(|i| i.stato_iscrizione == stato).count();

        assert!(
            attesi > 0,
            "nessun iscritto con iscrizione {stato:?} nei dati di prova"
        );
        assert_eq!(
            pagina.totale_elementi as usize, attesi,
            "conteggio {stato:?}"
        );
        assert!(pagina.elementi.iter().all(|i| i.stato_iscrizione == stato));
    }
}

#[test]
fn la_migrazione_conserva_gli_iscritti_esistenti_e_fa_un_backup_prima() {
    let cartella = tempfile::tempdir().unwrap();
    let percorso = cartella.path().join("palestra.db");
    {
        // Database creato con la prima versione dello schema
        let mut connessione = crea_database_criptato(&percorso, PASSWORD_TEST).unwrap();
        applica_migrazioni_fino_a(&mut connessione, 1).unwrap();
        connessione
            .execute(
                "INSERT INTO iscritti (nome, cognome, telefono, data_iscrizione)
                 VALUES ('Luca', 'Verdi', '+39 333 0000000', '2025-02-01')",
                [],
            )
            .unwrap();
    }

    let stato = StatoApp::nuovo(percorso.clone());
    stato
        .sblocca_database(Zeroizing::new(PASSWORD_TEST.to_string()))
        .unwrap();

    let connessione = apri_database_criptato(&percorso, PASSWORD_TEST).unwrap();
    assert_eq!(
        versione_schema_corrente(&connessione).unwrap(),
        versione_schema_supportata()
    );
    let (nome, telefono, data_nascita): (String, String, Option<String>) = connessione
        .query_row(
            "SELECT nome, telefono, data_nascita FROM iscritti",
            [],
            |riga| Ok((riga.get(0)?, riga.get(1)?, riga.get(2)?)),
        )
        .unwrap();
    assert_eq!(
        (nome.as_str(), telefono.as_str(), data_nascita),
        ("Luca", "+39 333 0000000", None)
    );

    let backup_pre_migrazione = std::fs::read_dir(stato.cartella_backup())
        .unwrap()
        .filter(|voce| {
            voce.as_ref()
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with("pre-migrazione-v1-")
        })
        .count();
    assert_eq!(backup_pre_migrazione, 1);
}
