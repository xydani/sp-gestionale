mod comune;

use chrono::NaiveDate;
use comune::DatabaseTemporaneo;
use rusqlite::Connection;
use sp_gestionale_lib::db::iscritti::{DatiIscritto, Riferimento};
use sp_gestionale_lib::db::iscrizioni::{DatiIscrizione, proponi_iscrizione};
use sp_gestionale_lib::db::nuovo_iscritto::{DatiNuovoIscritto, iscrivi_nuovo_iscritto};
use sp_gestionale_lib::db::pagamenti::{DatiPagamento, proponi_pagamento};
use sp_gestionale_lib::domain::pagamento::MetodoPagamento;
use sp_gestionale_lib::domain::stato::StatoValidita;

const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const RIFERIMENTO: Riferimento = Riferimento {
    oggi: OGGI,
    giorni_preavviso: 7,
};
const MENSILE: i64 = 1;
/// Valore qualsiasi: l'id viene sostituito con quello dell'iscritto appena creato.
const ID_IGNORATO: i64 = 0;

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

fn dati_completi(connessione: &Connection) -> DatiNuovoIscritto {
    let iscrizione = proponi_iscrizione(connessione, None, OGGI).unwrap();
    let pagamento = proponi_pagamento(connessione, None, MENSILE, None, OGGI).unwrap();
    DatiNuovoIscritto {
        anagrafica: DatiIscritto {
            nome: "Giulia".into(),
            cognome: "Esposito".into(),
            data_nascita: Some(data("1995-04-12")),
            luogo_nascita: Some("Bari".into()),
            telefono: None,
            email: None,
            note: None,
            data_scadenza_certificato: None,
            certificato_presentato: false,
        },
        iscrizione: DatiIscrizione {
            iscritto_id: ID_IGNORATO,
            importo_cent: iscrizione.importo_cent,
            data_pagamento: OGGI,
            metodo: MetodoPagamento::Contanti,
            data_inizio: iscrizione.data_inizio,
            data_fine: iscrizione.data_fine,
            note: None,
        },
        pagamento: DatiPagamento {
            iscritto_id: ID_IGNORATO,
            tipologia_id: MENSILE,
            importo_cent: pagamento.importo_cent,
            data_pagamento: OGGI,
            metodo: MetodoPagamento::Carta,
            data_inizio: pagamento.data_inizio,
            data_scadenza: pagamento.data_scadenza,
            note: None,
            rinnovo_di_id: None,
        },
    }
}

#[test]
fn senza_iscritto_si_propongono_anno_in_corso_e_inizio_da_oggi() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let iscrizione = proponi_iscrizione(&connessione, None, OGGI).unwrap();
    assert_eq!(
        (iscrizione.data_inizio, iscrizione.data_fine),
        (data("2026-09-01"), data("2027-08-31"))
    );
    let pagamento = proponi_pagamento(&connessione, None, MENSILE, None, OGGI).unwrap();
    assert_eq!(pagamento.data_inizio, OGGI);
    assert_eq!(pagamento.rinnovo_di_id, None);
}

#[test]
fn il_nuovo_iscritto_ha_subito_iscrizione_e_abbonamento_attivi() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let dati = dati_completi(&connessione);
    let iscritto = iscrivi_nuovo_iscritto(&mut connessione, &dati, RIFERIMENTO).unwrap();

    assert_eq!(iscritto.cognome, "Esposito");
    assert_eq!(iscritto.stato_iscrizione, StatoValidita::Attivo);
    assert_eq!(iscritto.stato_abbonamento, StatoValidita::Attivo);
    let collegati: i64 = connessione
        .query_row(
            "SELECT (SELECT count(*) FROM iscrizioni WHERE iscritto_id = ?1)
                  + (SELECT count(*) FROM pagamenti WHERE iscritto_id = ?1)",
            [iscritto.id],
            |riga| riga.get(0),
        )
        .unwrap();
    assert_eq!(collegati, 2);
}

#[test]
fn con_un_dato_sbagliato_non_si_salva_nulla() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let mut dati = dati_completi(&connessione);
    dati.pagamento.importo_cent = -100;

    assert!(iscrivi_nuovo_iscritto(&mut connessione, &dati, RIFERIMENTO).is_err());
    assert_eq!(conta(&connessione, "iscritti"), 0);
    assert_eq!(conta(&connessione, "iscrizioni"), 0);
    assert_eq!(conta(&connessione, "pagamenti"), 0);
}

#[test]
fn con_un_abbonamento_inesistente_non_resta_l_iscritto() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let mut dati = dati_completi(&connessione);
    dati.pagamento.tipologia_id = 999;

    assert!(iscrivi_nuovo_iscritto(&mut connessione, &dati, RIFERIMENTO).is_err());
    assert_eq!(conta(&connessione, "iscritti"), 0);
}
