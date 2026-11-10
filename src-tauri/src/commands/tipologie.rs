use tauri::State;

use crate::db::tipologie::{self, DatiTipologia, TipologiaAbbonamento};
use crate::errore::ErroreApp;
use crate::stato_app::StatoApp;

#[tauri::command]
pub fn elenca_tipologie(
    stato: State<'_, StatoApp>,
    solo_attive: bool,
) -> Result<Vec<TipologiaAbbonamento>, ErroreApp> {
    stato.con_connessione(|connessione| tipologie::elenca_tipologie(connessione, solo_attive))
}

#[tauri::command]
pub fn crea_tipologia(
    stato: State<'_, StatoApp>,
    dati: DatiTipologia,
) -> Result<TipologiaAbbonamento, ErroreApp> {
    stato.con_connessione(|connessione| tipologie::crea_tipologia(connessione, &dati))
}

#[tauri::command]
pub fn modifica_tipologia(
    stato: State<'_, StatoApp>,
    id: i64,
    dati: DatiTipologia,
) -> Result<TipologiaAbbonamento, ErroreApp> {
    stato.con_connessione(|connessione| tipologie::modifica_tipologia(connessione, id, &dati))
}

#[tauri::command]
pub fn imposta_tipologia_attiva(
    stato: State<'_, StatoApp>,
    id: i64,
    attiva: bool,
) -> Result<TipologiaAbbonamento, ErroreApp> {
    stato
        .con_connessione(|connessione| tipologie::imposta_tipologia_attiva(connessione, id, attiva))
}
