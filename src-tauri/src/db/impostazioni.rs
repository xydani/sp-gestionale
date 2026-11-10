use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

use super::ErroreDatabase;
use crate::domain::anno_sportivo::MESE_INIZIO_ANNO_SPORTIVO_PREDEFINITO;
use crate::domain::stato::GIORNI_PREAVVISO_PREDEFINITI;

pub const CHIAVE_BACKUP_CONSERVATI: &str = "backup_conservati";
pub const BACKUP_CONSERVATI_PREDEFINITI: usize = 30;
pub const CHIAVE_GIORNI_PREAVVISO: &str = "giorni_preavviso";
pub const CHIAVE_MESE_INIZIO_ANNO_SPORTIVO: &str = "mese_inizio_anno_sportivo";
pub const CHIAVE_QUOTA_ISCRIZIONE_CENT: &str = "quota_iscrizione_cent";
pub const QUOTA_ISCRIZIONE_PREDEFINITA_CENT: usize = 3000;

/// Legge un'impostazione numerica; se manca o non è un numero usa il valore predefinito.
pub fn leggi_impostazione_numerica(
    connessione: &Connection,
    chiave: &str,
    predefinito: usize,
) -> Result<usize, ErroreDatabase> {
    Ok(leggi_testo(connessione, chiave)?
        .and_then(|testo| testo.parse().ok())
        .unwrap_or(predefinito))
}

pub const CHIAVE_NOME_PALESTRA: &str = "nome_palestra";
pub const CHIAVE_CONFIGURAZIONE_INIZIALE_COMPLETATA: &str = "configurazione_iniziale_completata";

pub const GIORNI_PREAVVISO_MASSIMI: u32 = 60;
pub const BACKUP_CONSERVATI_MASSIMI: u32 = 365;
pub const QUOTA_ISCRIZIONE_MASSIMA_CENT: u32 = 1_000_000;
pub const LUNGHEZZA_MASSIMA_NOME_PALESTRA: usize = 80;

/// Opzioni del programma modificabili dallo staff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Impostazioni {
    pub nome_palestra: String,
    pub giorni_preavviso: u32,
    pub quota_iscrizione_cent: u32,
    pub mese_inizio_anno_sportivo: u32,
    pub backup_conservati: u32,
}

pub fn leggi_impostazioni(connessione: &Connection) -> Result<Impostazioni, ErroreDatabase> {
    let numero = |chiave, predefinito: u32| -> Result<u32, ErroreDatabase> {
        Ok(leggi_impostazione_numerica(connessione, chiave, predefinito as usize)? as u32)
    };
    Ok(Impostazioni {
        nome_palestra: leggi_testo(connessione, CHIAVE_NOME_PALESTRA)?.unwrap_or_default(),
        giorni_preavviso: numero(CHIAVE_GIORNI_PREAVVISO, GIORNI_PREAVVISO_PREDEFINITI)?,
        quota_iscrizione_cent: numero(
            CHIAVE_QUOTA_ISCRIZIONE_CENT,
            QUOTA_ISCRIZIONE_PREDEFINITA_CENT as u32,
        )?,
        mese_inizio_anno_sportivo: numero(
            CHIAVE_MESE_INIZIO_ANNO_SPORTIVO,
            MESE_INIZIO_ANNO_SPORTIVO_PREDEFINITO,
        )?,
        backup_conservati: numero(
            CHIAVE_BACKUP_CONSERVATI,
            BACKUP_CONSERVATI_PREDEFINITI as u32,
        )?,
    })
}

/// Salva tutte le impostazioni.
pub fn salva_impostazioni(
    connessione: &mut Connection,
    impostazioni: &Impostazioni,
) -> Result<Impostazioni, ErroreDatabase> {
    let impostazioni = valida_impostazioni(impostazioni)?;
    let transazione = connessione.transaction()?;
    for (chiave, valore) in [
        (CHIAVE_NOME_PALESTRA, impostazioni.nome_palestra.clone()),
        (
            CHIAVE_GIORNI_PREAVVISO,
            impostazioni.giorni_preavviso.to_string(),
        ),
        (
            CHIAVE_QUOTA_ISCRIZIONE_CENT,
            impostazioni.quota_iscrizione_cent.to_string(),
        ),
        (
            CHIAVE_MESE_INIZIO_ANNO_SPORTIVO,
            impostazioni.mese_inizio_anno_sportivo.to_string(),
        ),
        (
            CHIAVE_BACKUP_CONSERVATI,
            impostazioni.backup_conservati.to_string(),
        ),
    ] {
        scrivi(&transazione, chiave, &valore)?;
    }
    transazione.commit()?;
    Ok(impostazioni)
}

pub fn configurazione_iniziale_completata(
    connessione: &Connection,
) -> Result<bool, ErroreDatabase> {
    Ok(
        leggi_impostazione_numerica(connessione, CHIAVE_CONFIGURAZIONE_INIZIALE_COMPLETATA, 1)?
            == 1,
    )
}

/// Chiamata all'ultimo passo della configurazione guidata: da qui il programma è utilizzabile.
pub fn completa_configurazione_iniziale(connessione: &Connection) -> Result<(), ErroreDatabase> {
    scrivi(connessione, CHIAVE_CONFIGURAZIONE_INIZIALE_COMPLETATA, "1")
}

/// Un database appena creato parte dalla configurazione guidata.
pub fn richiedi_configurazione_iniziale(connessione: &Connection) -> Result<(), ErroreDatabase> {
    scrivi(connessione, CHIAVE_CONFIGURAZIONE_INIZIALE_COMPLETATA, "0")
}

fn valida_impostazioni(impostazioni: &Impostazioni) -> Result<Impostazioni, ErroreDatabase> {
    let non_valido = |messaggio: String| Err(ErroreDatabase::DatiNonValidi(messaggio));
    let nome_palestra = impostazioni.nome_palestra.trim().to_string();
    if nome_palestra.chars().count() > LUNGHEZZA_MASSIMA_NOME_PALESTRA {
        return non_valido(format!(
            "Il nome della palestra può avere al massimo {LUNGHEZZA_MASSIMA_NOME_PALESTRA} caratteri"
        ));
    }
    if !(1..=GIORNI_PREAVVISO_MASSIMI).contains(&impostazioni.giorni_preavviso) {
        return non_valido(format!(
            "I giorni di preavviso devono essere tra 1 e {GIORNI_PREAVVISO_MASSIMI}"
        ));
    }
    if impostazioni.quota_iscrizione_cent > QUOTA_ISCRIZIONE_MASSIMA_CENT {
        return non_valido("La quota di iscrizione è troppo alta".into());
    }
    if !(1..=12).contains(&impostazioni.mese_inizio_anno_sportivo) {
        return non_valido("Il mese di inizio dell'anno sportivo non è valido".into());
    }
    if !(1..=BACKUP_CONSERVATI_MASSIMI).contains(&impostazioni.backup_conservati) {
        return non_valido(format!(
            "I backup conservati devono essere tra 1 e {BACKUP_CONSERVATI_MASSIMI}"
        ));
    }
    Ok(Impostazioni {
        nome_palestra,
        ..impostazioni.clone()
    })
}

fn leggi_testo(connessione: &Connection, chiave: &str) -> Result<Option<String>, ErroreDatabase> {
    Ok(connessione
        .query_row(
            "SELECT valore FROM impostazioni WHERE chiave = ?1",
            [chiave],
            |riga| riga.get(0),
        )
        .optional()?)
}

fn scrivi(connessione: &Connection, chiave: &str, valore: &str) -> Result<(), ErroreDatabase> {
    connessione.execute(
        "INSERT INTO impostazioni (chiave, valore) VALUES (?1, ?2)
         ON CONFLICT (chiave) DO UPDATE SET valore = excluded.valore",
        [chiave, valore],
    )?;
    Ok(())
}
