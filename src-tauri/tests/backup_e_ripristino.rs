mod comune;

use std::fs;

use chrono::{NaiveDate, NaiveDateTime};
use comune::PASSWORD_TEST;
use rusqlite::Connection;
use sp_gestionale_lib::dati_prova::genera_dati_prova;
use sp_gestionale_lib::db::backup::{esegui_backup, esegui_backup_giornaliero};
use sp_gestionale_lib::db::crittografia::apri_database_criptato;
use sp_gestionale_lib::db::migrazioni::{versione_schema_corrente, versione_schema_supportata};
use sp_gestionale_lib::stato_app::StatoApp;
use zeroize::Zeroizing;

const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const INTESTAZIONE_SQLITE_IN_CHIARO: &[u8] = b"SQLite format 3\0";

fn segreto(testo: &str) -> Zeroizing<String> {
    Zeroizing::new(testo.to_string())
}

fn orologio_fisso() -> NaiveDateTime {
    OGGI.and_hms_opt(10, 0, 0).unwrap()
}

fn conta_iscritti(connessione: &Connection) -> i64 {
    connessione
        .query_row("SELECT count(*) FROM iscritti", [], |riga| riga.get(0))
        .unwrap()
}

#[test]
fn il_backup_e_criptato_e_contiene_tutti_i_dati() {
    let database = comune::DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    genera_dati_prova(&mut connessione, 200, OGGI, 7).unwrap();
    let destinazione = database.percorso.with_file_name("copia.db");

    esegui_backup(&connessione, &destinazione).unwrap();

    assert!(
        !fs::read(&destinazione)
            .unwrap()
            .starts_with(INTESTAZIONE_SQLITE_IN_CHIARO)
    );
    let copia = apri_database_criptato(&destinazione, PASSWORD_TEST).unwrap();
    assert_eq!(conta_iscritti(&copia), 200);
    assert!(apri_database_criptato(&destinazione, "password-sbagliata").is_err());
}

#[test]
fn il_backup_conserva_la_versione_dello_schema() {
    let database = comune::DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let destinazione = database.percorso.with_file_name("copia.db");

    esegui_backup(&connessione, &destinazione).unwrap();

    let copia = apri_database_criptato(&destinazione, PASSWORD_TEST).unwrap();
    assert_eq!(
        versione_schema_corrente(&copia).unwrap(),
        versione_schema_supportata()
    );
}

#[test]
fn il_backup_non_sovrascrive_un_file_esistente() {
    let database = comune::DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let destinazione = database.percorso.with_file_name("esistente.db");
    fs::write(&destinazione, b"dati preziosi").unwrap();

    assert!(esegui_backup(&connessione, &destinazione).is_err());
    assert_eq!(fs::read(&destinazione).unwrap(), b"dati preziosi");
}

#[test]
fn il_backup_giornaliero_rispetta_il_numero_da_conservare() {
    let database = comune::DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    let cartella_backup = database.percorso.with_file_name("backup");

    for giorno in 1..=4 {
        let adesso = NaiveDate::from_ymd_opt(2026, 11, giorno)
            .unwrap()
            .and_hms_opt(9, 0, 0)
            .unwrap();
        esegui_backup_giornaliero(&connessione, &cartella_backup, adesso, 3).unwrap();
    }

    assert_eq!(fs::read_dir(&cartella_backup).unwrap().count(), 3);
}

#[test]
fn il_ripristino_riporta_i_dati_del_backup_e_salva_quelli_attuali() {
    let cartella = tempfile::tempdir().unwrap();
    let stato = StatoApp::con_orologio(cartella.path().join("palestra.db"), orologio_fisso);
    stato
        .crea_database(segreto(PASSWORD_TEST), segreto(PASSWORD_TEST))
        .unwrap();

    // Backup di un archivio con 200 iscritti
    let percorso_sorgente = cartella.path().join("sorgente.db");
    {
        let mut sorgente = sp_gestionale_lib::db::crittografia::crea_database_criptato(
            &percorso_sorgente,
            PASSWORD_TEST,
        )
        .unwrap();
        sp_gestionale_lib::db::migrazioni::applica_migrazioni(&mut sorgente).unwrap();
        genera_dati_prova(&mut sorgente, 200, OGGI, 3).unwrap();
    }
    let cartella_esterna = cartella.path().join("chiavetta");
    fs::create_dir_all(&cartella_esterna).unwrap();
    let stato_sorgente = StatoApp::con_orologio(percorso_sorgente, orologio_fisso);
    stato_sorgente
        .sblocca_database(segreto(PASSWORD_TEST))
        .unwrap();
    let percorso_backup = stato_sorgente
        .esegui_backup_manuale(&cartella_esterna)
        .unwrap();

    stato
        .ripristina_backup(&percorso_backup, segreto(PASSWORD_TEST))
        .unwrap();

    stato.blocca_database();
    stato.sblocca_database(segreto(PASSWORD_TEST)).unwrap();
    let ripristinato =
        apri_database_criptato(&cartella.path().join("palestra.db"), PASSWORD_TEST).unwrap();
    assert_eq!(conta_iscritti(&ripristinato), 200);

    let copie_pre_ripristino = fs::read_dir(stato.cartella_backup())
        .unwrap()
        .filter(|voce| {
            voce.as_ref()
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with("pre-ripristino-")
        })
        .count();
    assert_eq!(copie_pre_ripristino, 1);
}

#[test]
fn il_ripristino_con_password_sbagliata_non_tocca_i_dati() {
    let cartella = tempfile::tempdir().unwrap();
    let stato = StatoApp::con_orologio(cartella.path().join("palestra.db"), orologio_fisso);
    stato
        .crea_database(segreto(PASSWORD_TEST), segreto(PASSWORD_TEST))
        .unwrap();
    let percorso_backup = stato.esegui_backup_manuale(cartella.path()).unwrap();

    let errore = stato
        .ripristina_backup(&percorso_backup, segreto("password-sbagliata"))
        .unwrap_err();

    assert_eq!(errore.codice, "password_errata");
    assert_eq!(
        stato.stato_database(),
        sp_gestionale_lib::stato_app::StatoDatabase::Sbloccato
    );
}
