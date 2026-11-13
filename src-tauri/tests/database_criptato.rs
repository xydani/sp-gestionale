mod comune;

use std::fs;

use comune::{DatabaseTemporaneo, PASSWORD_TEST};
use rusqlite::Connection;
use sp_gestionale_lib::db::ErroreDatabase;
use sp_gestionale_lib::db::crittografia::{apri_database_criptato, crea_database_criptato};
use sp_gestionale_lib::db::migrazioni::{
    applica_migrazioni, ci_sono_migrazioni_da_applicare, versione_schema_corrente,
    versione_schema_supportata,
};

const INTESTAZIONE_SQLITE_IN_CHIARO: &[u8] = b"SQLite format 3\0";

#[test]
fn il_file_su_disco_e_criptato() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    connessione
        .execute(
            "INSERT INTO iscritti (nome, cognome) VALUES ('Testo', 'Riconoscibile')",
            [],
        )
        .unwrap();
    connessione
        .pragma_update(None, "wal_checkpoint", "TRUNCATE")
        .unwrap();
    drop(connessione);

    let contenuto = fs::read(&database.percorso).unwrap();
    assert!(!contenuto.starts_with(INTESTAZIONE_SQLITE_IN_CHIARO));
    let testo_in_chiaro = b"Riconoscibile";
    assert!(
        !contenuto
            .windows(testo_in_chiaro.len())
            .any(|finestra| finestra == testo_in_chiaro)
    );
}

#[test]
fn senza_password_il_database_non_si_legge() {
    let database = DatabaseTemporaneo::nuovo();
    drop(database.crea_con_schema());

    let connessione_senza_chiave = Connection::open(&database.percorso).unwrap();
    assert!(
        connessione_senza_chiave
            .query_row("SELECT count(*) FROM sqlite_master", [], |_| Ok(()))
            .is_err()
    );
}

#[test]
fn con_la_password_giusta_il_database_si_riapre() {
    let database = DatabaseTemporaneo::nuovo();
    drop(database.crea_con_schema());

    let connessione = apri_database_criptato(&database.percorso, PASSWORD_TEST).unwrap();
    let numero_tipologie: i64 = connessione
        .query_row("SELECT count(*) FROM tipologie_abbonamento", [], |riga| {
            riga.get(0)
        })
        .unwrap();
    assert_eq!(numero_tipologie, 4);
}

#[test]
fn con_la_password_sbagliata_l_apertura_e_rifiutata() {
    let database = DatabaseTemporaneo::nuovo();
    drop(database.crea_con_schema());

    let risultato = apri_database_criptato(&database.percorso, "password-sbagliata");
    assert!(matches!(risultato, Err(ErroreDatabase::PasswordErrata)));
}

#[test]
fn password_troppo_corta_e_rifiutata_alla_creazione() {
    let database = DatabaseTemporaneo::nuovo();
    let risultato = crea_database_criptato(&database.percorso, "7caratt");
    assert!(matches!(
        risultato,
        Err(ErroreDatabase::PasswordTroppoCorta { minimo: 8 })
    ));
    assert!(!database.percorso.exists());
}

#[test]
fn bastano_otto_caratteri() {
    let database = DatabaseTemporaneo::nuovo();
    assert!(crea_database_criptato(&database.percorso, "8caratte").is_ok());
}

#[test]
fn non_si_sovrascrive_un_database_esistente() {
    let database = DatabaseTemporaneo::nuovo();
    drop(database.crea_con_schema());
    let risultato = crea_database_criptato(&database.percorso, PASSWORD_TEST);
    assert!(matches!(
        risultato,
        Err(ErroreDatabase::DatabaseGiaEsistente)
    ));
}

#[test]
fn aprire_un_database_inesistente_e_un_errore() {
    let database = DatabaseTemporaneo::nuovo();
    let risultato = apri_database_criptato(&database.percorso, PASSWORD_TEST);
    assert!(matches!(risultato, Err(ErroreDatabase::DatabaseNonTrovato)));
}

#[test]
fn le_migrazioni_portano_lo_schema_all_ultima_versione() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = crea_database_criptato(&database.percorso, PASSWORD_TEST).unwrap();
    assert!(ci_sono_migrazioni_da_applicare(&connessione).unwrap());

    let applicate = applica_migrazioni(&mut connessione).unwrap();

    assert_eq!(applicate, versione_schema_supportata());
    assert_eq!(
        versione_schema_corrente(&connessione).unwrap(),
        versione_schema_supportata()
    );
    assert!(!ci_sono_migrazioni_da_applicare(&connessione).unwrap());
}

#[test]
fn riapplicare_le_migrazioni_non_cambia_nulla() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    assert_eq!(applica_migrazioni(&mut connessione).unwrap(), 0);
}

#[test]
fn database_di_una_versione_successiva_e_rifiutato() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let versione_futura = versione_schema_supportata() + 1;
    connessione
        .pragma_update(None, "user_version", versione_futura)
        .unwrap();

    let risultato = applica_migrazioni(&mut connessione);

    assert!(matches!(
        risultato,
        Err(ErroreDatabase::VersioneDatabaseSuccessiva { versione_database, .. })
            if versione_database == versione_futura
    ));
}

#[test]
fn le_chiavi_esterne_sono_attive() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let risultato = connessione.execute(
        "INSERT INTO pagamenti (iscritto_id, tipologia_nome, importo_cent, data_pagamento, metodo, data_inizio, data_scadenza)
         VALUES (999, 'Mensile', 5000, '2026-01-01', 'contanti', '2026-01-01', '2026-01-31')",
        [],
    );
    assert!(risultato.is_err());
}

#[test]
fn lo_schema_rifiuta_date_non_iso_e_scadenze_prima_dell_inizio() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let data_non_iso = connessione.execute(
        "INSERT INTO iscritti (nome, cognome, data_nascita) VALUES ('Mario', 'Prova', '01/02/1990')",
        [],
    );
    assert!(data_non_iso.is_err());

    connessione
        .execute(
            "INSERT INTO iscritti (nome, cognome) VALUES ('Mario', 'Prova')",
            [],
        )
        .unwrap();
    let scadenza_prima_dell_inizio = connessione.execute(
        "INSERT INTO pagamenti (iscritto_id, tipologia_nome, importo_cent, data_pagamento, metodo, data_inizio, data_scadenza)
         VALUES (1, 'Mensile', 5000, '2026-01-01', 'contanti', '2026-02-01', '2026-01-31')",
        [],
    );
    assert!(scadenza_prima_dell_inizio.is_err());
}

#[test]
fn il_metodo_di_pagamento_deve_essere_tra_quelli_ammessi() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    connessione
        .execute(
            "INSERT INTO iscritti (nome, cognome) VALUES ('Mario', 'Prova')",
            [],
        )
        .unwrap();
    let risultato = connessione.execute(
        "INSERT INTO pagamenti (iscritto_id, tipologia_nome, importo_cent, data_pagamento, metodo, data_inizio, data_scadenza)
         VALUES (1, 'Mensile', 5000, '2026-01-01', 'assegno', '2026-01-01', '2026-01-31')",
        [],
    );
    assert!(risultato.is_err());
}
