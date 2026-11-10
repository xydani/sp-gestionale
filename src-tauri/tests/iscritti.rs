mod comune;

use std::time::{Duration, Instant};

use chrono::NaiveDate;
use comune::DatabaseTemporaneo;
use rusqlite::Connection;
use sp_gestionale_lib::dati_prova::genera_dati_prova;
use sp_gestionale_lib::db::ErroreDatabase;
use sp_gestionale_lib::db::iscritti::{
    DatiIscritto, FiltroIscritti, Riferimento, crea_iscritto, elenca_iscritti,
    elenca_iscritti_paginati, imposta_iscritto_archiviato, modifica_iscritto, riferimento_per_oggi,
    trova_iscritto,
};
use sp_gestionale_lib::db::paginazione::RichiestaPagina;
use sp_gestionale_lib::domain::stato::StatoValidita;

const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const RIFERIMENTO: Riferimento = Riferimento {
    oggi: OGGI,
    giorni_preavviso: 7,
};
const TEMPO_MASSIMO_RICERCA: Duration = Duration::from_millis(100);

fn dati(nome: &str, cognome: &str) -> DatiIscritto {
    DatiIscritto {
        nome: nome.into(),
        cognome: cognome.into(),
        data_nascita: NaiveDate::from_ymd_opt(1990, 5, 20).unwrap(),
        luogo_nascita: "Roma".into(),
        telefono: Some("+39 333 1234567".into()),
        email: None,
        note: None,
    }
}

fn cerca(connessione: &Connection, testo: &str) -> Vec<String> {
    let filtro = FiltroIscritti {
        testo: testo.into(),
        ..Default::default()
    };
    elenca_iscritti(connessione, &filtro, RIFERIMENTO)
        .unwrap()
        .into_iter()
        .map(|iscritto| format!("{} {}", iscritto.nome, iscritto.cognome))
        .collect()
}

fn aggiungi_pagamento(connessione: &Connection, iscritto_id: i64, inizio: &str, scadenza: &str) {
    connessione
        .execute(
            "INSERT INTO pagamenti (iscritto_id, tipologia_nome, importo_cent, data_pagamento, metodo, data_inizio, data_scadenza)
             VALUES (?1, 'Mensile', 5000, ?2, 'contanti', ?2, ?3)",
            rusqlite::params![iscritto_id, inizio, scadenza],
        )
        .unwrap();
}

#[test]
fn si_crea_e_si_ritrova_un_iscritto() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();

    let creato = crea_iscritto(&connessione, &dati(" Mario ", "Rossi"), RIFERIMENTO).unwrap();

    assert_eq!(creato.nome, "Mario");
    assert_eq!(creato.stato_abbonamento, StatoValidita::Nessuno);
    assert!(!creato.archiviato);
    assert_eq!(
        trova_iscritto(&connessione, creato.id, RIFERIMENTO).unwrap(),
        creato
    );
}

#[test]
fn trovare_un_iscritto_inesistente_e_un_errore() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    assert!(matches!(
        trova_iscritto(&connessione, 42, RIFERIMENTO),
        Err(ErroreDatabase::NonTrovato)
    ));
}

#[test]
fn si_modificano_i_dati_di_un_iscritto() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let creato = crea_iscritto(&connessione, &dati("Mario", "Rossi"), RIFERIMENTO).unwrap();

    let mut nuovi = dati("Mario", "Rossi");
    nuovi.telefono = None;
    nuovi.note = Some("Preferisce la sera".into());
    let modificato = modifica_iscritto(&connessione, creato.id, &nuovi, RIFERIMENTO).unwrap();

    assert_eq!(modificato.telefono, None);
    assert_eq!(modificato.note.as_deref(), Some("Preferisce la sera"));
}

#[test]
fn la_ricerca_ignora_maiuscole_accenti_e_ordine_delle_parole() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    crea_iscritto(&connessione, &dati("Nicolò", "D'Angelo"), RIFERIMENTO).unwrap();
    crea_iscritto(&connessione, &dati("Mario", "Rossi"), RIFERIMENTO).unwrap();
    crea_iscritto(&connessione, &dati("Maria", "Rossini"), RIFERIMENTO).unwrap();

    assert_eq!(cerca(&connessione, "nicolo"), ["Nicolò D'Angelo"]);
    assert_eq!(cerca(&connessione, "D'ANG"), ["Nicolò D'Angelo"]);
    assert_eq!(cerca(&connessione, "rossi mario"), ["Mario Rossi"]);
    assert_eq!(
        cerca(&connessione, "ross"),
        ["Mario Rossi", "Maria Rossini"]
    );
    assert_eq!(cerca(&connessione, "  "), cerca(&connessione, ""));
    assert!(cerca(&connessione, "bianchi").is_empty());
}

#[test]
fn la_ricerca_tratta_percentuale_e_trattino_basso_come_testo() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    crea_iscritto(&connessione, &dati("Mario", "Rossi"), RIFERIMENTO).unwrap();
    assert!(cerca(&connessione, "%").is_empty());
    assert!(cerca(&connessione, "_").is_empty());
}

#[test]
fn gli_archiviati_sono_esclusi_salvo_richiesta() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let creato = crea_iscritto(&connessione, &dati("Mario", "Rossi"), RIFERIMENTO).unwrap();

    imposta_iscritto_archiviato(&connessione, creato.id, true, RIFERIMENTO).unwrap();

    assert!(cerca(&connessione, "").is_empty());
    let filtro = FiltroIscritti {
        includi_archiviati: true,
        ..Default::default()
    };
    let tutti = elenca_iscritti(&connessione, &filtro, RIFERIMENTO).unwrap();
    assert_eq!(tutti.len(), 1);
    assert!(tutti[0].archiviato);
}

#[test]
fn lo_stato_dipende_dall_ultima_scadenza() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let scaduto = crea_iscritto(&connessione, &dati("Anna", "Scaduta"), RIFERIMENTO).unwrap();
    let in_scadenza =
        crea_iscritto(&connessione, &dati("Bruno", "Inscadenza"), RIFERIMENTO).unwrap();
    let attivo = crea_iscritto(&connessione, &dati("Carla", "Attiva"), RIFERIMENTO).unwrap();
    aggiungi_pagamento(&connessione, scaduto.id, "2026-10-01", "2026-10-31");
    aggiungi_pagamento(&connessione, in_scadenza.id, "2026-10-15", "2026-11-14");
    aggiungi_pagamento(&connessione, attivo.id, "2026-10-01", "2026-10-31");
    aggiungi_pagamento(&connessione, attivo.id, "2026-11-01", "2026-12-31");

    let stato_di = |id| {
        trova_iscritto(&connessione, id, RIFERIMENTO)
            .unwrap()
            .stato_abbonamento
    };
    assert_eq!(stato_di(scaduto.id), StatoValidita::Scaduto);
    assert_eq!(stato_di(in_scadenza.id), StatoValidita::InScadenza);
    assert_eq!(stato_di(attivo.id), StatoValidita::Attivo);

    let filtro = FiltroIscritti {
        stato_abbonamento: Some(StatoValidita::Scaduto),
        ..Default::default()
    };
    let scaduti = elenca_iscritti(&connessione, &filtro, RIFERIMENTO).unwrap();
    assert_eq!(
        scaduti.iter().map(|i| i.id).collect::<Vec<_>>(),
        [scaduto.id]
    );
}

#[test]
fn il_preavviso_viene_letto_dalle_impostazioni() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    assert_eq!(
        riferimento_per_oggi(&connessione, OGGI)
            .unwrap()
            .giorni_preavviso,
        7
    );

    connessione
        .execute(
            "UPDATE impostazioni SET valore = '14' WHERE chiave = 'giorni_preavviso'",
            [],
        )
        .unwrap();
    assert_eq!(
        riferimento_per_oggi(&connessione, OGGI)
            .unwrap()
            .giorni_preavviso,
        14
    );
}

#[test]
fn elenco_ordinato_per_cognome_e_nome() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    crea_iscritto(&connessione, &dati("Zeno", "Bianchi"), RIFERIMENTO).unwrap();
    crea_iscritto(&connessione, &dati("Anna", "Verdi"), RIFERIMENTO).unwrap();
    crea_iscritto(&connessione, &dati("Aldo", "bianchi"), RIFERIMENTO).unwrap();
    assert_eq!(
        cerca(&connessione, ""),
        ["Aldo bianchi", "Zeno Bianchi", "Anna Verdi"]
    );
}

#[test]
fn con_250_iscritti_ricerca_ed_elenco_sono_veloci() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    genera_dati_prova(&mut connessione, 250, OGGI, 11).unwrap();

    let inizio = Instant::now();
    let tutti = elenca_iscritti(&connessione, &FiltroIscritti::default(), RIFERIMENTO).unwrap();
    let durata_elenco = inizio.elapsed();

    let inizio = Instant::now();
    let trovati = cerca(&connessione, "ross");
    let durata_ricerca = inizio.elapsed();

    assert_eq!(tutti.len(), 250);
    assert!(!trovati.is_empty());
    assert!(
        trovati
            .iter()
            .all(|nome| nome.to_lowercase().contains("ross"))
    );
    assert!(
        durata_elenco < TEMPO_MASSIMO_RICERCA,
        "elenco in {durata_elenco:?}"
    );
    assert!(
        durata_ricerca < TEMPO_MASSIMO_RICERCA,
        "ricerca in {durata_ricerca:?}"
    );
}

#[test]
fn con_250_iscritti_le_pagine_sono_da_50() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    genera_dati_prova(&mut connessione, 250, OGGI, 11).unwrap();
    let filtro = FiltroIscritti::default();
    let tutti = elenca_iscritti(&connessione, &filtro, RIFERIMENTO).unwrap();

    let pagina = |numero| {
        elenca_iscritti_paginati(
            &connessione,
            &filtro,
            RIFERIMENTO,
            RichiestaPagina { numero },
        )
        .unwrap()
    };

    assert_eq!(pagina(1).elementi[..], tutti[..50]);
    assert_eq!(pagina(5).elementi[..], tutti[200..250]);
    assert!(pagina(6).elementi.is_empty());
    assert_eq!(pagina(1).totale_elementi, 250);
}

#[test]
fn il_filtro_per_stato_nel_database_coincide_con_lo_stato_calcolato() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    genera_dati_prova(&mut connessione, 250, OGGI, 11).unwrap();
    let tutti = elenca_iscritti(&connessione, &FiltroIscritti::default(), RIFERIMENTO).unwrap();

    for stato in [
        StatoValidita::Attivo,
        StatoValidita::InScadenza,
        StatoValidita::Scaduto,
        StatoValidita::Nessuno,
    ] {
        let filtro = FiltroIscritti {
            stato_abbonamento: Some(stato),
            ..Default::default()
        };
        let pagina = elenca_iscritti_paginati(
            &connessione,
            &filtro,
            RIFERIMENTO,
            RichiestaPagina::default(),
        )
        .unwrap();
        let attesi: Vec<_> = tutti
            .iter()
            .filter(|i| i.stato_abbonamento == stato)
            .collect();

        assert!(
            !attesi.is_empty(),
            "nessun iscritto {stato:?} nei dati di prova"
        );
        assert_eq!(
            pagina.totale_elementi as usize,
            attesi.len(),
            "conteggio {stato:?}"
        );
        assert!(pagina.elementi.iter().all(|i| i.stato_abbonamento == stato));
        let prima_pagina_attesa: Vec<_> = attesi.into_iter().take(50).cloned().collect();
        assert_eq!(pagina.elementi, prima_pagina_attesa);
    }
}
