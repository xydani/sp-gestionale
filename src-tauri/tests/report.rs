mod comune;

use chrono::NaiveDate;
use comune::DatabaseTemporaneo;
use rusqlite::Connection;
use sp_gestionale_lib::dati_prova::genera_dati_prova;
use sp_gestionale_lib::db::dashboard::{Incassi, incassi_tra, mese_di};
use sp_gestionale_lib::db::report::{
    Raggruppamento, RiepilogoIncassi, elenca_movimenti, esporta_incassi_csv, riepilogo_incassi,
};

const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const DAL: NaiveDate = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
const AL: NaiveDate = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();

fn database_con_dati_prova() -> (DatabaseTemporaneo, Connection) {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    genera_dati_prova(&mut connessione, 250, OGGI, 7).unwrap();
    (database, connessione)
}

fn somma(incassi: impl Iterator<Item = Incassi>) -> Incassi {
    incassi.fold(Incassi::default(), |totale, parziale| Incassi {
        abbonamenti_cent: totale.abbonamenti_cent + parziale.abbonamenti_cent,
        iscrizioni_cent: totale.iscrizioni_cent + parziale.iscrizioni_cent,
        totale_cent: totale.totale_cent + parziale.totale_cent,
    })
}

fn riepilogo(connessione: &Connection, raggruppamento: Raggruppamento) -> RiepilogoIncassi {
    riepilogo_incassi(connessione, DAL, AL, raggruppamento).unwrap()
}

#[test]
fn i_totali_per_periodo_e_per_metodo_coincidono_con_il_totale() {
    let (_database, connessione) = database_con_dati_prova();
    for raggruppamento in [
        Raggruppamento::Giorno,
        Raggruppamento::Mese,
        Raggruppamento::Anno,
    ] {
        let riepilogo = riepilogo(&connessione, raggruppamento);
        assert!(riepilogo.totale.iscrizioni_cent > 0);
        assert!(riepilogo.totale.abbonamenti_cent > 0);
        assert_eq!(
            somma(riepilogo.per_periodo.iter().map(|p| p.incassi)),
            riepilogo.totale
        );
        assert_eq!(
            somma(riepilogo.per_metodo.iter().map(|m| m.incassi)),
            riepilogo.totale
        );
    }
}

#[test]
fn le_chiavi_seguono_il_raggruppamento_in_ordine() {
    let (_database, connessione) = database_con_dati_prova();
    for (raggruppamento, lunghezza) in [
        (Raggruppamento::Giorno, 10),
        (Raggruppamento::Mese, 7),
        (Raggruppamento::Anno, 4),
    ] {
        let periodi: Vec<String> = riepilogo(&connessione, raggruppamento)
            .per_periodo
            .into_iter()
            .map(|p| p.periodo)
            .collect();
        assert!(periodi.iter().all(|p| p.len() == lunghezza));
        assert!(periodi.windows(2).all(|coppia| coppia[0] < coppia[1]));
    }
}

#[test]
fn ogni_mese_coincide_con_gli_incassi_della_dashboard() {
    let (_database, connessione) = database_con_dati_prova();
    for mese in riepilogo(&connessione, Raggruppamento::Mese).per_periodo {
        let primo: NaiveDate = format!("{}-01", mese.periodo).parse().unwrap();
        let (dal, al) = mese_di(primo);
        assert_eq!(mese.incassi, incassi_tra(&connessione, dal, al).unwrap());
    }
}

#[test]
fn gli_estremi_del_periodo_sono_inclusi() {
    let (_database, connessione) = database_con_dati_prova();
    let giorni = riepilogo(&connessione, Raggruppamento::Giorno).per_periodo;
    let giorno: NaiveDate = giorni[giorni.len() / 2].periodo.parse().unwrap();
    let solo_quel_giorno =
        riepilogo_incassi(&connessione, giorno, giorno, Raggruppamento::Giorno).unwrap();
    assert_eq!(solo_quel_giorno.per_periodo.len(), 1);
    assert_eq!(solo_quel_giorno.totale, giorni[giorni.len() / 2].incassi);
}

#[test]
fn un_periodo_senza_movimenti_ha_totali_a_zero() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let riepilogo = riepilogo(&connessione, Raggruppamento::Mese);
    assert_eq!(riepilogo.totale, Incassi::default());
    assert_eq!(riepilogo.numero_movimenti, 0);
    assert!(riepilogo.per_periodo.is_empty());
    assert!(riepilogo.per_metodo.is_empty());
}

#[test]
fn un_periodo_invertito_e_rifiutato() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    assert!(riepilogo_incassi(&connessione, AL, DAL, Raggruppamento::Mese).is_err());
    assert!(esporta_incassi_csv(&connessione, AL, DAL).is_err());
}

#[test]
fn il_csv_contiene_una_riga_per_movimento() {
    let (_database, connessione) = database_con_dati_prova();
    let riepilogo = riepilogo(&connessione, Raggruppamento::Anno);
    let csv = esporta_incassi_csv(&connessione, DAL, AL).unwrap();
    let righe: Vec<&str> = csv.trim_end().split("\r\n").collect();
    assert_eq!(
        righe[0],
        "\u{feff}Data;Tipo;Cognome;Nome;Descrizione;Metodo;Importo (€)"
    );
    assert_eq!(righe.len() as u32, riepilogo.numero_movimenti + 1);
    assert!(righe[1..].iter().any(|riga| riga.contains(";Iscrizione;")));
    assert!(righe[1..].iter().any(|riga| riga.contains(";Abbonamento;")));
}

#[test]
fn i_movimenti_del_giorno_tornano_con_il_totale_della_cassa() {
    let (_database, connessione) = database_con_dati_prova();
    let riepilogo = riepilogo_incassi(&connessione, DAL, AL, Raggruppamento::Giorno).unwrap();
    let giorno_con_incassi = riepilogo.per_periodo.last().unwrap();
    let giorno: NaiveDate = giorno_con_incassi.periodo.parse().unwrap();

    let movimenti = elenca_movimenti(&connessione, giorno, giorno).unwrap();

    assert!(!movimenti.is_empty());
    assert!(movimenti.iter().all(|movimento| movimento.data == giorno));
    let totale: i64 = movimenti
        .iter()
        .map(|movimento| movimento.importo_cent)
        .sum();
    assert_eq!(totale, giorno_con_incassi.incassi.totale_cent);
}
