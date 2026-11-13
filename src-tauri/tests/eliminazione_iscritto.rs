mod comune;

use chrono::NaiveDate;
use comune::DatabaseTemporaneo;
use rusqlite::Connection;
use sp_gestionale_lib::db::ErroreDatabase;
use sp_gestionale_lib::db::eliminazione::elimina_dati_iscritto;
use sp_gestionale_lib::db::iscritti::{
    DatiIscritto, FiltroIscritti, Riferimento, crea_iscritto, elenca_iscritti,
    imposta_iscritto_archiviato, modifica_iscritto, trova_iscritto,
};
use sp_gestionale_lib::db::pagamenti::{FiltroPagamenti, elenca_pagamenti};
use sp_gestionale_lib::db::report::{Raggruppamento, riepilogo_incassi};

const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const RIFERIMENTO: Riferimento = Riferimento {
    oggi: OGGI,
    giorni_preavviso: 7,
};

fn dati_anagrafici() -> DatiIscritto {
    DatiIscritto {
        nome: "Giulia".into(),
        cognome: "Esposito".into(),
        data_nascita: "1995-04-12".parse().unwrap(),
        luogo_nascita: "Bari".into(),
        telefono: Some("333 1234567".into()),
        email: Some("giulia@esempio.it".into()),
        note: Some("Allergica al lattice".into()),
        data_scadenza_certificato: Some("2027-03-01".parse().unwrap()),
        certificato_presentato: false,
    }
}

/// Iscritto con un abbonamento, la quota annuale, un promemoria inviato; archiviato o no.
fn iscritto_con_movimenti(connessione: &Connection, archiviato: bool) -> i64 {
    let id = crea_iscritto(connessione, &dati_anagrafici(), RIFERIMENTO)
        .unwrap()
        .id;
    connessione
        .execute_batch(&format!(
            "INSERT INTO pagamenti (iscritto_id, tipologia_nome, importo_cent, data_pagamento, metodo, data_inizio, data_scadenza, note)
             VALUES ({id}, 'Mensile', 5000, '2026-11-01', 'contanti', '2026-11-01', '2026-11-30', 'Pagato dalla madre');
             INSERT INTO iscrizioni (iscritto_id, importo_cent, data_pagamento, metodo, data_inizio, data_fine, note)
             VALUES ({id}, 3000, '2026-11-01', 'carta', '2026-09-01', '2027-08-31', 'Sconto famiglia');
             INSERT INTO promemoria_inviati (iscritto_id, tipo, data_scadenza)
             VALUES ({id}, 'abbonamento', '2026-11-30');"
        ))
        .unwrap();
    if archiviato {
        imposta_iscritto_archiviato(connessione, id, true, RIFERIMENTO).unwrap();
    }
    id
}

fn totale_incassi(connessione: &Connection) -> i64 {
    riepilogo_incassi(
        connessione,
        "2026-01-01".parse().unwrap(),
        "2026-12-31".parse().unwrap(),
        Raggruppamento::Mese,
    )
    .unwrap()
    .totale
    .totale_cent
}

#[test]
fn si_puo_eliminare_solo_un_iscritto_archiviato() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let id = iscritto_con_movimenti(&connessione, false);

    let errore = elimina_dati_iscritto(&mut connessione, id, OGGI).unwrap_err();

    assert!(matches!(errore, ErroreDatabase::DatiNonValidi(_)));
    assert_eq!(
        trova_iscritto(&connessione, id, RIFERIMENTO).unwrap().nome,
        "Giulia"
    );
}

#[test]
fn l_anagrafica_e_le_note_vengono_cancellate() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let id = iscritto_con_movimenti(&connessione, true);

    elimina_dati_iscritto(&mut connessione, id, OGGI).unwrap();

    // coalesce restituisce il primo dato personale rimasto, se ce n'è uno
    let riga: (String, String, Option<String>, String) = connessione
        .query_row(
            "SELECT nome, cognome,
                    coalesce(data_nascita, luogo_nascita, telefono, email, note,
                             data_scadenza_certificato),
                    data_eliminazione
             FROM iscritti WHERE id = ?1",
            [id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )
        .unwrap();
    assert_eq!(
        riga,
        (
            "Iscritto".into(),
            "eliminato".into(),
            None,
            "2026-11-10".into()
        )
    );
    let note_rimaste: i64 = connessione
        .query_row(
            "SELECT (SELECT count(note) FROM pagamenti) + (SELECT count(note) FROM iscrizioni)
                    + (SELECT count(*) FROM promemoria_inviati)",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(note_rimaste, 0);
}

#[test]
fn gli_incassi_restano_invariati() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let id = iscritto_con_movimenti(&connessione, true);
    let prima = totale_incassi(&connessione);

    elimina_dati_iscritto(&mut connessione, id, OGGI).unwrap();

    assert_eq!(prima, 8000);
    assert_eq!(totale_incassi(&connessione), prima);
}

#[test]
fn i_pagamenti_restano_segnati_come_di_un_iscritto_eliminato() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let id = iscritto_con_movimenti(&connessione, true);

    elimina_dati_iscritto(&mut connessione, id, OGGI).unwrap();

    let pagamenti = elenca_pagamenti(&connessione, &FiltroPagamenti::default()).unwrap();
    assert_eq!(pagamenti.len(), 1);
    assert!(pagamenti[0].iscritto_eliminato);
    assert_eq!(pagamenti[0].note, None);
}

#[test]
fn l_iscritto_eliminato_sparisce_e_non_si_puo_piu_modificare() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let id = iscritto_con_movimenti(&connessione, true);
    let altro = iscritto_con_movimenti(&connessione, true);

    elimina_dati_iscritto(&mut connessione, id, OGGI).unwrap();

    let filtro = FiltroIscritti {
        includi_archiviati: true,
        ..Default::default()
    };
    let rimasti: Vec<i64> = elenca_iscritti(&connessione, &filtro, RIFERIMENTO)
        .unwrap()
        .iter()
        .map(|iscritto| iscritto.id)
        .collect();
    assert_eq!(rimasti, vec![altro]);
    assert!(matches!(
        trova_iscritto(&connessione, id, RIFERIMENTO),
        Err(ErroreDatabase::NonTrovato)
    ));
    assert!(matches!(
        modifica_iscritto(&connessione, id, &dati_anagrafici(), RIFERIMENTO),
        Err(ErroreDatabase::NonTrovato)
    ));
    assert!(matches!(
        imposta_iscritto_archiviato(&connessione, id, false, RIFERIMENTO),
        Err(ErroreDatabase::NonTrovato)
    ));
    assert!(matches!(
        elimina_dati_iscritto(&mut connessione, id, OGGI),
        Err(ErroreDatabase::NonTrovato)
    ));
}
