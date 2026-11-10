use tauri::State;

use crate::db::iscrizioni::{self, DatiIscrizione, Iscrizione, PropostaIscrizione};
use crate::errore::ErroreApp;
use crate::stato_app::StatoApp;

#[tauri::command]
pub fn proponi_iscrizione(
    stato: State<'_, StatoApp>,
    iscritto_id: Option<i64>,
) -> Result<PropostaIscrizione, ErroreApp> {
    let oggi = stato.oggi();
    stato.con_connessione(|connessione| {
        iscrizioni::proponi_iscrizione(connessione, iscritto_id, oggi)
    })
}

#[tauri::command]
pub fn registra_iscrizione(
    stato: State<'_, StatoApp>,
    dati: DatiIscrizione,
) -> Result<Iscrizione, ErroreApp> {
    let oggi = stato.oggi();
    stato.con_connessione(|connessione| iscrizioni::registra_iscrizione(connessione, &dati, oggi))
}

#[tauri::command]
pub fn elenca_iscrizioni(
    stato: State<'_, StatoApp>,
    iscritto_id: i64,
) -> Result<Vec<Iscrizione>, ErroreApp> {
    stato.con_connessione(|connessione| iscrizioni::elenca_iscrizioni(connessione, iscritto_id))
}

#[tauri::command]
pub fn elimina_iscrizione(stato: State<'_, StatoApp>, id: i64) -> Result<(), ErroreApp> {
    stato.con_connessione(|connessione| iscrizioni::elimina_iscrizione(connessione, id))
}
