mod comune;

use comune::{DatabaseTemporaneo, PASSWORD_TEST};
use sp_gestionale_lib::db::crittografia::crea_database_criptato;
use sp_gestionale_lib::db::impostazioni::{
    Impostazioni, configurazione_iniziale_completata, leggi_impostazioni,
    richiedi_configurazione_iniziale, salva_impostazioni,
};
use sp_gestionale_lib::db::migrazioni::{applica_migrazioni, applica_migrazioni_fino_a};

fn impostazioni_valide() -> Impostazioni {
    Impostazioni {
        nome_palestra: "  Palestra Esempio  ".into(),
        giorni_preavviso: 10,
        quota_iscrizione_cent: 4000,
        mese_inizio_anno_sportivo: 10,
        backup_conservati: 15,
    }
}

#[test]
fn un_database_nuovo_ha_le_impostazioni_predefinite() {
    let database = DatabaseTemporaneo::nuovo();
    let connessione = database.crea_con_schema();
    assert_eq!(
        leggi_impostazioni(&connessione).unwrap(),
        Impostazioni {
            nome_palestra: String::new(),
            giorni_preavviso: 7,
            quota_iscrizione_cent: 3000,
            mese_inizio_anno_sportivo: 9,
            backup_conservati: 30,
        }
    );
}

#[test]
fn le_impostazioni_salvate_si_rileggono_con_il_nome_ripulito() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let salvate = salva_impostazioni(&mut connessione, &impostazioni_valide()).unwrap();
    assert_eq!(salvate.nome_palestra, "Palestra Esempio");
    assert_eq!(leggi_impostazioni(&connessione).unwrap(), salvate);
}

#[test]
fn i_valori_fuori_intervallo_sono_rifiutati() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    let varianti: [fn(&mut Impostazioni); 7] = [
        |i| i.giorni_preavviso = 0,
        |i| i.giorni_preavviso = 61,
        |i| i.mese_inizio_anno_sportivo = 0,
        |i| i.mese_inizio_anno_sportivo = 13,
        |i| i.backup_conservati = 0,
        |i| i.quota_iscrizione_cent = 1_000_001,
        |i| i.nome_palestra = "x".repeat(81),
    ];
    for modifica in varianti {
        let mut impostazioni = impostazioni_valide();
        modifica(&mut impostazioni);
        assert!(salva_impostazioni(&mut connessione, &impostazioni).is_err());
    }
    assert_eq!(
        leggi_impostazioni(&connessione).unwrap().giorni_preavviso,
        7
    );
}

#[test]
fn il_salvataggio_completa_la_configurazione_iniziale() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = database.crea_con_schema();
    richiedi_configurazione_iniziale(&connessione).unwrap();
    assert!(!configurazione_iniziale_completata(&connessione).unwrap());
    salva_impostazioni(&mut connessione, &impostazioni_valide()).unwrap();
    assert!(configurazione_iniziale_completata(&connessione).unwrap());
}

#[test]
fn un_database_gia_in_uso_non_ripete_la_configurazione() {
    let database = DatabaseTemporaneo::nuovo();
    let mut connessione = crea_database_criptato(&database.percorso, PASSWORD_TEST).unwrap();
    applica_migrazioni_fino_a(&mut connessione, 3).unwrap();
    applica_migrazioni(&mut connessione).unwrap();
    assert!(configurazione_iniziale_completata(&connessione).unwrap());
}
