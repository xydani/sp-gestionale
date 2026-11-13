mod comune;

use std::collections::HashMap;
use std::time::{Duration, Instant};

use chrono::NaiveDate;
use comune::{DatabaseTemporaneo, PASSWORD_TEST};
use rusqlite::Connection;
use sp_gestionale_lib::dati_prova::genera_dati_prova;
use sp_gestionale_lib::db::crittografia::apri_database_criptato;
use sp_gestionale_lib::domain::stato::{
    GIORNI_PREAVVISO_PREDEFINITI, StatoValidita, calcola_stato_validita,
};

const NUMERO_ISCRITTI: u32 = 250;
const SEME: u64 = 42;
const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const TEMPO_MASSIMO_QUERY: Duration = Duration::from_millis(100);

fn database_con_dati_prova() -> (DatabaseTemporaneo, Connection) {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    genera_dati_prova(&mut connessione, NUMERO_ISCRITTI, OGGI, SEME).unwrap();
    (database, connessione)
}

fn conta(connessione: &Connection, sql: &str) -> i64 {
    connessione.query_row(sql, [], |riga| riga.get(0)).unwrap()
}

fn conta_iscritti_per_stato(connessione: &Connection) -> HashMap<StatoValidita, u32> {
    conta_per_stato(
        connessione,
        "SELECT ultima_data_scadenza FROM v_ultima_scadenza_iscritti",
    )
}

fn conta_per_stato(connessione: &Connection, sql: &str) -> HashMap<StatoValidita, u32> {
    let mut query = connessione.prepare(sql).unwrap();
    let mut conteggi = HashMap::new();
    let scadenze = query
        .query_map([], |riga| riga.get::<_, Option<String>>(0))
        .unwrap();
    for scadenza in scadenze {
        let ultima_data_scadenza = scadenza.unwrap().map(|testo| testo.parse().unwrap());
        let stato =
            calcola_stato_validita(ultima_data_scadenza, OGGI, GIORNI_PREAVVISO_PREDEFINITI);
        *conteggi.entry(stato).or_insert(0) += 1;
    }
    conteggi
}

#[test]
fn vengono_creati_tutti_gli_iscritti_richiesti() {
    let (_database, connessione) = database_con_dati_prova();
    assert_eq!(
        conta(&connessione, "SELECT count(*) FROM iscritti"),
        i64::from(NUMERO_ISCRITTI)
    );
}

#[test]
fn ogni_iscritto_con_pagamenti_ha_uno_storico() {
    let (_database, connessione) = database_con_dati_prova();
    let pagamenti = conta(&connessione, "SELECT count(*) FROM pagamenti");
    assert!(
        pagamenti > i64::from(NUMERO_ISCRITTI),
        "pagamenti generati: {pagamenti}"
    );
}

#[test]
fn sono_presenti_tutti_gli_stati_di_abbonamento() {
    let (_database, connessione) = database_con_dati_prova();
    let conteggi = conta_iscritti_per_stato(&connessione);
    for stato in [
        StatoValidita::Attivo,
        StatoValidita::InScadenza,
        StatoValidita::Scaduto,
        StatoValidita::Nessuno,
    ] {
        assert!(
            conteggi.get(&stato).copied().unwrap_or(0) > 0,
            "manca lo stato {stato:?}"
        );
    }
    assert_eq!(conteggi.values().sum::<u32>(), NUMERO_ISCRITTI);
}

#[test]
fn sono_presenti_tutti_gli_stati_di_iscrizione() {
    let (_database, connessione) = database_con_dati_prova();
    let conteggi = conta_per_stato(
        &connessione,
        "SELECT ultima_data_fine_iscrizione FROM iscritti
         LEFT JOIN v_ultima_iscrizione_iscritti ON iscritto_id = iscritti.id",
    );
    for stato in [
        StatoValidita::Attivo,
        StatoValidita::InScadenza,
        StatoValidita::Scaduto,
        StatoValidita::Nessuno,
    ] {
        assert!(
            conteggi.get(&stato).copied().unwrap_or(0) > 0,
            "manca lo stato di iscrizione {stato:?}"
        );
    }
}

#[test]
fn sono_presenti_tutti_gli_stati_del_certificato() {
    let (_database, connessione) = database_con_dati_prova();
    let conteggi = conta_per_stato(
        &connessione,
        "SELECT data_scadenza_certificato FROM iscritti",
    );
    for stato in [
        StatoValidita::Attivo,
        StatoValidita::InScadenza,
        StatoValidita::Scaduto,
        StatoValidita::Nessuno,
    ] {
        assert!(
            conteggi.get(&stato).copied().unwrap_or(0) > 0,
            "manca lo stato del certificato {stato:?}"
        );
    }
}

#[test]
fn esistono_abbonamenti_attivi_senza_iscrizione_valida() {
    let (_database, connessione) = database_con_dati_prova();
    let oggi = OGGI.to_string();
    let senza_iscrizione: i64 = connessione
        .query_row(
            "SELECT COUNT(*) FROM v_ultima_scadenza_iscritti a
             LEFT JOIN v_ultima_iscrizione_iscritti i USING (iscritto_id)
             WHERE a.ultima_data_scadenza >= ?1
               AND (i.ultima_data_fine_iscrizione IS NULL OR i.ultima_data_fine_iscrizione < ?1)",
            [&oggi],
            |riga| riga.get(0),
        )
        .unwrap();
    assert!(senza_iscrizione > 0);
}

#[test]
fn sono_presenti_tutti_i_metodi_di_pagamento() {
    let (_database, connessione) = database_con_dati_prova();
    assert_eq!(
        conta(&connessione, "SELECT count(DISTINCT metodo) FROM pagamenti"),
        4
    );
}

#[test]
fn i_rinnovi_sono_collegati_al_pagamento_precedente() {
    let (_database, connessione) = database_con_dati_prova();
    let rinnovi_incoerenti = conta(
        &connessione,
        "SELECT count(*) FROM pagamenti p
         JOIN pagamenti precedente ON precedente.id = p.rinnovo_di_id
         WHERE precedente.iscritto_id <> p.iscritto_id
            OR p.data_inizio <= precedente.data_scadenza",
    );
    assert_eq!(rinnovi_incoerenti, 0);
    assert!(
        conta(
            &connessione,
            "SELECT count(*) FROM pagamenti WHERE rinnovo_di_id IS NOT NULL"
        ) > 0
    );
}

#[test]
fn i_dati_sono_gli_stessi_con_lo_stesso_seme() {
    let (_primo, prima_connessione) = database_con_dati_prova();
    let (_secondo, seconda_connessione) = database_con_dati_prova();
    let firma = "SELECT group_concat(cognome || nome || data_nascita, ',') FROM iscritti";
    let prima: String = prima_connessione
        .query_row(firma, [], |riga| riga.get(0))
        .unwrap();
    let seconda: String = seconda_connessione
        .query_row(firma, [], |riga| riga.get(0))
        .unwrap();
    assert_eq!(prima, seconda);
}

#[test]
fn i_dati_restano_leggibili_dopo_la_riapertura() {
    let (database, connessione) = database_con_dati_prova();
    drop(connessione);
    let riaperta = apri_database_criptato(&database.percorso, PASSWORD_TEST).unwrap();
    assert_eq!(
        conta(&riaperta, "SELECT count(*) FROM iscritti"),
        i64::from(NUMERO_ISCRITTI)
    );
}

#[test]
fn lo_stato_di_tutti_gli_iscritti_si_calcola_velocemente() {
    let (_database, connessione) = database_con_dati_prova();
    let inizio = Instant::now();
    conta_iscritti_per_stato(&connessione);
    let durata = inizio.elapsed();
    assert!(durata < TEMPO_MASSIMO_QUERY, "calcolo stati in {durata:?}");
}

#[test]
fn alcuni_iscritti_sono_gia_attivi_e_da_completare() {
    use sp_gestionale_lib::db::iscritti::{FiltroIscritti, Riferimento, elenca_iscritti};
    let (_database, connessione) = database_con_dati_prova();
    let riferimento = Riferimento {
        oggi: OGGI,
        giorni_preavviso: GIORNI_PREAVVISO_PREDEFINITI,
    };
    let filtro = FiltroIscritti {
        solo_da_completare: true,
        ..Default::default()
    };
    let da_completare = elenca_iscritti(&connessione, &filtro, riferimento).unwrap();
    assert!(!da_completare.is_empty());
    for iscritto in da_completare {
        assert_ne!(iscritto.stato_abbonamento, StatoValidita::Scaduto);
    }
}

/// I dati inventati devono superare gli stessi controlli dei dati inseriti a mano,
/// altrimenti "Modifica dati" su un iscritto di prova verrebbe rifiutato.
#[test]
fn gli_iscritti_inventati_superano_i_controlli_dell_anagrafica() {
    use sp_gestionale_lib::db::iscritti::{
        DatiIscritto, FiltroIscritti, Riferimento, elenca_iscritti, valida_dati_iscritto,
    };
    let (_database, connessione) = database_con_dati_prova();
    let riferimento = Riferimento {
        oggi: OGGI,
        giorni_preavviso: GIORNI_PREAVVISO_PREDEFINITI,
    };
    let filtro = FiltroIscritti {
        includi_archiviati: true,
        ..Default::default()
    };
    let iscritti = elenca_iscritti(&connessione, &filtro, riferimento).unwrap();
    let completi = iscritti
        .into_iter()
        .filter(|iscritto| !iscritto.da_completare);
    for iscritto in completi {
        let dati = DatiIscritto {
            nome: iscritto.nome,
            cognome: iscritto.cognome,
            data_nascita: iscritto.data_nascita,
            luogo_nascita: iscritto.luogo_nascita,
            telefono: iscritto.telefono,
            email: iscritto.email,
            note: iscritto.note,
            data_scadenza_certificato: iscritto.data_scadenza_certificato,
            certificato_presentato: false,
        };
        if let Err(errore) = valida_dati_iscritto(&dati, OGGI) {
            panic!("{} {}: {errore}", dati.nome, dati.cognome);
        }
    }
}

/// Abbastanza righe per vedere in dashboard e in Scadenze ogni tipo, passato e vicino.
const MINIMO_PER_TIPO: i64 = 10;

/// Quanti iscritti hanno la scadenza di ogni tipo già passata e quanti nei prossimi 30 giorni.
fn scaduti_e_in_arrivo(connessione: &Connection, sql_scadenze: &str) -> (i64, i64) {
    let scaduti = conta(
        connessione,
        &format!("SELECT count(*) FROM ({sql_scadenze}) WHERE scadenza < '2026-11-10'"),
    );
    let in_arrivo = conta(
        connessione,
        &format!(
            "SELECT count(*) FROM ({sql_scadenze})
             WHERE scadenza BETWEEN '2026-11-10' AND '2026-12-10'"
        ),
    );
    (scaduti, in_arrivo)
}

#[test]
fn ci_sono_scadenze_passate_e_vicine_di_ogni_tipo() {
    let (_database, connessione) = database_con_dati_prova();
    let tipi = [
        (
            "abbonamenti",
            "SELECT ultima_data_scadenza AS scadenza FROM v_ultima_scadenza_iscritti",
        ),
        (
            "iscrizioni",
            "SELECT ultima_data_fine_iscrizione AS scadenza FROM v_ultima_iscrizione_iscritti",
        ),
        (
            "certificati",
            "SELECT data_scadenza_certificato AS scadenza FROM iscritti",
        ),
    ];
    for (tipo, sql) in tipi {
        let (scaduti, in_arrivo) = scaduti_e_in_arrivo(&connessione, sql);
        assert!(
            scaduti >= MINIMO_PER_TIPO && in_arrivo >= MINIMO_PER_TIPO,
            "{tipo}: {scaduti} scaduti, {in_arrivo} nei prossimi 30 giorni"
        );
    }
}
