use tauri::State;

use crate::db::iscritti::{self, DatiIscritto, FiltroIscritti, Iscritto, riferimento_per_oggi};
use crate::errore::ErroreApp;
use crate::stato_app::StatoApp;

#[tauri::command]
pub fn elenca_iscritti(
    stato: State<'_, StatoApp>,
    filtro: FiltroIscritti,
) -> Result<Vec<Iscritto>, ErroreApp> {
    let oggi = stato.oggi();
    stato.con_connessione(|connessione| {
        let riferimento = riferimento_per_oggi(connessione, oggi)?;
        iscritti::elenca_iscritti(connessione, &filtro, riferimento)
    })
}

#[tauri::command]
pub fn trova_iscritto(stato: State<'_, StatoApp>, id: i64) -> Result<Iscritto, ErroreApp> {
    let oggi = stato.oggi();
    stato.con_connessione(|connessione| {
        let riferimento = riferimento_per_oggi(connessione, oggi)?;
        iscritti::trova_iscritto(connessione, id, riferimento)
    })
}

#[tauri::command]
pub fn crea_iscritto(
    stato: State<'_, StatoApp>,
    dati: DatiIscritto,
) -> Result<Iscritto, ErroreApp> {
    let oggi = stato.oggi();
    stato.con_connessione(|connessione| {
        let riferimento = riferimento_per_oggi(connessione, oggi)?;
        iscritti::crea_iscritto(connessione, &dati, riferimento)
    })
}

#[tauri::command]
pub fn modifica_iscritto(
    stato: State<'_, StatoApp>,
    id: i64,
    dati: DatiIscritto,
) -> Result<Iscritto, ErroreApp> {
    let oggi = stato.oggi();
    stato.con_connessione(|connessione| {
        let riferimento = riferimento_per_oggi(connessione, oggi)?;
        iscritti::modifica_iscritto(connessione, id, &dati, riferimento)
    })
}

#[tauri::command]
pub fn imposta_iscritto_archiviato(
    stato: State<'_, StatoApp>,
    id: i64,
    archiviato: bool,
) -> Result<Iscritto, ErroreApp> {
    let oggi = stato.oggi();
    stato.con_connessione(|connessione| {
        let riferimento = riferimento_per_oggi(connessione, oggi)?;
        iscritti::imposta_iscritto_archiviato(connessione, id, archiviato, riferimento)
    })
}
