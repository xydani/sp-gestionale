//! I controlli di plausibilità valgono anche passando dal database, non solo nel dominio.

mod comune;

use chrono::NaiveDate;
use comune::DatabaseTemporaneo;
use rusqlite::Connection;
use sp_gestionale_lib::db::ErroreDatabase;
use sp_gestionale_lib::db::iscritti::{DatiIscritto, Riferimento, crea_iscritto};
use sp_gestionale_lib::db::iscrizioni::{DatiIscrizione, registra_iscrizione};
use sp_gestionale_lib::db::pagamenti::{DatiPagamento, registra_pagamento};
use sp_gestionale_lib::db::tipologie::{DatiTipologia, crea_tipologia};
use sp_gestionale_lib::domain::date::UnitaDurata;
use sp_gestionale_lib::domain::pagamento::MetodoPagamento;

const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const RIFERIMENTO: Riferimento = Riferimento {
    oggi: OGGI,
    giorni_preavviso: 7,
};
const MENSILE: i64 = 1;

/// Una modifica che rende i dati non validi e una parola attesa nel messaggio d'errore.
type Variante<T> = (fn(&mut T), &'static str);

fn data(testo: &str) -> NaiveDate {
    testo.parse().unwrap()
}

fn anagrafica() -> DatiIscritto {
    DatiIscritto {
        nome: "Mario".into(),
        cognome: "Rossi".into(),
        data_nascita: data("1990-05-20"),
        luogo_nascita: "Roma".into(),
        telefono: Some("+39 333 1234567".into()),
        email: Some("mario.rossi@esempio.it".into()),
        note: None,
        data_scadenza_certificato: Some(data("2027-06-30")),
    }
}

fn pagamento(iscritto_id: i64) -> DatiPagamento {
    DatiPagamento {
        iscritto_id,
        tipologia_id: MENSILE,
        importo_cent: 5000,
        data_pagamento: OGGI,
        metodo: MetodoPagamento::Contanti,
        data_inizio: OGGI,
        data_scadenza: data("2026-12-09"),
        note: None,
        rinnovo_di_id: None,
    }
}

fn iscrizione(iscritto_id: i64) -> DatiIscrizione {
    DatiIscrizione {
        iscritto_id,
        importo_cent: 3000,
        data_pagamento: OGGI,
        metodo: MetodoPagamento::Carta,
        data_inizio: data("2026-09-01"),
        data_fine: data("2027-08-31"),
        note: None,
    }
}

fn messaggio<T: std::fmt::Debug>(esito: Result<T, ErroreDatabase>) -> String {
    match esito.unwrap_err() {
        ErroreDatabase::DatiNonValidi(messaggio) => messaggio,
        altro => panic!("errore inatteso: {altro:?}"),
    }
}

fn con_iscritto() -> (DatabaseTemporaneo, Connection, i64) {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let id = crea_iscritto(&connessione, &anagrafica(), RIFERIMENTO)
        .unwrap()
        .id;
    (database, connessione, id)
}

#[test]
fn l_anagrafica_rifiuta_dati_non_plausibili() {
    let (_database, connessione, _) = con_iscritto();
    let varianti: [Variante<DatiIscritto>; 6] = [
        (|d| d.luogo_nascita = "123".into(), "luogo di nascita"),
        (|d| d.nome = "Mario2".into(), "nome"),
        (|d| d.data_nascita = data("2026-11-11"), "futuro"),
        (|d| d.telefono = Some("telefono".into()), "telefono"),
        (|d| d.email = Some("mario@esempio".into()), "email"),
        (
            |d| d.data_scadenza_certificato = Some(data("2029-01-01")),
            "certificato",
        ),
    ];
    for (modifica, parola_attesa) in varianti {
        let mut dati = anagrafica();
        modifica(&mut dati);
        let testo = messaggio(crea_iscritto(&connessione, &dati, RIFERIMENTO));
        assert!(testo.contains(parola_attesa), "{testo}");
    }
}

#[test]
fn il_pagamento_rifiuta_date_e_importi_non_plausibili() {
    let (_database, connessione, id) = con_iscritto();
    let varianti: [Variante<DatiPagamento>; 4] = [
        (|p| p.data_pagamento = data("2026-11-11"), "futuro"),
        (|p| p.importo_cent = 1_000_001, "troppo alto"),
        (|p| p.data_scadenza = data("2030-01-01"), "troppo lungo"),
        (|p| p.data_pagamento = data("2012-01-01"), "anni fa"),
    ];
    for (modifica, parola_attesa) in varianti {
        let mut dati = pagamento(id);
        modifica(&mut dati);
        let testo = messaggio(registra_pagamento(&connessione, &dati, OGGI));
        assert!(testo.contains(parola_attesa), "{testo}");
    }
    assert!(registra_pagamento(&connessione, &pagamento(id), OGGI).is_ok());
}

#[test]
fn l_iscrizione_copre_al_massimo_un_anno() {
    let (_database, connessione, id) = con_iscritto();
    let mut dati = iscrizione(id);
    dati.data_fine = data("2028-08-31");
    assert!(messaggio(registra_iscrizione(&connessione, &dati, OGGI)).contains("troppo lungo"));
    assert!(registra_iscrizione(&connessione, &iscrizione(id), OGGI).is_ok());
}

#[test]
fn il_listino_rifiuta_durate_e_prezzi_esagerati() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let tipologia = |durata_valore, durata_unita, prezzo_cent| DatiTipologia {
        nome: "Prova".into(),
        durata_valore,
        durata_unita,
        prezzo_cent,
    };
    assert!(crea_tipologia(&connessione, &tipologia(37, UnitaDurata::Mesi, 1000)).is_err());
    assert!(crea_tipologia(&connessione, &tipologia(1096, UnitaDurata::Giorni, 1000)).is_err());
    assert!(crea_tipologia(&connessione, &tipologia(1, UnitaDurata::Mesi, 1_000_001)).is_err());
    assert!(crea_tipologia(&connessione, &tipologia(36, UnitaDurata::Mesi, 1000)).is_ok());
}
