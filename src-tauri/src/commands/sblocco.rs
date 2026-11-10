use tauri::State;
use zeroize::Zeroizing;

use crate::errore::ErroreApp;
use crate::stato_app::{EsitoSblocco, StatoApp, StatoDatabase};

#[tauri::command]
pub fn stato_database(stato: State<'_, StatoApp>) -> StatoDatabase {
    stato.stato_database()
}

#[tauri::command]
pub fn crea_database(
    stato: State<'_, StatoApp>,
    password: String,
    conferma_password: String,
) -> Result<(), ErroreApp> {
    stato.crea_database(Zeroizing::new(password), Zeroizing::new(conferma_password))
}

#[tauri::command]
pub fn sblocca_database(
    stato: State<'_, StatoApp>,
    password: String,
) -> Result<EsitoSblocco, ErroreApp> {
    stato.sblocca_database(Zeroizing::new(password))
}

#[tauri::command]
pub fn blocca_database(stato: State<'_, StatoApp>) {
    stato.blocca_database();
}
