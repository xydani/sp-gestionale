mod comune;

use chrono::{Days, NaiveDate};
use comune::{DatabaseTemporaneo, PASSWORD_TEST};
use rusqlite::Connection;
use sp_gestionale_lib::db::crittografia::crea_database_criptato;
use sp_gestionale_lib::db::dashboard::riepilogo_dashboard;
use sp_gestionale_lib::db::iscritti::{
    DatiIscritto, FiltroIscritti, Riferimento, crea_iscritto, elenca_iscritti, modifica_iscritto,
};
use sp_gestionale_lib::db::migrazioni::{applica_migrazioni, applica_migrazioni_fino_a};
use sp_gestionale_lib::db::promemoria::{conta_in_scadenza, controlla_promemoria};
use sp_gestionale_lib::domain::stato::StatoValidita;

const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const RIFERIMENTO: Riferimento = Riferimento {
    oggi: OGGI,
    giorni_preavviso: 7,
};

fn dati(cognome: &str, scadenza_certificato: Option<NaiveDate>) -> DatiIscritto {
    DatiIscritto {
        nome: "Prova".into(),
        cognome: cognome.into(),
        data_nascita: "1990-01-01".parse().unwrap(),
        luogo_nascita: "Roma".into(),
        telefono: None,
        email: None,
        note: None,
        data_scadenza_certificato: scadenza_certificato,
        certificato_presentato: false,
    }
}

fn tra_giorni(giorni: i64) -> Option<NaiveDate> {
    Some(OGGI + chrono::Duration::days(giorni))
}

/// Quattro iscritti, uno per stato del certificato: valido, in scadenza, scaduto, mancante.
fn iscritti_con_ogni_stato(connessione: &Connection) {
    for (cognome, scadenza) in [
        ("Valido", tra_giorni(200)),
        ("InScadenza", tra_giorni(3)),
        ("Scaduto", tra_giorni(-10)),
        ("Mancante", None),
    ] {
        crea_iscritto(connessione, &dati(cognome, scadenza), RIFERIMENTO).unwrap();
    }
}

#[test]
fn lo_stato_del_certificato_segue_la_scadenza() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    iscritti_con_ogni_stato(&connessione);
    let stati: Vec<(String, StatoValidita)> =
        elenca_iscritti(&connessione, &FiltroIscritti::default(), RIFERIMENTO)
            .unwrap()
            .into_iter()
            .map(|iscritto| (iscritto.cognome, iscritto.stato_certificato))
            .collect();
    assert!(stati.contains(&("Valido".into(), StatoValidita::Attivo)));
    assert!(stati.contains(&("InScadenza".into(), StatoValidita::InScadenza)));
    assert!(stati.contains(&("Scaduto".into(), StatoValidita::Scaduto)));
    assert!(stati.contains(&("Mancante".into(), StatoValidita::Nessuno)));
}

#[test]
fn si_filtrano_gli_iscritti_per_stato_del_certificato() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    iscritti_con_ogni_stato(&connessione);
    let filtro = FiltroIscritti {
        stato_certificato: Some(StatoValidita::Nessuno),
        ..FiltroIscritti::default()
    };
    let senza = elenca_iscritti(&connessione, &filtro, RIFERIMENTO).unwrap();
    assert_eq!(senza.len(), 1);
    assert_eq!(senza[0].cognome, "Mancante");
}

#[test]
fn la_modifica_registra_il_certificato_presentato() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = crea_iscritto(&connessione, &dati("Rossi", None), RIFERIMENTO).unwrap();
    assert_eq!(iscritto.stato_certificato, StatoValidita::Nessuno);
    let scadenza = OGGI + Days::new(365);
    let modificato = modifica_iscritto(
        &connessione,
        iscritto.id,
        &dati("Rossi", Some(scadenza)),
        RIFERIMENTO,
    )
    .unwrap();
    assert_eq!(modificato.data_scadenza_certificato, Some(scadenza));
    assert_eq!(modificato.stato_certificato, StatoValidita::Attivo);
}

#[test]
fn i_certificati_in_scadenza_si_notificano_una_volta() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    iscritti_con_ogni_stato(&connessione);
    assert_eq!(
        conta_in_scadenza(&connessione, RIFERIMENTO)
            .unwrap()
            .certificati,
        1
    );
    let primo = controlla_promemoria(&mut connessione, RIFERIMENTO).unwrap();
    assert_eq!(primo.nuovi.certificati, 1);
    let secondo = controlla_promemoria(&mut connessione, RIFERIMENTO).unwrap();
    assert_eq!(secondo.nuovi.certificati, 0);
}

#[test]
fn la_dashboard_conta_i_certificati_per_stato() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    iscritti_con_ogni_stato(&connessione);
    let riepilogo = riepilogo_dashboard(&connessione, RIFERIMENTO).unwrap();
    let certificati = riepilogo.certificati;
    assert_eq!(
        (
            certificati.attivi,
            certificati.in_scadenza,
            certificati.scaduti,
            certificati.assenti
        ),
        (1, 1, 1, 1)
    );
    assert!(
        riepilogo
            .prossime_scadenze
            .iter()
            .any(|scadenza| scadenza.tipo == "certificato" && scadenza.cognome == "InScadenza")
    );
}

#[test]
fn la_migrazione_conserva_i_promemoria_gia_inviati() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = crea_database_criptato(&database.percorso, PASSWORD_TEST).unwrap();
    applica_migrazioni_fino_a(&mut connessione, 4).unwrap();
    connessione
        .execute_batch(
            "INSERT INTO iscritti (nome, cognome, data_nascita, luogo_nascita)
                 VALUES ('Anna', 'Neri', '1990-01-01', 'Roma');
             INSERT INTO promemoria_inviati (tipo, iscritto_id, data_scadenza)
                 VALUES ('abbonamento', 1, '2026-11-12');",
        )
        .unwrap();
    applica_migrazioni(&mut connessione).unwrap();

    let promemoria: i64 = connessione
        .query_row("SELECT count(*) FROM promemoria_inviati", [], |riga| {
            riga.get(0)
        })
        .unwrap();
    assert_eq!(promemoria, 1);
    let certificato: Option<String> = connessione
        .query_row(
            "SELECT data_scadenza_certificato FROM iscritti WHERE id = 1",
            [],
            |riga| riga.get(0),
        )
        .unwrap();
    assert_eq!(certificato, None);
}

fn presentato_senza_scadenza(cognome: &str) -> DatiIscritto {
    DatiIscritto {
        certificato_presentato: true,
        ..dati(cognome, None)
    }
}

#[test]
fn un_certificato_senza_scadenza_conta_come_presentato_e_valido() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let iscritto = crea_iscritto(
        &connessione,
        &presentato_senza_scadenza("SenzaData"),
        RIFERIMENTO,
    )
    .unwrap();

    assert!(iscritto.certificato_presentato);
    assert_eq!(iscritto.data_scadenza_certificato, None);
    assert_eq!(iscritto.stato_certificato, StatoValidita::Attivo);

    let mancanti = FiltroIscritti {
        stato_certificato: Some(StatoValidita::Nessuno),
        ..Default::default()
    };
    assert!(
        elenca_iscritti(&connessione, &mancanti, RIFERIMENTO)
            .unwrap()
            .is_empty()
    );
    let certificati = riepilogo_dashboard(&connessione, RIFERIMENTO)
        .unwrap()
        .certificati;
    assert_eq!((certificati.attivi, certificati.assenti), (1, 0));
    let promemoria = controlla_promemoria(&mut connessione, RIFERIMENTO).unwrap();
    assert_eq!(promemoria.nuovi.certificati, 0);
}

#[test]
fn togliendo_la_spunta_il_certificato_torna_mancante() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscritto = crea_iscritto(
        &connessione,
        &presentato_senza_scadenza("SenzaData"),
        RIFERIMENTO,
    )
    .unwrap();

    let modificato = modifica_iscritto(
        &connessione,
        iscritto.id,
        &dati("SenzaData", None),
        RIFERIMENTO,
    )
    .unwrap();

    assert!(!modificato.certificato_presentato);
    assert_eq!(modificato.stato_certificato, StatoValidita::Nessuno);
}

#[test]
fn con_una_data_il_certificato_segue_la_sua_scadenza() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let scaduto = DatiIscritto {
        certificato_presentato: true,
        ..dati("ConData", tra_giorni(-1))
    };

    let iscritto = crea_iscritto(&connessione, &scaduto, RIFERIMENTO).unwrap();

    assert!(iscritto.certificato_presentato);
    assert_eq!(iscritto.stato_certificato, StatoValidita::Scaduto);
}
