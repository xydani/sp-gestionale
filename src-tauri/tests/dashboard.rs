mod comune;

use chrono::NaiveDate;
use comune::DatabaseTemporaneo;
use sp_gestionale_lib::dati_prova::genera_dati_prova;
use sp_gestionale_lib::db::dashboard::{PROSSIME_SCADENZE_MASSIME, riepilogo_dashboard};
use sp_gestionale_lib::db::iscritti::{
    FiltroIscritti, Riferimento, elenca_iscritti, imposta_iscritto_archiviato,
};
use sp_gestionale_lib::domain::stato::StatoValidita;

const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const RIFERIMENTO: Riferimento = Riferimento {
    oggi: OGGI,
    giorni_preavviso: 7,
};

#[test]
fn i_conteggi_coincidono_con_gli_stati_degli_iscritti() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    genera_dati_prova(&mut connessione, 250, OGGI, 13).unwrap();
    imposta_iscritto_archiviato(&connessione, 1, true, RIFERIMENTO).unwrap();
    let iscritti = elenca_iscritti(&connessione, &FiltroIscritti::default(), RIFERIMENTO).unwrap();
    let conta = |stato_di: fn(&sp_gestionale_lib::db::iscritti::Iscritto) -> StatoValidita,
                 stato: StatoValidita| {
        iscritti.iter().filter(|i| stato_di(i) == stato).count() as u32
    };

    let riepilogo = riepilogo_dashboard(&connessione, RIFERIMENTO).unwrap();

    assert_eq!(riepilogo.iscritti, 249);
    let abbonamento = |i: &sp_gestionale_lib::db::iscritti::Iscritto| i.stato_abbonamento;
    let iscrizione = |i: &sp_gestionale_lib::db::iscritti::Iscritto| i.stato_iscrizione;
    assert_eq!(
        riepilogo.abbonamenti.attivi,
        conta(abbonamento, StatoValidita::Attivo)
    );
    assert_eq!(
        riepilogo.abbonamenti.in_scadenza,
        conta(abbonamento, StatoValidita::InScadenza)
    );
    assert_eq!(
        riepilogo.abbonamenti.scaduti,
        conta(abbonamento, StatoValidita::Scaduto)
    );
    assert_eq!(
        riepilogo.abbonamenti.assenti,
        conta(abbonamento, StatoValidita::Nessuno)
    );
    assert_eq!(
        riepilogo.iscrizioni.attivi,
        conta(iscrizione, StatoValidita::Attivo)
    );
    assert_eq!(
        riepilogo.iscrizioni.scaduti,
        conta(iscrizione, StatoValidita::Scaduto)
    );
    let somma = riepilogo.abbonamenti.attivi
        + riepilogo.abbonamenti.in_scadenza
        + riepilogo.abbonamenti.scaduti
        + riepilogo.abbonamenti.assenti;
    assert_eq!(somma, 249);
}

#[test]
fn gli_incassi_del_mese_sommano_abbonamenti_e_iscrizioni() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    connessione
        .execute_batch(
            "INSERT INTO iscritti (nome, cognome) VALUES ('Mario', 'Rossi');
             INSERT INTO pagamenti (iscritto_id, tipologia_nome, importo_cent, data_pagamento, metodo, data_inizio, data_scadenza)
             VALUES (1, 'Mensile', 5000, '2026-11-01', 'contanti', '2026-11-01', '2026-11-30'),
                    (1, 'Mensile', 5000, '2026-11-30', 'carta', '2026-12-01', '2026-12-31'),
                    (1, 'Mensile', 4500, '2026-10-31', 'carta', '2026-10-01', '2026-10-31');
             INSERT INTO iscrizioni (iscritto_id, importo_cent, data_pagamento, metodo, data_inizio, data_fine)
             VALUES (1, 3000, '2026-11-05', 'contanti', '2026-09-01', '2027-08-31');",
        )
        .unwrap();

    let riepilogo = riepilogo_dashboard(&connessione, RIFERIMENTO).unwrap();

    assert_eq!(riepilogo.incassi_mese.abbonamenti_cent, 10000);
    assert_eq!(riepilogo.incassi_mese.iscrizioni_cent, 3000);
    assert_eq!(riepilogo.incassi_mese.totale_cent, 13000);
    assert_eq!(riepilogo.incassi_mese_precedente.totale_cent, 4500);
}

#[test]
fn le_prossime_scadenze_sono_ordinate_e_limitate() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    genera_dati_prova(&mut connessione, 250, OGGI, 13).unwrap();

    let riepilogo = riepilogo_dashboard(&connessione, RIFERIMENTO).unwrap();
    let scadenze = &riepilogo.prossime_scadenze;

    assert!(!scadenze.is_empty());
    assert!(scadenze.len() <= PROSSIME_SCADENZE_MASSIME as usize);
    assert!(scadenze.iter().all(|s| s.data_scadenza >= OGGI));
    assert!(
        scadenze
            .windows(2)
            .all(|coppia| coppia[0].data_scadenza <= coppia[1].data_scadenza)
    );
    assert!(
        scadenze
            .iter()
            .all(|s| s.tipo == "abbonamento" || s.tipo == "iscrizione")
    );
}
