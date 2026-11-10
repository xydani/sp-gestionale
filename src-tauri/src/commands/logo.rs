use std::path::PathBuf;

use tauri::State;

use crate::errore::ErroreApp;
use crate::logo_palestra;
use crate::stato_app::StatoApp;

/// Disponibile anche ad archivio bloccato: il logo si vede nella schermata della password.
#[tauri::command]
pub fn leggi_logo_palestra(stato: State<'_, StatoApp>) -> Result<Option<String>, ErroreApp> {
    logo_palestra::leggi_logo(stato.cartella_dati())
}

#[tauri::command]
pub fn imposta_logo_palestra(
    stato: State<'_, StatoApp>,
    percorso: PathBuf,
) -> Result<(), ErroreApp> {
    stato.verifica_sbloccato()?;
    logo_palestra::salva_logo(stato.cartella_dati(), &percorso)
}

#[tauri::command]
pub fn rimuovi_logo_palestra(stato: State<'_, StatoApp>) -> Result<(), ErroreApp> {
    stato.verifica_sbloccato()?;
    logo_palestra::rimuovi_logo(stato.cartella_dati())
}
