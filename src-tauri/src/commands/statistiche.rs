use chrono::NaiveDate;
use tauri::State;

use crate::db::statistiche::{self, Statistiche};
use crate::errore::ErroreApp;
use crate::stato_app::StatoApp;

/// Tutti i dati della pagina Statistiche per il periodo scelto.
// Operazione lenta su archivi grandi: gira fuori dal thread principale
#[tauri::command(async)]
pub fn leggi_statistiche(
    stato: State<'_, StatoApp>,
    dal: NaiveDate,
    al: NaiveDate,
) -> Result<Statistiche, ErroreApp> {
    let oggi = stato.oggi();
    stato.con_connessione(|connessione| statistiche::leggi_statistiche(connessione, dal, al, oggi))
}
