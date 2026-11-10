//! Crea un database criptato con dati fittizi per provare l'app.
//!
//! Uso: genera_dati_prova <percorso.db> [password] [numero_iscritti]

use std::path::PathBuf;
use std::process::ExitCode;

use sp_gestionale_lib::dati_prova::{NUMERO_ISCRITTI_PREDEFINITO, genera_dati_prova};
use sp_gestionale_lib::db::crittografia::crea_database_criptato;
use sp_gestionale_lib::db::migrazioni::applica_migrazioni;

const PASSWORD_PREDEFINITA: &str = "password-prova";
const SEME: u64 = 2026;

fn main() -> ExitCode {
    let argomenti: Vec<String> = std::env::args().skip(1).collect();
    let Some(percorso) = argomenti.first().map(PathBuf::from) else {
        eprintln!("Uso: genera_dati_prova <percorso.db> [password] [numero_iscritti]");
        return ExitCode::FAILURE;
    };
    let password = argomenti
        .get(1)
        .map_or(PASSWORD_PREDEFINITA, String::as_str);
    let numero_iscritti = match argomenti.get(2).map(|valore| valore.parse::<u32>()) {
        None => NUMERO_ISCRITTI_PREDEFINITO,
        Some(Ok(numero)) => numero,
        Some(Err(_)) => {
            eprintln!("Il numero di iscritti deve essere un intero positivo");
            return ExitCode::FAILURE;
        }
    };

    if let Some(cartella) = percorso.parent() {
        let _ = std::fs::create_dir_all(cartella);
    }

    let oggi = chrono::Local::now().date_naive();
    let risultato = crea_database_criptato(&percorso, password).and_then(|mut connessione| {
        applica_migrazioni(&mut connessione)?;
        genera_dati_prova(&mut connessione, numero_iscritti, oggi, SEME)
    });

    match risultato {
        Ok(()) => {
            println!(
                "Creato {} con {numero_iscritti} iscritti (password: {password})",
                percorso.display()
            );
            ExitCode::SUCCESS
        }
        Err(errore) => {
            eprintln!("Errore: {errore}");
            ExitCode::FAILURE
        }
    }
}
