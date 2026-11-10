use tauri::State;

use crate::db::iscritti::riferimento_per_oggi;
use crate::db::promemoria::{self, ConteggioInScadenza, EsitoPromemoria};
use crate::errore::ErroreApp;
use crate::stato_app::StatoApp;

/// Chiamato all'apertura del programma e poi periodicamente dall'interfaccia.
#[tauri::command]
pub fn controlla_promemoria(stato: State<'_, StatoApp>) -> Result<EsitoPromemoria, ErroreApp> {
    let oggi = stato.oggi();
    stato.con_connessione_modificabile(|connessione| {
        let riferimento = riferimento_per_oggi(connessione, oggi)?;
        promemoria::controlla_promemoria(connessione, riferimento)
    })
}

#[tauri::command]
pub fn conta_in_scadenza(stato: State<'_, StatoApp>) -> Result<ConteggioInScadenza, ErroreApp> {
    let oggi = stato.oggi();
    stato.con_connessione(|connessione| {
        let riferimento = riferimento_per_oggi(connessione, oggi)?;
        promemoria::conta_in_scadenza(connessione, riferimento)
    })
}
