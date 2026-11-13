//! Inserimento rapido di chi era già iscritto quando si è iniziato a usare il programma:
//! anagrafica essenziale, abbonamento in corso e quota dell'anno con i loro importi.
//! Sono incassi veri: la quota conta nel mese di inizio dell'anno sportivo,
//! l'abbonamento nel mese in cui è iniziato.

use chrono::NaiveDate;
use rusqlite::{Connection, params};
use serde::Deserialize;

use super::ErroreDatabase;
use super::impostazioni::{CHIAVE_MESE_INIZIO_ANNO_SPORTIVO, leggi_impostazione_numerica};
use super::iscritti::{Iscritto, Riferimento, trova_iscritto};
use super::iscrizioni::{DatiIscrizione, registra_iscrizione};
use super::pagamenti::{DatiPagamento, registra_pagamento, trova_tipologia_in_vendita};
use crate::domain::anno_sportivo::{
    MESE_INIZIO_ANNO_SPORTIVO_PREDEFINITO, anno_sportivo_contenente,
};
use crate::domain::date::calcola_data_inizio_da_scadenza;
use crate::domain::pagamento::MetodoPagamento;
use crate::domain::testo::testo_facoltativo;
use crate::domain::validazione::{
    EsitoControllo, LUNGHEZZA_MASSIMA_NOME, controlla_abbonamento_pregresso,
    controlla_nome_proprio, controlla_obbligatorio, controlla_scadenza_certificato,
    controlla_telefono,
};

/// Dati minimi per inserire una persona già iscritta; il resto si completa dalla scheda.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatiIscrittoGiaAttivo {
    pub nome: String,
    pub cognome: String,
    pub telefono: Option<String>,
    #[serde(default)]
    pub data_scadenza_certificato: Option<NaiveDate>,
    pub tipologia_id: i64,
    pub data_scadenza_abbonamento: NaiveDate,
    pub importo_abbonamento_cent: i64,
    /// La quota dell'anno sportivo in corso è già stata pagata.
    pub quota_pagata: bool,
    /// Usata solo se la quota è stata pagata.
    pub quota_cent: i64,
    /// Come sono stati pagati abbonamento e quota.
    pub metodo: MetodoPagamento,
}

/// Crea l'iscritto con abbonamento in corso ed eventuale quota dell'anno:
/// se un dato non è valido non viene salvato nulla.
pub fn registra_iscritto_gia_attivo(
    connessione: &mut Connection,
    dati: &DatiIscrittoGiaAttivo,
    riferimento: Riferimento,
) -> Result<Iscritto, ErroreDatabase> {
    let transazione = connessione.transaction()?;
    let iscritto_id = crea_anagrafica_essenziale(&transazione, dati, riferimento.oggi)?;
    registra_abbonamento_in_corso(&transazione, iscritto_id, dati, riferimento.oggi)?;
    if dati.quota_pagata {
        registra_quota_dell_anno(&transazione, iscritto_id, dati, riferimento.oggi)?;
    }
    let iscritto = trova_iscritto(&transazione, iscritto_id, riferimento)?;
    transazione.commit()?;
    Ok(iscritto)
}

/// Data e luogo di nascita restano vuoti: l'iscritto risulta da completare.
fn crea_anagrafica_essenziale(
    connessione: &Connection,
    dati: &DatiIscrittoGiaAttivo,
    oggi: NaiveDate,
) -> Result<i64, ErroreDatabase> {
    let nome = dati.nome.trim();
    let cognome = dati.cognome.trim();
    let telefono = testo_facoltativo(dati.telefono.as_deref());
    controlla_anagrafica_essenziale(nome, cognome, telefono.as_deref(), dati, oggi)
        .map_err(ErroreDatabase::non_valido)?;
    connessione.execute(
        "INSERT INTO iscritti (nome, cognome, telefono, data_scadenza_certificato)
         VALUES (?1, ?2, ?3, ?4)",
        params![nome, cognome, telefono, dati.data_scadenza_certificato],
    )?;
    Ok(connessione.last_insert_rowid())
}

fn controlla_anagrafica_essenziale(
    nome: &str,
    cognome: &str,
    telefono: Option<&str>,
    dati: &DatiIscrittoGiaAttivo,
    oggi: NaiveDate,
) -> EsitoControllo {
    controlla_obbligatorio(nome, "Il nome")?;
    controlla_obbligatorio(cognome, "Il cognome")?;
    controlla_nome_proprio(nome, "Il nome", LUNGHEZZA_MASSIMA_NOME)?;
    controlla_nome_proprio(cognome, "Il cognome", LUNGHEZZA_MASSIMA_NOME)?;
    if let Some(telefono) = telefono {
        controlla_telefono(telefono)?;
    }
    if let Some(scadenza) = dati.data_scadenza_certificato {
        controlla_scadenza_certificato(scadenza, oggi)?;
    }
    Ok(())
}

/// L'inizio si ricava dalla scadenza e dalla durata della tipologia; il pagamento
/// è datato a quel giorno. Il rinnovo successivo partirà dal giorno dopo la scadenza.
fn registra_abbonamento_in_corso(
    connessione: &Connection,
    iscritto_id: i64,
    dati: &DatiIscrittoGiaAttivo,
    oggi: NaiveDate,
) -> Result<(), ErroreDatabase> {
    let tipologia = trova_tipologia_in_vendita(connessione, dati.tipologia_id)?;
    let data_scadenza = dati.data_scadenza_abbonamento;
    let data_inizio = calcola_data_inizio_da_scadenza(data_scadenza, tipologia.durata())
        .map_err(|errore| ErroreDatabase::non_valido(errore.to_string()))?;
    controlla_abbonamento_pregresso(data_inizio, data_scadenza, oggi)
        .map_err(ErroreDatabase::non_valido)?;
    let pagamento = DatiPagamento {
        iscritto_id,
        tipologia_id: tipologia.id,
        importo_cent: dati.importo_abbonamento_cent,
        data_pagamento: data_inizio,
        metodo: dati.metodo,
        data_inizio,
        data_scadenza,
        note: None,
        rinnovo_di_id: None,
    };
    registra_pagamento(connessione, &pagamento, oggi)?;
    Ok(())
}

/// Copre l'anno sportivo in corso ed è datata al suo primo giorno (di solito il 1° settembre).
fn registra_quota_dell_anno(
    connessione: &Connection,
    iscritto_id: i64,
    dati: &DatiIscrittoGiaAttivo,
    oggi: NaiveDate,
) -> Result<(), ErroreDatabase> {
    let mese_inizio = leggi_impostazione_numerica(
        connessione,
        CHIAVE_MESE_INIZIO_ANNO_SPORTIVO,
        MESE_INIZIO_ANNO_SPORTIVO_PREDEFINITO as usize,
    )?;
    let anno_sportivo = anno_sportivo_contenente(oggi, mese_inizio as u32);
    let iscrizione = DatiIscrizione {
        iscritto_id,
        importo_cent: dati.quota_cent,
        data_pagamento: anno_sportivo.inizio,
        metodo: dati.metodo,
        data_inizio: anno_sportivo.inizio,
        data_fine: anno_sportivo.fine,
        note: None,
    };
    registra_iscrizione(connessione, &iscrizione, oggi)?;
    Ok(())
}
