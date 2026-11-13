//! Ogni euro registrato, da qualunque punto del programma, deve comparire uguale ovunque
//! si leggano gli incassi: elenchi, riepilogo, dashboard, cassa di oggi, CSV e statistiche.

mod comune;

use chrono::NaiveDate;
use comune::DatabaseTemporaneo;
use rusqlite::Connection;
use sp_gestionale_lib::db::dashboard::{Incassi, riepilogo_dashboard};
use sp_gestionale_lib::db::iscritti::{DatiIscritto, Riferimento};
use sp_gestionale_lib::db::iscritto_gia_attivo::{
    DatiIscrittoGiaAttivo, registra_iscritto_gia_attivo,
};
use sp_gestionale_lib::db::iscrizioni::{DatiIscrizione, elenca_iscrizioni, proponi_iscrizione};
use sp_gestionale_lib::db::nuovo_iscritto::{DatiNuovoIscritto, iscrivi_nuovo_iscritto};
use sp_gestionale_lib::db::pagamenti::{
    DatiPagamento, FiltroPagamenti, elenca_pagamenti, proponi_pagamento, registra_pagamento,
};
use sp_gestionale_lib::db::report::{
    Raggruppamento, elenca_movimenti, esporta_incassi_csv, riepilogo_incassi,
};
use sp_gestionale_lib::db::statistiche::leggi_statistiche;
use sp_gestionale_lib::domain::pagamento::MetodoPagamento;

const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const RIFERIMENTO: Riferimento = Riferimento {
    oggi: OGGI,
    giorni_preavviso: 7,
};
const MENSILE: i64 = 1;
const INIZIO_ANNO_SPORTIVO: NaiveDate = NaiveDate::from_ymd_opt(2026, 9, 1).unwrap();
/// Un ID qualsiasi: viene sostituito con quello dell'iscritto appena creato.
const ID_IGNORATO: i64 = 0;

fn data(testo: &str) -> NaiveDate {
    testo.parse().unwrap()
}

/// Nuovo iscritto oggi: quota 30 € e primo mensile 50 €, in contanti.
fn iscrivi_persona_nuova(connessione: &mut Connection) -> i64 {
    let iscrizione = proponi_iscrizione(connessione, None, OGGI).unwrap();
    let pagamento = proponi_pagamento(connessione, None, MENSILE, None, OGGI).unwrap();
    let dati = DatiNuovoIscritto {
        anagrafica: DatiIscritto {
            nome: "Nuova".into(),
            cognome: "Persona".into(),
            data_nascita: None,
            luogo_nascita: None,
            telefono: None,
            email: None,
            note: None,
            data_scadenza_certificato: None,
            certificato_presentato: false,
        },
        iscrizione: DatiIscrizione {
            iscritto_id: ID_IGNORATO,
            importo_cent: 3000,
            data_pagamento: OGGI,
            metodo: MetodoPagamento::Contanti,
            data_inizio: iscrizione.data_inizio,
            data_fine: iscrizione.data_fine,
            note: None,
        },
        pagamento: DatiPagamento {
            iscritto_id: ID_IGNORATO,
            tipologia_id: MENSILE,
            importo_cent: 5000,
            data_pagamento: OGGI,
            metodo: MetodoPagamento::Contanti,
            data_inizio: pagamento.data_inizio,
            data_scadenza: pagamento.data_scadenza,
            note: None,
            rinnovo_di_id: None,
        },
    };
    iscrivi_nuovo_iscritto(connessione, &dati, RIFERIMENTO)
        .unwrap()
        .id
}

/// Già iscritto: mensile da 45 € che scade il 25/11 (quindi pagato il 26/10) e quota 30 €,
/// entrambi con bonifico.
fn inserisci_gia_iscritto(connessione: &mut Connection) -> i64 {
    let dati = DatiIscrittoGiaAttivo {
        nome: "Gia".into(),
        cognome: "Iscritta".into(),
        telefono: None,
        data_scadenza_certificato: None,
        certificato_presentato: false,
        tipologia_id: MENSILE,
        data_scadenza_abbonamento: data("2026-11-25"),
        importo_abbonamento_cent: 4500,
        quota_pagata: true,
        quota_cent: 3000,
        metodo: MetodoPagamento::Bonifico,
    };
    registra_iscritto_gia_attivo(connessione, &dati, RIFERIMENTO)
        .unwrap()
        .id
}

/// Rinnovo anticipato oggi del mensile della persona nuova, 50 € con carta.
fn rinnova(connessione: &Connection, iscritto_id: i64) {
    let proposta = proponi_pagamento(connessione, Some(iscritto_id), MENSILE, None, OGGI).unwrap();
    let dati = DatiPagamento {
        iscritto_id,
        tipologia_id: MENSILE,
        importo_cent: proposta.importo_cent,
        data_pagamento: OGGI,
        metodo: MetodoPagamento::Carta,
        data_inizio: proposta.data_inizio,
        data_scadenza: proposta.data_scadenza,
        note: None,
        rinnovo_di_id: proposta.rinnovo_di_id,
    };
    registra_pagamento(connessione, &dati, OGGI).unwrap();
}

fn incassi(abbonamenti_cent: i64, iscrizioni_cent: i64) -> Incassi {
    Incassi {
        abbonamenti_cent,
        iscrizioni_cent,
        totale_cent: abbonamenti_cent + iscrizioni_cent,
    }
}

struct Scenario {
    _database: DatabaseTemporaneo,
    connessione: Connection,
    persona_nuova: i64,
    gia_iscritta: i64,
}

fn scenario_con_tutti_i_pagamenti() -> Scenario {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let persona_nuova = iscrivi_persona_nuova(&mut connessione);
    let gia_iscritta = inserisci_gia_iscritto(&mut connessione);
    rinnova(&connessione, persona_nuova);
    Scenario {
        _database: database,
        connessione,
        persona_nuova,
        gia_iscritta,
    }
}

#[test]
fn ogni_pagamento_e_registrato_con_importo_data_e_metodo_giusti() {
    let scenario = scenario_con_tutti_i_pagamenti();
    let connessione = &scenario.connessione;
    let pagamenti_di = |iscritto_id| {
        let filtro = FiltroPagamenti {
            iscritto_id: Some(iscritto_id),
            ..Default::default()
        };
        elenca_pagamenti(connessione, &filtro).unwrap()
    };

    let nuova = pagamenti_di(scenario.persona_nuova);
    assert_eq!(nuova.len(), 2, "primo abbonamento e rinnovo");
    assert!(
        nuova
            .iter()
            .all(|pagamento| pagamento.data_pagamento == OGGI)
    );
    let rinnovo = &nuova[0];
    assert_eq!(rinnovo.metodo, MetodoPagamento::Carta);
    assert_eq!(
        rinnovo.data_inizio,
        nuova[1].data_scadenza.succ_opt().unwrap()
    );
    assert_eq!(rinnovo.rinnovo_di_id, Some(nuova[1].id));

    let gia = pagamenti_di(scenario.gia_iscritta);
    assert_eq!(gia.len(), 1);
    assert_eq!(
        (gia[0].importo_cent, gia[0].data_pagamento, gia[0].metodo),
        (4500, data("2026-10-26"), MetodoPagamento::Bonifico)
    );

    let quota_nuova = elenca_iscrizioni(connessione, scenario.persona_nuova).unwrap();
    assert_eq!(
        (quota_nuova[0].importo_cent, quota_nuova[0].data_pagamento),
        (3000, OGGI)
    );
    let quota_gia = elenca_iscrizioni(connessione, scenario.gia_iscritta).unwrap();
    assert_eq!(
        (quota_gia[0].importo_cent, quota_gia[0].data_pagamento),
        (3000, INIZIO_ANNO_SPORTIVO)
    );
}

#[test]
fn gli_incassi_tornano_uguali_in_ogni_riepilogo() {
    let scenario = scenario_con_tutti_i_pagamenti();
    let connessione = &scenario.connessione;
    let (dal, al) = (INIZIO_ANNO_SPORTIVO, OGGI);

    // Pagina Pagamenti: tutti gli abbonamenti, nessuno escluso
    let pagamenti = elenca_pagamenti(connessione, &FiltroPagamenti::default()).unwrap();
    let totale_abbonamenti: i64 = pagamenti.iter().map(|p| p.importo_cent).sum();
    assert_eq!(totale_abbonamenti, 5000 + 4500 + 5000);

    // Pagina Incassi: totale, mese per mese e per metodo
    let riepilogo = riepilogo_incassi(connessione, dal, al, Raggruppamento::Mese).unwrap();
    assert_eq!(riepilogo.totale, incassi(14500, 6000));
    assert_eq!(riepilogo.numero_movimenti, 5);
    let mesi: Vec<(&str, Incassi)> = riepilogo
        .per_periodo
        .iter()
        .map(|periodo| (periodo.periodo.as_str(), periodo.incassi))
        .collect();
    assert_eq!(
        mesi,
        vec![
            ("2026-09", incassi(0, 3000)),
            ("2026-10", incassi(4500, 0)),
            ("2026-11", incassi(10000, 3000)),
        ]
    );
    let metodi: Vec<(MetodoPagamento, i64)> = riepilogo
        .per_metodo
        .iter()
        .map(|voce| (voce.metodo, voce.incassi.totale_cent))
        .collect();
    assert!(metodi.contains(&(MetodoPagamento::Contanti, 8000)));
    assert!(metodi.contains(&(MetodoPagamento::Carta, 5000)));
    assert!(metodi.contains(&(MetodoPagamento::Bonifico, 7500)));

    // Dashboard: mese in corso e precedente
    let dashboard = riepilogo_dashboard(connessione, RIFERIMENTO).unwrap();
    assert_eq!(dashboard.incassi_mese, incassi(10000, 3000));
    assert_eq!(dashboard.incassi_mese_precedente, incassi(4500, 0));

    // Cassa di oggi: i tre movimenti di oggi
    let oggi = elenca_movimenti(connessione, OGGI, OGGI).unwrap();
    assert_eq!(oggi.len(), 3);
    assert_eq!(oggi.iter().map(|m| m.importo_cent).sum::<i64>(), 13000);

    // CSV: una riga per movimento, più l'intestazione
    let csv = esporta_incassi_csv(connessione, dal, al).unwrap();
    assert_eq!(csv.trim_end().split("\r\n").count(), 1 + 5);

    // Statistiche: stesso totale, due persone che hanno pagato
    let statistiche = leggi_statistiche(connessione, dal, al, OGGI).unwrap();
    assert_eq!(statistiche.valore_medio.incassi_cent, 20500);
    assert_eq!(statistiche.valore_medio.iscritti_paganti, 2);
    let vendute: u32 = statistiche
        .vendite_per_tipologia
        .totali
        .iter()
        .map(|tipologia| tipologia.numero)
        .sum();
    assert_eq!(vendute, 3);
}
