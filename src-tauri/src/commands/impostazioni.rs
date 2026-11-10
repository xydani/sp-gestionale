use tauri::State;
use zeroize::Zeroizing;

use crate::db::impostazioni::{self, Impostazioni};
use crate::errore::ErroreApp;
use crate::stato_app::StatoApp;

#[tauri::command]
pub fn leggi_impostazioni(stato: State<'_, StatoApp>) -> Result<Impostazioni, ErroreApp> {
    stato.con_connessione(impostazioni::leggi_impostazioni)
}

#[tauri::command]
pub fn salva_impostazioni(
    stato: State<'_, StatoApp>,
    impostazioni: Impostazioni,
) -> Result<Impostazioni, ErroreApp> {
    stato.con_connessione_modificabile(|connessione| {
        impostazioni::salva_impostazioni(connessione, &impostazioni)
    })
}

#[tauri::command]
pub fn configurazione_iniziale_completata(stato: State<'_, StatoApp>) -> Result<bool, ErroreApp> {
    stato.con_connessione(impostazioni::configurazione_iniziale_completata)
}

#[tauri::command]
pub fn completa_configurazione_iniziale(stato: State<'_, StatoApp>) -> Result<(), ErroreApp> {
    stato.con_connessione(impostazioni::completa_configurazione_iniziale)
}

#[tauri::command]
pub fn cambia_password(
    stato: State<'_, StatoApp>,
    password_attuale: String,
    nuova_password: String,
    conferma_password: String,
) -> Result<(), ErroreApp> {
    stato.cambia_password(
        Zeroizing::new(password_attuale),
        Zeroizing::new(nuova_password),
        Zeroizing::new(conferma_password),
    )
}
