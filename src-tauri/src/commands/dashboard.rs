use tauri::State;

use crate::db::dashboard::{self, RiepilogoDashboard};
use crate::db::iscritti::riferimento_per_oggi;
use crate::errore::ErroreApp;
use crate::stato_app::StatoApp;

#[tauri::command]
pub fn riepilogo_dashboard(stato: State<'_, StatoApp>) -> Result<RiepilogoDashboard, ErroreApp> {
    let oggi = stato.oggi();
    stato.con_connessione(|connessione| {
        let riferimento = riferimento_per_oggi(connessione, oggi)?;
        dashboard::riepilogo_dashboard(connessione, riferimento)
    })
}
