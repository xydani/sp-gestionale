pub mod commands;
pub mod dati_prova;
pub mod db;
pub mod domain;
pub mod errore;
pub mod stato_app;
pub mod versione;

use tauri::Manager;

use crate::stato_app::StatoApp;

const NOME_FILE_DATABASE: &str = "palestra.db";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let cartella_dati = app.path().app_data_dir()?;
            app.manage(StatoApp::nuovo(cartella_dati.join(NOME_FILE_DATABASE)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::sblocco::stato_database,
            commands::sblocco::crea_database,
            commands::sblocco::sblocca_database,
            commands::sblocco::blocca_database,
            commands::backup::esegui_backup_manuale,
            commands::backup::ripristina_backup,
            commands::backup::cartella_backup_automatici,
            commands::tipologie::elenca_tipologie,
            commands::tipologie::crea_tipologia,
            commands::tipologie::modifica_tipologia,
            commands::tipologie::imposta_tipologia_attiva,
            commands::iscritti::elenca_iscritti,
            commands::iscritti::trova_iscritto,
            commands::iscritti::crea_iscritto,
            commands::iscritti::modifica_iscritto,
            commands::iscritti::imposta_iscritto_archiviato,
            commands::iscrizioni::proponi_iscrizione,
            commands::iscrizioni::registra_iscrizione,
            commands::iscrizioni::elenca_iscrizioni,
            commands::iscrizioni::elimina_iscrizione,
            commands::pagamenti::proponi_pagamento,
            commands::pagamenti::registra_pagamento,
            commands::pagamenti::elenca_pagamenti,
            commands::pagamenti::elimina_pagamento,
            commands::promemoria::controlla_promemoria,
            commands::promemoria::conta_in_scadenza,
        ])
        // Le due finestre (accesso e principale) vivono insieme: chiuderne una chiude il programma
        .on_window_event(|finestra, evento| {
            if let tauri::WindowEvent::CloseRequested { .. } = evento {
                finestra.app_handle().exit(0);
            }
        })
        .run(tauri::generate_context!())
        .expect("errore durante l'avvio dell'applicazione");
}
