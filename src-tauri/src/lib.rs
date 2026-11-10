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
        ])
        .run(tauri::generate_context!())
        .expect("errore durante l'avvio dell'applicazione");
}
