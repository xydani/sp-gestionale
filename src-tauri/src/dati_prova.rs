//! Generatore deterministico di dati fittizi per test e prove manuali.
//! Tutti i nomi e i numeri di telefono sono inventati.

use chrono::{Days, NaiveDate};
use rusqlite::{Connection, params};

use crate::db::ErroreDatabase;
use crate::db::iscritti::Riferimento;
use crate::db::iscritto_gia_attivo::{DatiIscrittoGiaAttivo, registra_iscritto_gia_attivo};
use crate::db::tipologie::{TipologiaAbbonamento, elenca_tipologie};
use crate::domain::anno_sportivo::Periodo;
use crate::domain::anno_sportivo::{
    MESE_INIZIO_ANNO_SPORTIVO_PREDEFINITO, anno_sportivo_contenente,
};
use crate::domain::date::calcola_data_scadenza;
use crate::domain::pagamento::MetodoPagamento;
use crate::domain::stato::GIORNI_PREAVVISO_PREDEFINITI;

pub const NUMERO_ISCRITTI_PREDEFINITO: u32 = 250;

/// Uno su venti è inserito come già iscritto prima del programma, con anagrafica da completare.
const ISCRITTI_PER_OGNI_GIA_ATTIVO: u32 = 20;
/// Gli abbonamenti dei già iscritti scadono entro questi giorni (anche il mensile è già iniziato).
const GIORNI_MASSIMI_SCADENZA_GIA_ATTIVI: u64 = 25;

const NOMI: &[&str] = &[
    "Marco",
    "Giulia",
    "Luca",
    "Francesca",
    "Alessandro",
    "Chiara",
    "Andrea",
    "Sara",
    "Matteo",
    "Martina",
    "Davide",
    "Elena",
    "Simone",
    "Valentina",
    "Federico",
    "Alessia",
    "Lorenzo",
    "Silvia",
    "Riccardo",
    "Laura",
    "Stefano",
    "Giorgia",
    "Paolo",
    "Elisa",
    "Gabriele",
    "Anna",
    "Nicola",
    "Federica",
    "Tommaso",
    "Ilaria",
    "Daniele",
    "Roberta",
    "Emanuele",
    "Beatrice",
    "Fabio",
    "Marta",
];

const COGNOMI: &[&str] = &[
    "Rossi",
    "Russo",
    "Ferrari",
    "Esposito",
    "Bianchi",
    "Romano",
    "Colombo",
    "Ricci",
    "Marino",
    "Greco",
    "Bruno",
    "Gallo",
    "Conti",
    "De Luca",
    "Mancini",
    "Costa",
    "Giordano",
    "Rizzo",
    "Lombardi",
    "Moretti",
    "Barbieri",
    "Fontana",
    "Santoro",
    "Mariani",
    "Rinaldi",
    "Caruso",
    "Ferrara",
    "Galli",
    "Martini",
    "Leone",
    "Longo",
    "Gentile",
    "Martinelli",
    "Vitale",
    "D'Angelo",
];

const LUOGHI_NASCITA: &[&str] = &[
    "Roma", "Milano", "Napoli", "Torino", "Palermo", "Genova", "Bologna", "Firenze", "Bari",
    "Catania", "Verona", "Padova", "Brescia", "Parma", "Modena", "Perugia", "Pescara", "Lecce",
];

const QUOTA_ISCRIZIONE_CENT: i64 = 3000;
/// Percentuale di iscritti con l'email indicata.
const PERCENTUALE_CON_EMAIL: u64 = 60;

/// Percentuale di iscritti senza alcun pagamento.
const PERCENTUALE_SENZA_PAGAMENTI: u64 = 8;
const ANNI_MASSIMI_DI_STORICO: u64 = 3;
/// Quote di iscritti con casi particolari di iscrizione annuale, per provare stati e avvisi.
const PERCENTUALE_ISCRIZIONE_IN_SCADENZA: u64 = 10;
/// Le iscrizioni in scadenza finiscono entro questi giorni: compaiono nelle prossime scadenze.
const GIORNI_MASSIMI_ISCRIZIONE_IN_SCADENZA: u64 = 30;
const PERCENTUALE_ISCRIZIONE_NON_RINNOVATA: u64 = 6;
const PERCENTUALE_SENZA_ISCRIZIONE: u64 = 3;
const PERCENTUALE_CON_CERTIFICATO: u64 = 80;
/// Le scadenze dei certificati vanno da due mesi fa a quasi un anno da oggi.
const GIORNI_CERTIFICATO_PRIMA_DI_OGGI: u64 = 60;
const GIORNI_CERTIFICATO_DOPO_OGGI: u64 = 330;
/// Una parte dei certificati scade a ridosso di oggi, per avere sempre scaduti recenti e vicini.
const PERCENTUALE_CERTIFICATO_VICINO: u64 = 25;
const GIORNI_CERTIFICATO_VICINO: u64 = 30;

/// Come viene generata l'iscrizione annuale di un iscritto.
#[derive(Clone, Copy, PartialEq, Eq)]
enum CasoIscrizione {
    /// Un'iscrizione per ogni anno sportivo di attività.
    Regolare,
    /// L'iscrizione in corso termina entro un mese.
    InScadenza,
    /// Manca l'iscrizione dell'anno sportivo in corso, anche se l'abbonamento è attivo.
    NonRinnovata,
    /// Nessuna iscrizione mai pagata.
    Assente,
}

impl CasoIscrizione {
    fn scegli(casuale: &mut GeneratoreCasuale) -> Self {
        let estratto = casuale.tra(1, 100);
        let soglia_in_scadenza = PERCENTUALE_ISCRIZIONE_IN_SCADENZA;
        let soglia_non_rinnovata = soglia_in_scadenza + PERCENTUALE_ISCRIZIONE_NON_RINNOVATA;
        let soglia_assente = soglia_non_rinnovata + PERCENTUALE_SENZA_ISCRIZIONE;
        if estratto <= soglia_in_scadenza {
            Self::InScadenza
        } else if estratto <= soglia_non_rinnovata {
            Self::NonRinnovata
        } else if estratto <= soglia_assente {
            Self::Assente
        } else {
            Self::Regolare
        }
    }
}

/// Generatore pseudo-casuale xorshift: con lo stesso seme produce sempre gli stessi dati.
struct GeneratoreCasuale(u64);

impl GeneratoreCasuale {
    fn nuovo(seme: u64) -> Self {
        Self(seme.max(1))
    }

    fn prossimo(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// Numero intero nell'intervallo [minimo, massimo].
    fn tra(&mut self, minimo: u64, massimo: u64) -> u64 {
        minimo + self.prossimo() % (massimo - minimo + 1)
    }

    fn scegli<'a, T>(&mut self, elementi: &'a [T]) -> &'a T {
        &elementi[self.tra(0, elementi.len() as u64 - 1) as usize]
    }
}

/// Inserisce `numero_iscritti` iscritti fittizi con il loro storico di pagamenti.
/// Le scadenze sono distribuite attorno a `oggi`: ci sono scaduti, in scadenza e attivi.
pub fn genera_dati_prova(
    connessione: &mut Connection,
    numero_iscritti: u32,
    oggi: NaiveDate,
    seme: u64,
) -> Result<(), ErroreDatabase> {
    let mut casuale = GeneratoreCasuale::nuovo(seme);
    let tipologie = elenca_tipologie(connessione, true)?;
    let numero_gia_attivi = numero_iscritti / ISCRITTI_PER_OGNI_GIA_ATTIVO;
    let transazione = connessione.transaction()?;

    for _ in 0..numero_iscritti - numero_gia_attivi {
        let giorni_dall_iscrizione = casuale.tra(30, ANNI_MASSIMI_DI_STORICO * 365);
        let data_iscrizione = oggi - Days::new(giorni_dall_iscrizione);
        let iscritto_id = inserisci_iscritto(&transazione, &mut casuale, oggi)?;

        if casuale.tra(1, 100) <= PERCENTUALE_SENZA_PAGAMENTI {
            continue;
        }

        // L'ultimo abbonamento copre questa data: da 150 giorni fa a 30 giorni da oggi
        let ultima_data_coperta = oggi - Days::new(150) + Days::new(casuale.tra(0, 180));
        let attivita = Periodo {
            inizio: data_iscrizione,
            fine: ultima_data_coperta,
        };
        inserisci_pagamenti(
            &transazione,
            &mut casuale,
            &tipologie,
            iscritto_id,
            attivita,
        )?;

        let caso = CasoIscrizione::scegli(&mut casuale);
        let fine_iscrizioni = if caso == CasoIscrizione::InScadenza {
            ultima_data_coperta.max(oggi)
        } else {
            ultima_data_coperta
        };
        let periodo_iscrizioni = Periodo {
            inizio: data_iscrizione,
            fine: fine_iscrizioni,
        };
        inserisci_iscrizioni(
            &transazione,
            &mut casuale,
            iscritto_id,
            periodo_iscrizioni,
            oggi,
            caso,
        )?;
    }

    transazione.commit()?;

    for _ in 0..numero_gia_attivi {
        inserisci_iscritto_gia_attivo(connessione, &mut casuale, &tipologie, oggi)?;
    }
    Ok(())
}

/// Solo nome, cognome, abbonamento in corso e quota, come dall'inserimento rapido.
fn inserisci_iscritto_gia_attivo(
    connessione: &mut Connection,
    casuale: &mut GeneratoreCasuale,
    tipologie: &[TipologiaAbbonamento],
    oggi: NaiveDate,
) -> Result<(), ErroreDatabase> {
    let tipologia = casuale.scegli(tipologie);
    let dati = DatiIscrittoGiaAttivo {
        nome: casuale.scegli(NOMI).to_string(),
        cognome: casuale.scegli(COGNOMI).to_string(),
        telefono: None,
        data_scadenza_certificato: None,
        certificato_presentato: false,
        tipologia_id: tipologia.id,
        data_scadenza_abbonamento: oggi
            + Days::new(casuale.tra(0, GIORNI_MASSIMI_SCADENZA_GIA_ATTIVI)),
        importo_abbonamento_cent: tipologia.prezzo_cent,
        quota_pagata: true,
        quota_cent: QUOTA_ISCRIZIONE_CENT,
        metodo: MetodoPagamento::Contanti,
    };
    let riferimento = Riferimento {
        oggi,
        giorni_preavviso: GIORNI_PREAVVISO_PREDEFINITI,
    };
    registra_iscritto_gia_attivo(connessione, &dati, riferimento)?;
    Ok(())
}

/// Anagrafica inventata; restituisce l'id del nuovo iscritto.
fn inserisci_iscritto(
    transazione: &rusqlite::Transaction<'_>,
    casuale: &mut GeneratoreCasuale,
    oggi: NaiveDate,
) -> Result<i64, ErroreDatabase> {
    let nome = casuale.scegli(NOMI);
    let cognome = casuale.scegli(COGNOMI);
    let data_nascita = oggi - Days::new(casuale.tra(16 * 365, 70 * 365));

    let mut email = None;
    if casuale.tra(1, 100) <= PERCENTUALE_CON_EMAIL {
        let cognome_senza_spazi = cognome.to_lowercase().replace(['\'', ' '], "");
        email = Some(format!(
            "{}.{cognome_senza_spazi}@esempio.it",
            nome.to_lowercase()
        ));
    }

    let mut scadenza_certificato = None;
    if casuale.tra(1, 100) <= PERCENTUALE_CON_CERTIFICATO {
        scadenza_certificato = Some(scegli_scadenza_certificato(casuale, oggi));
    }

    let luogo_nascita = casuale.scegli(LUOGHI_NASCITA);
    let telefono = format!(
        "+39 3{:02} {:07}",
        casuale.tra(20, 99),
        casuale.tra(0, 9_999_999)
    );
    transazione.execute(
        "INSERT INTO iscritti (nome, cognome, data_nascita, luogo_nascita, telefono, email,
                               data_scadenza_certificato)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            nome,
            cognome,
            data_nascita,
            luogo_nascita,
            telefono,
            email,
            scadenza_certificato,
        ],
    )?;
    Ok(transazione.last_insert_rowid())
}

/// Per lo più sparse tra due mesi fa e quasi un anno, una parte entro un mese da oggi.
fn scegli_scadenza_certificato(casuale: &mut GeneratoreCasuale, oggi: NaiveDate) -> NaiveDate {
    let (giorni_prima, giorni_dopo) = if casuale.tra(1, 100) <= PERCENTUALE_CERTIFICATO_VICINO {
        (GIORNI_CERTIFICATO_VICINO, GIORNI_CERTIFICATO_VICINO)
    } else {
        (
            GIORNI_CERTIFICATO_PRIMA_DI_OGGI,
            GIORNI_CERTIFICATO_DOPO_OGGI,
        )
    };
    let minimo = oggi - Days::new(giorni_prima);
    minimo + Days::new(casuale.tra(0, giorni_prima + giorni_dopo))
}

/// Abbonamenti uno dopo l'altro, a volte con qualche giorno di pausa, fino a coprire l'attività.
fn inserisci_pagamenti(
    transazione: &rusqlite::Transaction<'_>,
    casuale: &mut GeneratoreCasuale,
    tipologie: &[TipologiaAbbonamento],
    iscritto_id: i64,
    attivita: Periodo,
) -> Result<(), ErroreDatabase> {
    let mut data_inizio = attivita.inizio;
    let mut pagamento_precedente_id: Option<i64> = None;

    while data_inizio <= attivita.fine {
        let tipologia = casuale.scegli(tipologie);
        let data_scadenza = calcola_data_scadenza(data_inizio, tipologia.durata())
            .expect("le tipologie di prova hanno durate valide");

        transazione.execute(
            "INSERT INTO pagamenti (iscritto_id, tipologia_id, tipologia_nome, importo_cent,
                 data_pagamento, metodo, data_inizio, data_scadenza, rinnovo_di_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                iscritto_id,
                tipologia.id,
                tipologia.nome,
                tipologia.prezzo_cent,
                data_inizio.to_string(),
                casuale.scegli(&MetodoPagamento::TUTTI).come_testo(),
                data_inizio.to_string(),
                data_scadenza.to_string(),
                pagamento_precedente_id,
            ],
        )?;
        pagamento_precedente_id = Some(transazione.last_insert_rowid());

        let mut giorni_di_pausa = 0;
        if casuale.tra(1, 100) <= 20 {
            giorni_di_pausa = casuale.tra(1, 30);
        }
        data_inizio = data_scadenza + Days::new(1 + giorni_di_pausa);
    }
    Ok(())
}

/// Un'iscrizione per ogni anno sportivo del periodo di attività, salvo i casi particolari.
fn inserisci_iscrizioni(
    transazione: &rusqlite::Transaction<'_>,
    casuale: &mut GeneratoreCasuale,
    iscritto_id: i64,
    attivita: Periodo,
    oggi: NaiveDate,
    caso: CasoIscrizione,
) -> Result<(), ErroreDatabase> {
    if caso == CasoIscrizione::Assente {
        return Ok(());
    }
    let mut periodo =
        anno_sportivo_contenente(attivita.inizio, MESE_INIZIO_ANNO_SPORTIVO_PREDEFINITO);
    while periodo.inizio <= attivita.fine {
        let comprende_oggi = periodo.inizio <= oggi && oggi <= periodo.fine;
        if comprende_oggi && caso == CasoIscrizione::NonRinnovata {
            break;
        }
        let data_fine = if comprende_oggi && caso == CasoIscrizione::InScadenza {
            oggi + Days::new(casuale.tra(0, GIORNI_MASSIMI_ISCRIZIONE_IN_SCADENZA))
        } else {
            periodo.fine
        };
        transazione.execute(
            "INSERT INTO iscrizioni (iscritto_id, importo_cent, data_pagamento, metodo,
                 data_inizio, data_fine)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                iscritto_id,
                QUOTA_ISCRIZIONE_CENT,
                periodo.inizio.max(attivita.inizio),
                casuale.scegli(&MetodoPagamento::TUTTI).come_testo(),
                periodo.inizio,
                data_fine,
            ],
        )?;
        let inizio_anno_dopo = periodo
            .fine
            .succ_opt()
            .expect("data nell'intervallo supportato");
        periodo = anno_sportivo_contenente(inizio_anno_dopo, MESE_INIZIO_ANNO_SPORTIVO_PREDEFINITO);
    }
    Ok(())
}
