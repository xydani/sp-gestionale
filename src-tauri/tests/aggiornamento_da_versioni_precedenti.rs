//! Chi aggiorna dopo diverse release salta più versioni in un colpo: all'avvio le migrazioni
//! mancanti si applicano tutte, in ordine. Qui si parte da un archivio della prima versione
//! con dati veri e si verifica che, da qualunque versione intermedia, si arrivi all'ultima
//! con i dati intatti e il programma pienamente utilizzabile.

mod comune;

use chrono::{NaiveDate, NaiveDateTime};
use comune::{DatabaseTemporaneo, PASSWORD_TEST};
use rusqlite::Connection;
use sp_gestionale_lib::db::crittografia::crea_database_criptato;
use sp_gestionale_lib::db::dashboard::riepilogo_dashboard;
use sp_gestionale_lib::db::impostazioni::{configurazione_iniziale_completata, leggi_impostazioni};
use sp_gestionale_lib::db::iscritti::{FiltroIscritti, Riferimento, elenca_iscritti};
use sp_gestionale_lib::db::iscrizioni::{DatiIscrizione, registra_iscrizione};
use sp_gestionale_lib::db::migrazioni::{
    applica_migrazioni, applica_migrazioni_fino_a, versione_schema_corrente,
    versione_schema_supportata,
};
use sp_gestionale_lib::db::pagamenti::{FiltroPagamenti, elenca_pagamenti};
use sp_gestionale_lib::db::report::{Raggruppamento, riepilogo_incassi};
use sp_gestionale_lib::db::statistiche::leggi_statistiche;
use sp_gestionale_lib::domain::pagamento::MetodoPagamento;
use sp_gestionale_lib::stato_app::StatoApp;
use zeroize::Zeroizing;

const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
const RIFERIMENTO: Riferimento = Riferimento {
    oggi: OGGI,
    giorni_preavviso: 10,
};
const PRIMA_VERSIONE: u32 = 1;

fn data(testo: &str) -> NaiveDate {
    testo.parse().unwrap()
}

/// Dati come li scriveva la prima versione del programma (schema 1).
fn inserisci_dati_della_prima_versione(connessione: &Connection) {
    connessione
        .execute_batch(
            "INSERT INTO iscritti (nome, cognome, telefono, data_iscrizione, note)
             VALUES ('Mario', 'Rossi', '+39 333 1234567', '2025-09-01', 'Allergico'),
                    ('Anna', 'Bianchi', NULL, '2026-01-15', NULL);
             INSERT INTO pagamenti (iscritto_id, tipologia_id, tipologia_nome, importo_cent,
                                    data_pagamento, metodo, data_inizio, data_scadenza)
             VALUES (1, 1, 'Mensile', 5000, '2026-10-20', 'contanti', '2026-10-20', '2026-11-19'),
                    (2, 4, 'Annuale', 45000, '2026-01-15', 'carta', '2026-01-15', '2027-01-14');
             INSERT INTO notifiche_inviate (pagamento_id, tipo) VALUES (1, 'in_scadenza');
             UPDATE impostazioni SET valore = '10' WHERE chiave = 'giorni_preavviso';
             UPDATE impostazioni SET valore = 'Palestra Esempio' WHERE chiave = 'nome_palestra';",
        )
        .unwrap();
}

fn archivio_della_prima_versione(database: &DatabaseTemporaneo) -> Connection {
    let mut connessione = crea_database_criptato(&database.percorso, PASSWORD_TEST).unwrap();
    applica_migrazioni_fino_a(&mut connessione, PRIMA_VERSIONE).unwrap();
    inserisci_dati_della_prima_versione(&connessione);
    connessione
}

/// I dati di prima ci sono ancora e ogni parte del programma li legge.
fn verifica_archivio_aggiornato(connessione: &Connection) {
    assert_eq!(
        versione_schema_corrente(connessione).unwrap(),
        versione_schema_supportata()
    );

    let filtro = FiltroIscritti {
        includi_archiviati: true,
        ..Default::default()
    };
    let iscritti = elenca_iscritti(connessione, &filtro, RIFERIMENTO).unwrap();
    assert_eq!(iscritti.len(), 2);
    let mario = iscritti
        .iter()
        .find(|iscritto| iscritto.nome == "Mario")
        .unwrap();
    assert_eq!(mario.telefono.as_deref(), Some("+39 333 1234567"));
    assert_eq!(mario.note.as_deref(), Some("Allergico"));
    assert_eq!(
        mario.ultima_data_scadenza_abbonamento,
        Some(data("2026-11-19"))
    );
    // Prima la data di nascita non c'era: va completata, ma l'iscritto resta utilizzabile
    assert!(mario.da_completare);

    let pagamenti = elenca_pagamenti(connessione, &FiltroPagamenti::default()).unwrap();
    assert_eq!(pagamenti.len(), 2);
    assert!(pagamenti.iter().all(|pagamento| !pagamento.pregresso));

    let impostazioni = leggi_impostazioni(connessione).unwrap();
    assert_eq!(impostazioni.giorni_preavviso, 10);
    assert_eq!(impostazioni.nome_palestra, "Palestra Esempio");
    assert_eq!(impostazioni.mese_inizio_anno_sportivo, 9);
    assert_eq!(impostazioni.giorni_tolleranza_rientro, 30);
    // Chi usava già il programma non deve rifare la configurazione guidata
    assert!(configurazione_iniziale_completata(connessione).unwrap());

    let incassi =
        riepilogo_incassi(connessione, data("2026-01-01"), OGGI, Raggruppamento::Mese).unwrap();
    assert_eq!(incassi.totale.totale_cent, 50000);
    riepilogo_dashboard(connessione, RIFERIMENTO).unwrap();
    leggi_statistiche(connessione, data("2025-11-11"), OGGI, OGGI).unwrap();
}

/// Dopo l'aggiornamento si lavora come sempre: le nuove registrazioni funzionano.
fn verifica_che_si_possa_continuare_a_lavorare(connessione: &Connection) {
    let iscrizione = DatiIscrizione {
        iscritto_id: 1,
        importo_cent: 3000,
        data_pagamento: OGGI,
        metodo: MetodoPagamento::Contanti,
        data_inizio: data("2026-09-01"),
        data_fine: data("2027-08-31"),
        note: None,
    };
    registra_iscrizione(connessione, &iscrizione, OGGI).unwrap();
}

#[test]
fn dalla_prima_versione_si_arriva_all_ultima_in_un_solo_avvio() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = archivio_della_prima_versione(&database);

    applica_migrazioni(&mut connessione).unwrap();

    verifica_archivio_aggiornato(&connessione);
    verifica_che_si_possa_continuare_a_lavorare(&connessione);
}

#[test]
fn da_qualunque_versione_intermedia_si_arriva_all_ultima() {
    for versione_intermedia in PRIMA_VERSIONE..=versione_schema_supportata() {
        let database = DatabaseTemporaneo::nuovo();
        let mut connessione = archivio_della_prima_versione(&database);

        applica_migrazioni_fino_a(&mut connessione, versione_intermedia).unwrap();
        applica_migrazioni(&mut connessione).unwrap();

        verifica_archivio_aggiornato(&connessione);
    }
}

fn orologio_fisso() -> NaiveDateTime {
    NaiveDateTime::parse_from_str("2026-11-10 09:30:00", "%Y-%m-%d %H:%M:%S").unwrap()
}

#[test]
fn allo_sblocco_un_archivio_vecchio_viene_salvato_e_poi_aggiornato() {
    let database = DatabaseTemporaneo::nuovo();
    drop(archivio_della_prima_versione(&database));
    let stato = StatoApp::con_orologio(database.percorso.clone(), orologio_fisso);

    stato
        .sblocca_database(Zeroizing::new(PASSWORD_TEST.to_string()))
        .unwrap();

    let backup_pre_migrazione = std::fs::read_dir(stato.cartella_backup())
        .unwrap()
        .filter_map(Result::ok)
        .any(|voce| {
            let nome = voce.file_name().to_string_lossy().into_owned();
            nome.starts_with("pre-migrazione-v1-")
        });
    assert!(
        backup_pre_migrazione,
        "manca il backup prima delle migrazioni"
    );
    stato
        .con_connessione(|connessione| {
            verifica_archivio_aggiornato(connessione);
            Ok(())
        })
        .unwrap();
}

/// Impronte delle migrazioni già rilasciate. Chi aggiorna saltando versioni le esegue
/// tutte in fila: se una cambiasse, gli archivi creati prima e dopo non coinciderebbero.
/// Una modifica dello schema va in una migrazione nuova, da aggiungere qui dopo il rilascio.
const MIGRAZIONI_RILASCIATE: [(&str, u64); 8] = [
    ("0001_init.sql", 0xff798d23f0c04256),
    ("0002_iscrizioni_e_anagrafica.sql", 0x25dbc95ec8d08b86),
    ("0003_promemoria.sql", 0x1d8adf4514259c5c),
    ("0004_configurazione_iniziale.sql", 0x821af88621c78786),
    ("0005_certificato_medico.sql", 0xf69df582a30a052a),
    ("0006_eliminazione_iscritti.sql", 0xe3d0873fd5e91c0c),
    ("0007_situazione_iniziale.sql", 0x64ace4f1586f634f),
    ("0008_certificato_senza_scadenza.sql", 0x62aef471a28f167c),
];

/// FNV-1a a 64 bit: semplice e stabile tra versioni di Rust (a differenza di `DefaultHasher`).
fn impronta(contenuto: &[u8]) -> u64 {
    let mut impronta: u64 = 0xcbf29ce484222325;
    for &byte in contenuto {
        impronta ^= u64::from(byte);
        impronta = impronta.wrapping_mul(0x100000001b3);
    }
    impronta
}

#[test]
fn le_migrazioni_gia_rilasciate_non_cambiano() {
    let cartella = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    for (nome, attesa) in MIGRAZIONI_RILASCIATE {
        let contenuto = std::fs::read_to_string(cartella.join(nome)).unwrap();
        // Su Windows git può scrivere i file con CRLF: l'impronta usa sempre LF
        let senza_ritorni = contenuto.replace('\r', "");
        assert_eq!(
            impronta(senza_ritorni.as_bytes()),
            attesa,
            "{nome} è già stata rilasciata e non va modificata: crearne una nuova"
        );
    }
}
