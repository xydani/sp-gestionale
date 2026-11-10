use std::path::PathBuf;

use chrono::NaiveDate;
use tauri::State;

use crate::db::ErroreDatabase;
use crate::db::report::{self, Raggruppamento, RiepilogoIncassi};
use crate::errore::ErroreApp;
use crate::stato_app::StatoApp;

#[tauri::command]
pub fn riepilogo_incassi(
    stato: State<'_, StatoApp>,
    dal: NaiveDate,
    al: NaiveDate,
    raggruppamento: Raggruppamento,
) -> Result<RiepilogoIncassi, ErroreApp> {
    stato.con_connessione(|connessione| {
        report::riepilogo_incassi(connessione, dal, al, raggruppamento)
    })
}

#[tauri::command]
pub fn esporta_incassi_csv(
    stato: State<'_, StatoApp>,
    dal: NaiveDate,
    al: NaiveDate,
    percorso: PathBuf,
) -> Result<(), ErroreApp> {
    stato.con_connessione(|connessione| {
        let testo = report::esporta_incassi_csv(connessione, dal, al)?;
        std::fs::write(&percorso, testo).map_err(|errore| ErroreDatabase::File(errore.to_string()))
    })
}
