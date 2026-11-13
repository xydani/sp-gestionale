//! Anagrafica degli iscritti, con lo stato di iscrizione annuale e abbonamento.

use chrono::NaiveDate;
use rusqlite::types::Value;
use rusqlite::{Connection, OptionalExtension, Row, params, params_from_iter};
use serde::{Deserialize, Serialize};

use super::ErroreDatabase;
use super::impostazioni::{CHIAVE_GIORNI_PREAVVISO, leggi_impostazione_numerica};
use super::paginazione::{Pagina, RichiestaPagina};
use crate::domain::stato::{
    CondizioneScadenza, GIORNI_PREAVVISO_PREDEFINITI, StatoValidita, calcola_stato_validita,
    condizione_scadenza_per_stato,
};
use crate::domain::testo::{parole_di_ricerca, testo_facoltativo};
use crate::domain::validazione::{
    EsitoControllo, LUNGHEZZA_MASSIMA_LUOGO, LUNGHEZZA_MASSIMA_NOME, controlla_data_nascita,
    controlla_email, controlla_nome_proprio, controlla_note, controlla_obbligatorio,
    controlla_scadenza_certificato, controlla_telefono,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Iscritto {
    pub id: i64,
    pub nome: String,
    pub cognome: String,
    /// Mancano per gli iscritti inseriti di corsa (già iscritti) o prima che fossero obbligatorie.
    pub data_nascita: Option<NaiveDate>,
    pub luogo_nascita: Option<String>,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub note: Option<String>,
    pub archiviato: bool,
    pub ultima_data_fine_iscrizione: Option<NaiveDate>,
    pub stato_iscrizione: StatoValidita,
    pub ultima_data_scadenza_abbonamento: Option<NaiveDate>,
    pub stato_abbonamento: StatoValidita,
    /// Assente se il certificato non è stato presentato o è stato presentato senza scadenza.
    pub data_scadenza_certificato: Option<NaiveDate>,
    /// Anche senza una data di scadenza indicata.
    pub certificato_presentato: bool,
    pub stato_certificato: StatoValidita,
    /// Mancano dati anagrafici obbligatori: vanno completati dalla scheda.
    pub da_completare: bool,
}

/// Dati inseriti dall'utente per creare o modificare un iscritto.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatiIscritto {
    pub nome: String,
    pub cognome: String,
    /// Facoltativi: se mancano l'iscritto risulta da completare.
    pub data_nascita: Option<NaiveDate>,
    pub luogo_nascita: Option<String>,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub note: Option<String>,
    /// Scadenza del certificato medico; può mancare anche se il certificato è stato presentato.
    #[serde(default)]
    pub data_scadenza_certificato: Option<NaiveDate>,
    /// Certificato consegnato: con una scadenza conta comunque come presentato.
    #[serde(default)]
    pub certificato_presentato: bool,
}

impl DatiIscritto {
    /// Valore della colonna `certificato_senza_scadenza`.
    fn certificato_senza_scadenza(&self) -> bool {
        self.certificato_presentato && self.data_scadenza_certificato.is_none()
    }
}

/// Ordine dell'elenco: alfabetico, oppure per data (le scadenze vicine prima o le più recenti).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdinamentoIscritti {
    #[default]
    Alfabetico,
    ScadenzaAbbonamentoCrescente,
    ScadenzaAbbonamentoDecrescente,
    FineIscrizioneCrescente,
    FineIscrizioneDecrescente,
    ScadenzaCertificatoCrescente,
    ScadenzaCertificatoDecrescente,
}

impl OrdinamentoIscritti {
    fn come_sql(self) -> &'static str {
        match self {
            Self::Alfabetico => "ORDER BY i.cognome COLLATE NOCASE, i.nome COLLATE NOCASE, i.id",
            Self::ScadenzaAbbonamentoCrescente => {
                "ORDER BY a.ultima_data_scadenza, i.cognome COLLATE NOCASE, i.id"
            }
            Self::ScadenzaAbbonamentoDecrescente => {
                "ORDER BY a.ultima_data_scadenza DESC, i.cognome COLLATE NOCASE, i.id"
            }
            Self::FineIscrizioneCrescente => {
                "ORDER BY s.ultima_data_fine_iscrizione, i.cognome COLLATE NOCASE, i.id"
            }
            Self::FineIscrizioneDecrescente => {
                "ORDER BY s.ultima_data_fine_iscrizione DESC, i.cognome COLLATE NOCASE, i.id"
            }
            Self::ScadenzaCertificatoCrescente => {
                "ORDER BY c.data_scadenza_certificato, i.cognome COLLATE NOCASE, i.id"
            }
            Self::ScadenzaCertificatoDecrescente => {
                "ORDER BY c.data_scadenza_certificato DESC, i.cognome COLLATE NOCASE, i.id"
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiltroIscritti {
    /// Parole cercate in nome e cognome, senza distinzione di maiuscole e accenti.
    pub testo: String,
    pub includi_archiviati: bool,
    pub stato_abbonamento: Option<StatoValidita>,
    pub stato_iscrizione: Option<StatoValidita>,
    #[serde(default)]
    pub stato_certificato: Option<StatoValidita>,
    #[serde(default)]
    pub ordinamento: OrdinamentoIscritti,
    #[serde(default)]
    pub solo_da_completare: bool,
}

/// Data di oggi e preavviso: servono a calcolare gli stati di validità.
#[derive(Debug, Clone, Copy)]
pub struct Riferimento {
    pub oggi: NaiveDate,
    pub giorni_preavviso: u32,
}

/// Costruisce il riferimento leggendo i giorni di preavviso dalle impostazioni.
pub fn riferimento_per_oggi(
    connessione: &Connection,
    oggi: NaiveDate,
) -> Result<Riferimento, ErroreDatabase> {
    let giorni_preavviso = leggi_impostazione_numerica(
        connessione,
        CHIAVE_GIORNI_PREAVVISO,
        GIORNI_PREAVVISO_PREDEFINITI as usize,
    )?;
    Ok(Riferimento {
        oggi,
        giorni_preavviso: giorni_preavviso as u32,
    })
}

const SELEZIONE_ISCRITTI: &str = "
    SELECT i.id, i.nome, i.cognome, i.data_nascita, i.luogo_nascita, i.telefono, i.email,
           i.note, i.archiviato, s.ultima_data_fine_iscrizione, a.ultima_data_scadenza,
           i.data_scadenza_certificato, c.data_scadenza_certificato, i.certificato_senza_scadenza
    FROM iscritti i
    JOIN v_ultima_iscrizione_iscritti s ON s.iscritto_id = i.id
    JOIN v_ultima_scadenza_iscritti a ON a.iscritto_id = i.id
    JOIN v_certificato_iscritti c ON c.iscritto_id = i.id";

/// Gli iscritti eliminati restano solo come riferimento anonimo dei pagamenti.
const NON_ELIMINATO: &str = "i.data_eliminazione IS NULL";

const DA_COMPLETARE: &str = "(i.data_nascita IS NULL OR i.luogo_nascita IS NULL)";

const CONTEGGIO_ISCRITTI: &str = "
    SELECT count(*)
    FROM iscritti i
    JOIN v_ultima_iscrizione_iscritti s ON s.iscritto_id = i.id
    JOIN v_ultima_scadenza_iscritti a ON a.iscritto_id = i.id
    JOIN v_certificato_iscritti c ON c.iscritto_id = i.id";

/// Tutti gli iscritti che corrispondono al filtro (senza paginazione).
pub fn elenca_iscritti(
    connessione: &Connection,
    filtro: &FiltroIscritti,
    riferimento: Riferimento,
) -> Result<Vec<Iscritto>, ErroreDatabase> {
    let (clausola_where, parametri) = condizioni_filtro(filtro, riferimento);
    let mut query = connessione.prepare(&format!(
        "{SELEZIONE_ISCRITTI} {clausola_where} {}",
        filtro.ordinamento.come_sql()
    ))?;
    let iscritti = query
        .query_map(params_from_iter(parametri), |riga| {
            leggi_iscritto(riga, riferimento)
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(iscritti)
}

/// Una pagina dell'elenco filtrato: il database legge solo gli iscritti della pagina.
pub fn elenca_iscritti_paginati(
    connessione: &Connection,
    filtro: &FiltroIscritti,
    riferimento: Riferimento,
    richiesta: RichiestaPagina,
) -> Result<Pagina<Iscritto>, ErroreDatabase> {
    let (clausola_where, parametri) = condizioni_filtro(filtro, riferimento);

    let totale_elementi: u32 = connessione.query_row(
        &format!("{CONTEGGIO_ISCRITTI} {clausola_where}"),
        params_from_iter(parametri.iter()),
        |riga| riga.get(0),
    )?;

    let richiesta = richiesta.limitata_a(totale_elementi);
    let mut parametri_pagina = parametri;
    parametri_pagina.push(Value::from(i64::from(richiesta.dimensione())));
    parametri_pagina.push(Value::from(i64::from(richiesta.scarto())));
    let mut query = connessione.prepare(&format!(
        "{SELEZIONE_ISCRITTI} {clausola_where} {} LIMIT ? OFFSET ?",
        filtro.ordinamento.come_sql()
    ))?;
    let elementi = query
        .query_map(params_from_iter(parametri_pagina), |riga| {
            leggi_iscritto(riga, riferimento)
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Pagina {
        elementi,
        numero: richiesta.numero,
        elementi_per_pagina: richiesta.dimensione(),
        totale_elementi,
    })
}

pub fn trova_iscritto(
    connessione: &Connection,
    id: i64,
    riferimento: Riferimento,
) -> Result<Iscritto, ErroreDatabase> {
    connessione
        .query_row(
            &format!("{SELEZIONE_ISCRITTI} WHERE i.id = ?1 AND {NON_ELIMINATO}"),
            [id],
            |riga| leggi_iscritto(riga, riferimento),
        )
        .optional()?
        .ok_or(ErroreDatabase::NonTrovato)
}

pub fn crea_iscritto(
    connessione: &Connection,
    dati: &DatiIscritto,
    riferimento: Riferimento,
) -> Result<Iscritto, ErroreDatabase> {
    let dati = valida_dati_iscritto(dati, riferimento.oggi)?;
    connessione.execute(
        "INSERT INTO iscritti (nome, cognome, data_nascita, luogo_nascita, telefono, email, note,
                               data_scadenza_certificato, certificato_senza_scadenza)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            dati.nome,
            dati.cognome,
            dati.data_nascita,
            dati.luogo_nascita,
            dati.telefono,
            dati.email,
            dati.note,
            dati.data_scadenza_certificato,
            dati.certificato_senza_scadenza(),
        ],
    )?;
    trova_iscritto(connessione, connessione.last_insert_rowid(), riferimento)
}

pub fn modifica_iscritto(
    connessione: &Connection,
    id: i64,
    dati: &DatiIscritto,
    riferimento: Riferimento,
) -> Result<Iscritto, ErroreDatabase> {
    let dati = valida_dati_iscritto(dati, riferimento.oggi)?;
    let righe_modificate = connessione.execute(
        "UPDATE iscritti
         SET nome = ?1, cognome = ?2, data_nascita = ?3, luogo_nascita = ?4, telefono = ?5,
             email = ?6, note = ?7, data_scadenza_certificato = ?8,
             certificato_senza_scadenza = ?9, modificato_il = datetime('now')
         WHERE id = ?10 AND data_eliminazione IS NULL",
        params![
            dati.nome,
            dati.cognome,
            dati.data_nascita,
            dati.luogo_nascita,
            dati.telefono,
            dati.email,
            dati.note,
            dati.data_scadenza_certificato,
            dati.certificato_senza_scadenza(),
            id
        ],
    )?;
    if righe_modificate == 0 {
        return Err(ErroreDatabase::NonTrovato);
    }
    trova_iscritto(connessione, id, riferimento)
}

/// Di norma gli iscritti si archiviano, così lo storico resta; l'eliminazione è solo su richiesta.
pub fn imposta_iscritto_archiviato(
    connessione: &Connection,
    id: i64,
    archiviato: bool,
    riferimento: Riferimento,
) -> Result<Iscritto, ErroreDatabase> {
    let righe_modificate = connessione.execute(
        "UPDATE iscritti SET archiviato = ?1, modificato_il = datetime('now')
         WHERE id = ?2 AND data_eliminazione IS NULL",
        params![archiviato, id],
    )?;
    if righe_modificate == 0 {
        return Err(ErroreDatabase::NonTrovato);
    }
    trova_iscritto(connessione, id, riferimento)
}

/// Pulisce i dati (spazi, campi facoltativi vuoti) e controlla quelli obbligatori.
pub fn valida_dati_iscritto(
    dati: &DatiIscritto,
    oggi: NaiveDate,
) -> Result<DatiIscritto, ErroreDatabase> {
    let pulito = DatiIscritto {
        nome: dati.nome.trim().to_string(),
        cognome: dati.cognome.trim().to_string(),
        data_nascita: dati.data_nascita,
        luogo_nascita: testo_facoltativo(dati.luogo_nascita.as_deref()),
        telefono: testo_facoltativo(dati.telefono.as_deref()),
        email: testo_facoltativo(dati.email.as_deref()),
        note: testo_facoltativo(dati.note.as_deref()),
        data_scadenza_certificato: dati.data_scadenza_certificato,
        certificato_presentato: dati.certificato_presentato,
    };
    controlla_anagrafica(&pulito, oggi).map_err(ErroreDatabase::non_valido)?;
    Ok(pulito)
}

/// Controlla i dati già ripuliti dagli spazi; si ferma al primo errore.
fn controlla_anagrafica(dati: &DatiIscritto, oggi: NaiveDate) -> EsitoControllo {
    controlla_obbligatorio(&dati.nome, "Il nome")?;
    controlla_obbligatorio(&dati.cognome, "Il cognome")?;
    controlla_nome_proprio(&dati.nome, "Il nome", LUNGHEZZA_MASSIMA_NOME)?;
    controlla_nome_proprio(&dati.cognome, "Il cognome", LUNGHEZZA_MASSIMA_NOME)?;
    if let Some(luogo_nascita) = &dati.luogo_nascita {
        controlla_nome_proprio(
            luogo_nascita,
            "Il luogo di nascita",
            LUNGHEZZA_MASSIMA_LUOGO,
        )?;
    }
    if let Some(data_nascita) = dati.data_nascita {
        controlla_data_nascita(data_nascita, oggi)?;
    }
    if let Some(telefono) = &dati.telefono {
        controlla_telefono(telefono)?;
    }
    if let Some(email) = &dati.email {
        controlla_email(email)?;
    }
    if let Some(note) = &dati.note {
        controlla_note(note)?;
    }
    if let Some(scadenza) = dati.data_scadenza_certificato {
        controlla_scadenza_certificato(scadenza, oggi)?;
    }
    Ok(())
}

/// Clausola WHERE e parametri del filtro; gli stati sono tradotti in intervalli di date.
fn condizioni_filtro(filtro: &FiltroIscritti, riferimento: Riferimento) -> (String, Vec<Value>) {
    let mut condizioni = vec![
        "(i.archiviato = 0 OR ?1 = 1)".to_string(),
        NON_ELIMINATO.to_string(),
    ];
    let mut parametri: Vec<Value> = vec![Value::from(filtro.includi_archiviati)];
    if filtro.solo_da_completare {
        condizioni.push(DA_COMPLETARE.to_string());
    }

    // Ogni parola deve comparire nel nome o nel cognome: "ross mar" trova "Mario Rossi"
    for parola in parole_di_ricerca(&filtro.testo) {
        parametri.push(Value::from(format!("%{}%", escape_like(&parola))));
        let indice = parametri.len();
        condizioni.push(format!(
            "(normalizza(i.nome) LIKE ?{indice} ESCAPE '\\'
              OR normalizza(i.cognome) LIKE ?{indice} ESCAPE '\\')"
        ));
    }

    let filtri_stato = [
        ("a.ultima_data_scadenza", filtro.stato_abbonamento),
        ("s.ultima_data_fine_iscrizione", filtro.stato_iscrizione),
        ("c.data_scadenza_certificato", filtro.stato_certificato),
    ];
    for (colonna, stato) in filtri_stato {
        if let Some(stato) = stato {
            aggiungi_condizione_stato(colonna, stato, riferimento, &mut condizioni, &mut parametri);
        }
    }

    (format!("WHERE {}", condizioni.join(" AND ")), parametri)
}

fn aggiungi_condizione_stato(
    colonna: &str,
    stato: StatoValidita,
    riferimento: Riferimento,
    condizioni: &mut Vec<String>,
    parametri: &mut Vec<Value>,
) {
    let condizione =
        condizione_scadenza_per_stato(stato, riferimento.oggi, riferimento.giorni_preavviso);
    let CondizioneScadenza::Tra { dal, al } = condizione else {
        condizioni.push(format!("{colonna} IS NULL"));
        return;
    };
    condizioni.push(format!("{colonna} IS NOT NULL"));
    if let Some(dal) = dal {
        parametri.push(Value::from(dal.to_string()));
        condizioni.push(format!("{colonna} >= ?{}", parametri.len()));
    }
    if let Some(al) = al {
        parametri.push(Value::from(al.to_string()));
        condizioni.push(format!("{colonna} <= ?{}", parametri.len()));
    }
}

/// I caratteri % e _ sono jolly in LIKE: vanno protetti se l'utente li scrive.
fn escape_like(testo: &str) -> String {
    testo
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

fn leggi_iscritto(riga: &Row<'_>, riferimento: Riferimento) -> rusqlite::Result<Iscritto> {
    let stato_di = |data: Option<NaiveDate>| {
        calcola_stato_validita(data, riferimento.oggi, riferimento.giorni_preavviso)
    };
    let ultima_data_fine_iscrizione: Option<NaiveDate> = riga.get(9)?;
    let ultima_data_scadenza_abbonamento: Option<NaiveDate> = riga.get(10)?;
    let data_scadenza_certificato: Option<NaiveDate> = riga.get(11)?;
    // Per stato, promemoria e filtri un certificato senza data vale come sempre valido
    let scadenza_certificato_effettiva: Option<NaiveDate> = riga.get(12)?;
    let certificato_senza_scadenza: bool = riga.get(13)?;
    let data_nascita: Option<NaiveDate> = riga.get(3)?;
    let luogo_nascita: Option<String> = riga.get(4)?;
    let da_completare = data_nascita.is_none() || luogo_nascita.is_none();
    Ok(Iscritto {
        id: riga.get(0)?,
        nome: riga.get(1)?,
        cognome: riga.get(2)?,
        data_nascita,
        luogo_nascita,
        telefono: riga.get(5)?,
        email: riga.get(6)?,
        note: riga.get(7)?,
        archiviato: riga.get(8)?,
        ultima_data_fine_iscrizione,
        stato_iscrizione: stato_di(ultima_data_fine_iscrizione),
        ultima_data_scadenza_abbonamento,
        stato_abbonamento: stato_di(ultima_data_scadenza_abbonamento),
        data_scadenza_certificato,
        certificato_presentato: data_scadenza_certificato.is_some() || certificato_senza_scadenza,
        stato_certificato: stato_di(scadenza_certificato_effettiva),
        da_completare,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const OGGI: NaiveDate = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();

    fn dati(nome: &str, cognome: &str) -> DatiIscritto {
        DatiIscritto {
            nome: nome.into(),
            cognome: cognome.into(),
            data_nascita: Some(NaiveDate::from_ymd_opt(1990, 5, 20).unwrap()),
            luogo_nascita: Some(" Roma ".into()),
            telefono: Some("   ".into()),
            email: Some(" mario@esempio.it ".into()),
            note: Some(" nota ".into()),
            data_scadenza_certificato: None,
            certificato_presentato: false,
        }
    }

    #[test]
    fn la_validazione_ripulisce_spazi_e_campi_vuoti() {
        let puliti = valida_dati_iscritto(&dati("  Mario ", " Rossi "), OGGI).unwrap();
        assert_eq!(puliti.nome, "Mario");
        assert_eq!(puliti.cognome, "Rossi");
        assert_eq!(puliti.luogo_nascita.as_deref(), Some("Roma"));
        assert_eq!(puliti.telefono, None);
        assert_eq!(puliti.email.as_deref(), Some("mario@esempio.it"));
        assert_eq!(puliti.note.as_deref(), Some("nota"));
    }

    #[test]
    fn solo_nome_e_cognome_sono_obbligatori() {
        assert!(valida_dati_iscritto(&dati(" ", "Rossi"), OGGI).is_err());
        assert!(valida_dati_iscritto(&dati("Mario", ""), OGGI).is_err());
        let solo_nome_e_cognome = DatiIscritto {
            data_nascita: None,
            luogo_nascita: Some("  ".into()),
            ..dati("Mario", "Rossi")
        };
        let puliti = valida_dati_iscritto(&solo_nome_e_cognome, OGGI).unwrap();
        assert_eq!((puliti.data_nascita, puliti.luogo_nascita), (None, None));
    }

    #[test]
    fn la_data_di_nascita_non_puo_essere_nel_futuro() {
        let futura = DatiIscritto {
            data_nascita: Some(OGGI.succ_opt().unwrap()),
            ..dati("Mario", "Rossi")
        };
        assert!(valida_dati_iscritto(&futura, OGGI).is_err());
    }

    #[test]
    fn l_email_e_facoltativa_ma_se_presente_deve_essere_valida() {
        let senza_email = DatiIscritto {
            email: Some("  ".into()),
            ..dati("Mario", "Rossi")
        };
        assert_eq!(
            valida_dati_iscritto(&senza_email, OGGI).unwrap().email,
            None
        );
        for non_valida in [
            "mario",
            "mario@",
            "@esempio.it",
            "mario@esempio",
            "ma rio@esempio.it",
            "mario@esempio.",
        ] {
            let dati = DatiIscritto {
                email: Some(non_valida.into()),
                ..dati("Mario", "Rossi")
            };
            assert!(
                valida_dati_iscritto(&dati, OGGI).is_err(),
                "{non_valida} accettata"
            );
        }
    }

    #[test]
    fn i_jolly_di_like_sono_protetti() {
        assert_eq!(escape_like("50%_a\\b"), "50\\%\\_a\\\\b");
    }
}
