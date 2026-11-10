//! Backup criptati del database e loro rotazione.

use std::fs;
use std::path::{Path, PathBuf};

use chrono::{NaiveDate, NaiveDateTime};
use rusqlite::Connection;

use super::ErroreDatabase;
use super::crittografia::apri_database_criptato;

pub const PREFISSO_BACKUP_GIORNALIERO: &str = "backup-";
pub const PREFISSO_BACKUP_PRE_MIGRAZIONE: &str = "pre-migrazione-";
pub const PREFISSO_BACKUP_PRE_RIPRISTINO: &str = "pre-ripristino-";
pub const PREFISSO_BACKUP_MANUALE: &str = "sp-gestionale-";
const ESTENSIONE_BACKUP: &str = ".db";
const FORMATO_ISTANTE: &str = "%Y-%m-%d_%H-%M-%S";
const FORMATO_GIORNO: &str = "%Y-%m-%d";

/// Copia l'intero database in un nuovo file criptato con la stessa password.
///
/// Si usa `sqlcipher_export` e non `VACUUM INTO`, che con SQLCipher non garantisce
/// la cifratura della copia. Il database allegato senza KEY eredita la chiave principale.
pub fn esegui_backup(connessione: &Connection, destinazione: &Path) -> Result<(), ErroreDatabase> {
    if destinazione.exists() {
        return Err(ErroreDatabase::DatabaseGiaEsistente);
    }
    let percorso = destinazione.to_string_lossy();
    connessione.execute("ATTACH DATABASE ?1 AS copia_backup", [percorso.as_ref()])?;
    let esito = esporta_in_copia_allegata(connessione);
    // Il DETACH va eseguito anche se l'esportazione fallisce
    connessione.execute("DETACH DATABASE copia_backup", [])?;
    if esito.is_err() {
        let _ = fs::remove_file(destinazione);
    }
    esito?;
    Ok(())
}

/// `sqlcipher_export` copia schema e dati ma non `user_version`: va copiata a parte,
/// altrimenti un backup ripristinato rieseguirebbe tutte le migrazioni.
fn esporta_in_copia_allegata(connessione: &Connection) -> rusqlite::Result<()> {
    connessione.query_row("SELECT sqlcipher_export('copia_backup')", [], |_| Ok(()))?;
    let versione_schema: u32 =
        connessione.pragma_query_value(None, "user_version", |riga| riga.get(0))?;
    connessione.pragma_update(Some("copia_backup"), "user_version", versione_schema)
}

pub fn nome_backup_giornaliero(istante: NaiveDateTime) -> String {
    format!(
        "{PREFISSO_BACKUP_GIORNALIERO}{}{ESTENSIONE_BACKUP}",
        istante.format(FORMATO_ISTANTE)
    )
}

pub fn nome_backup_pre_migrazione(versione_corrente: u32, istante: NaiveDateTime) -> String {
    format!(
        "{PREFISSO_BACKUP_PRE_MIGRAZIONE}v{versione_corrente}-{}{ESTENSIONE_BACKUP}",
        istante.format(FORMATO_ISTANTE)
    )
}

pub fn nome_backup_pre_ripristino(istante: NaiveDateTime) -> String {
    format!(
        "{PREFISSO_BACKUP_PRE_RIPRISTINO}{}{ESTENSIONE_BACKUP}",
        istante.format(FORMATO_ISTANTE)
    )
}

pub fn nome_backup_manuale(istante: NaiveDateTime) -> String {
    format!(
        "{PREFISSO_BACKUP_MANUALE}{}{ESTENSIONE_BACKUP}",
        istante.format(FORMATO_ISTANTE)
    )
}

/// Vero se nella cartella non c'è ancora un backup giornaliero del giorno indicato.
pub fn serve_backup_giornaliero(cartella_backup: &Path, oggi: NaiveDate) -> bool {
    let prefisso_di_oggi = format!(
        "{PREFISSO_BACKUP_GIORNALIERO}{}",
        oggi.format(FORMATO_GIORNO)
    );
    !backup_giornalieri(cartella_backup)
        .iter()
        .any(|percorso| nome_file(percorso).starts_with(&prefisso_di_oggi))
}

/// Esegue il backup giornaliero se non è già stato fatto oggi e cancella i più vecchi.
/// Restituisce il percorso del backup creato, se creato.
pub fn esegui_backup_giornaliero(
    connessione: &Connection,
    cartella_backup: &Path,
    adesso: NaiveDateTime,
    backup_da_conservare: usize,
) -> Result<Option<PathBuf>, ErroreDatabase> {
    if !serve_backup_giornaliero(cartella_backup, adesso.date()) {
        return Ok(None);
    }
    crea_cartella(cartella_backup)?;
    let destinazione = cartella_backup.join(nome_backup_giornaliero(adesso));
    esegui_backup(connessione, &destinazione)?;
    elimina_backup_giornalieri_in_eccesso(cartella_backup, backup_da_conservare)?;
    Ok(Some(destinazione))
}

/// Tiene solo gli ultimi `da_conservare` backup giornalieri; gli altri tipi non vengono toccati.
pub fn elimina_backup_giornalieri_in_eccesso(
    cartella_backup: &Path,
    da_conservare: usize,
) -> Result<(), ErroreDatabase> {
    let mut backup = backup_giornalieri(cartella_backup);
    // Il nome contiene la data in formato ordinabile: l'ordine alfabetico è cronologico
    backup.sort();
    let da_eliminare = backup.len().saturating_sub(da_conservare);
    for percorso in &backup[..da_eliminare] {
        fs::remove_file(percorso).map_err(errore_file)?;
    }
    Ok(())
}

/// Verifica che il file sia un backup valido apribile con la password indicata.
pub fn verifica_backup(percorso_backup: &Path, password: &str) -> Result<(), ErroreDatabase> {
    apri_database_criptato(percorso_backup, password).map(drop)
}

/// Sostituisce il file del database con il backup. La connessione al database va chiusa prima.
pub fn sostituisci_database_con_backup(
    percorso_backup: &Path,
    percorso_database: &Path,
) -> Result<(), ErroreDatabase> {
    for suffisso in ["-wal", "-shm"] {
        let file_ausiliario = PathBuf::from(format!("{}{suffisso}", percorso_database.display()));
        if file_ausiliario.exists() {
            fs::remove_file(file_ausiliario).map_err(errore_file)?;
        }
    }
    fs::copy(percorso_backup, percorso_database).map_err(errore_file)?;
    Ok(())
}

pub fn crea_cartella(cartella: &Path) -> Result<(), ErroreDatabase> {
    fs::create_dir_all(cartella).map_err(errore_file)
}

fn backup_giornalieri(cartella_backup: &Path) -> Vec<PathBuf> {
    let Ok(voci) = fs::read_dir(cartella_backup) else {
        return Vec::new();
    };
    voci.filter_map(Result::ok)
        .map(|voce| voce.path())
        .filter(|percorso| {
            let nome = nome_file(percorso);
            nome.starts_with(PREFISSO_BACKUP_GIORNALIERO) && nome.ends_with(ESTENSIONE_BACKUP)
        })
        .collect()
}

fn nome_file(percorso: &Path) -> String {
    percorso
        .file_name()
        .map(|nome| nome.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn errore_file(errore: std::io::Error) -> ErroreDatabase {
    ErroreDatabase::File(errore.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn istante(testo: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(testo, "%Y-%m-%d %H:%M:%S").unwrap()
    }

    #[test]
    fn i_nomi_dei_backup_contengono_data_e_ora() {
        let adesso = istante("2026-11-10 14:05:09");
        assert_eq!(
            nome_backup_giornaliero(adesso),
            "backup-2026-11-10_14-05-09.db"
        );
        assert_eq!(
            nome_backup_pre_migrazione(1, adesso),
            "pre-migrazione-v1-2026-11-10_14-05-09.db"
        );
        assert_eq!(
            nome_backup_pre_ripristino(adesso),
            "pre-ripristino-2026-11-10_14-05-09.db"
        );
        assert_eq!(
            nome_backup_manuale(adesso),
            "sp-gestionale-2026-11-10_14-05-09.db"
        );
    }

    #[test]
    fn serve_il_backup_se_la_cartella_non_esiste() {
        let cartella = tempfile::tempdir().unwrap();
        let oggi = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
        assert!(serve_backup_giornaliero(
            &cartella.path().join("inesistente"),
            oggi
        ));
    }

    #[test]
    fn non_serve_il_backup_se_ce_gia_quello_di_oggi() {
        let cartella = tempfile::tempdir().unwrap();
        fs::write(cartella.path().join("backup-2026-11-10_08-00-00.db"), b"").unwrap();
        let oggi = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
        let domani = NaiveDate::from_ymd_opt(2026, 11, 11).unwrap();
        assert!(!serve_backup_giornaliero(cartella.path(), oggi));
        assert!(serve_backup_giornaliero(cartella.path(), domani));
    }

    #[test]
    fn la_rotazione_tiene_solo_i_backup_giornalieri_piu_recenti() {
        let cartella = tempfile::tempdir().unwrap();
        for giorno in 1..=5 {
            fs::write(
                cartella
                    .path()
                    .join(format!("backup-2026-11-0{giorno}_10-00-00.db")),
                b"",
            )
            .unwrap();
        }
        fs::write(
            cartella
                .path()
                .join("pre-migrazione-v1-2026-10-01_10-00-00.db"),
            b"",
        )
        .unwrap();

        elimina_backup_giornalieri_in_eccesso(cartella.path(), 2).unwrap();

        let mut rimasti: Vec<String> = fs::read_dir(cartella.path())
            .unwrap()
            .map(|voce| voce.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        rimasti.sort();
        assert_eq!(
            rimasti,
            [
                "backup-2026-11-04_10-00-00.db",
                "backup-2026-11-05_10-00-00.db",
                "pre-migrazione-v1-2026-10-01_10-00-00.db"
            ]
        );
    }
}
