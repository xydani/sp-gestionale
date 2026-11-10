mod comune;

use comune::DatabaseTemporaneo;
use sp_gestionale_lib::db::ErroreDatabase;
use sp_gestionale_lib::db::tipologie::{
    DatiTipologia, crea_tipologia, elenca_tipologie, imposta_tipologia_attiva, modifica_tipologia,
    trova_tipologia,
};
use sp_gestionale_lib::domain::date::UnitaDurata;

fn dati(
    nome: &str,
    durata_valore: u32,
    durata_unita: UnitaDurata,
    prezzo_cent: i64,
) -> DatiTipologia {
    DatiTipologia {
        nome: nome.into(),
        durata_valore,
        durata_unita,
        prezzo_cent,
    }
}

#[test]
fn il_listino_iniziale_contiene_le_quattro_tipologie_standard() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let nomi: Vec<String> = elenca_tipologie(&connessione, true)
        .unwrap()
        .into_iter()
        .map(|t| t.nome)
        .collect();
    assert_eq!(nomi, ["Mensile", "Trimestrale", "Semestrale", "Annuale"]);
}

#[test]
fn si_crea_una_tipologia_in_giorni() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();

    let creata = crea_tipologia(
        &connessione,
        &dati(" Settimanale ", 7, UnitaDurata::Giorni, 1500),
    )
    .unwrap();

    assert_eq!(creata.nome, "Settimanale");
    assert_eq!(creata.durata_unita, UnitaDurata::Giorni);
    assert!(creata.attiva);
    assert_eq!(trova_tipologia(&connessione, creata.id).unwrap(), creata);
}

#[test]
fn il_nome_deve_essere_unico_senza_distinzione_di_maiuscole() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let errore =
        crea_tipologia(&connessione, &dati("mensile", 1, UnitaDurata::Mesi, 4000)).unwrap_err();
    assert!(
        matches!(errore, ErroreDatabase::DatiNonValidi(messaggio) if messaggio.contains("Esiste già"))
    );
}

#[test]
fn si_modifica_prezzo_e_durata() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let mensile = &elenca_tipologie(&connessione, true).unwrap()[0];

    let modificata = modifica_tipologia(
        &connessione,
        mensile.id,
        &dati("Mensile", 1, UnitaDurata::Mesi, 5500),
    )
    .unwrap();

    assert_eq!(modificata.prezzo_cent, 5500);
}

#[test]
fn modificare_una_tipologia_inesistente_e_un_errore() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let errore =
        modifica_tipologia(&connessione, 999, &dati("X", 1, UnitaDurata::Mesi, 1)).unwrap_err();
    assert!(matches!(errore, ErroreDatabase::NonTrovato));
}

#[test]
fn una_tipologia_disattivata_sparisce_dal_listino_attivo_ma_resta_nell_elenco_completo() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let annuale = elenca_tipologie(&connessione, true).unwrap().pop().unwrap();

    imposta_tipologia_attiva(&connessione, annuale.id, false).unwrap();

    assert_eq!(elenca_tipologie(&connessione, true).unwrap().len(), 3);
    let tutte = elenca_tipologie(&connessione, false).unwrap();
    assert_eq!(tutte.len(), 4);
    assert!(!tutte.last().unwrap().attiva);
}

#[test]
fn rinominare_una_tipologia_non_cambia_lo_storico_dei_pagamenti() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    connessione
        .execute(
            "INSERT INTO iscritti (nome, cognome, data_iscrizione) VALUES ('Mario', 'Prova', '2026-01-01')",
            [],
        )
        .unwrap();
    connessione
        .execute(
            "INSERT INTO pagamenti (iscritto_id, tipologia_id, tipologia_nome, importo_cent, data_pagamento, metodo, data_inizio, data_scadenza)
             VALUES (1, 1, 'Mensile', 5000, '2026-01-01', 'contanti', '2026-01-01', '2026-01-31')",
            [],
        )
        .unwrap();

    modifica_tipologia(
        &connessione,
        1,
        &dati("Mensile Plus", 1, UnitaDurata::Mesi, 6000),
    )
    .unwrap();

    let nome_nello_storico: String = connessione
        .query_row(
            "SELECT tipologia_nome FROM pagamenti WHERE id = 1",
            [],
            |riga| riga.get(0),
        )
        .unwrap();
    assert_eq!(nome_nello_storico, "Mensile");
}
