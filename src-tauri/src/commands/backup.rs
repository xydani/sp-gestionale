use std::path::PathBuf;

use tauri::State;
use zeroize::Zeroizing;

use crate::errore::ErroreApp;
use crate::stato_app::StatoApp;

/// Restituisce il percorso del file di backup creato.
#[tauri::command]
pub fn esegui_backup_manuale(
    stato: State<'_, StatoApp>,
    cartella_destinazione: PathBuf,
) -> Result<PathBuf, ErroreApp> {
    stato.esegui_backup_manuale(&cartella_destinazione)
}

/// Backup prima di installare un aggiornamento: se qualcosa va storto i dati sono al sicuro.
#[tauri::command]
pub fn esegui_backup_pre_aggiornamento(
    app: tauri::AppHandle,
    stato: State<'_, StatoApp>,
) -> Result<PathBuf, ErroreApp> {
    stato.esegui_backup_pre_aggiornamento(&app.package_info().version.to_string())
}

#[tauri::command]
pub fn ripristina_backup(
    stato: State<'_, StatoApp>,
    percorso_backup: PathBuf,
    password_backup: String,
) -> Result<(), ErroreApp> {
    stato.ripristina_backup(&percorso_backup, Zeroizing::new(password_backup))
}

#[tauri::command]
pub fn cartella_backup_automatici(stato: State<'_, StatoApp>) -> PathBuf {
    stato.cartella_backup().to_path_buf()
}
