//! Generatore deterministico di dati fittizi per test e prove manuali.
//! Tutti i nomi e i numeri di telefono sono inventati.

use chrono::{Days, NaiveDate};
use rusqlite::{Connection, params};

use crate::db::ErroreDatabase;
use crate::db::tipologie::elenca_tipologie;
use crate::domain::date::calcola_data_scadenza;
use crate::domain::pagamento::MetodoPagamento;

pub const NUMERO_ISCRITTI_PREDEFINITO: u32 = 250;

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

/// Percentuale di iscritti senza alcun pagamento.
const PERCENTUALE_SENZA_PAGAMENTI: u64 = 8;
const ANNI_MASSIMI_DI_STORICO: u64 = 3;

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
    let transazione = connessione.transaction()?;

    for _ in 0..numero_iscritti {
        let giorni_dall_iscrizione = casuale.tra(30, ANNI_MASSIMI_DI_STORICO * 365);
        let data_iscrizione = oggi - Days::new(giorni_dall_iscrizione);

        transazione.execute(
            "INSERT INTO iscritti (nome, cognome, telefono, data_iscrizione) VALUES (?1, ?2, ?3, ?4)",
            params![
                casuale.scegli(NOMI),
                casuale.scegli(COGNOMI),
                format!("+39 3{:02} {:07}", casuale.tra(20, 99), casuale.tra(0, 9_999_999)),
                data_iscrizione.to_string(),
            ],
        )?;
        let iscritto_id = transazione.last_insert_rowid();

        if casuale.tra(1, 100) <= PERCENTUALE_SENZA_PAGAMENTI {
            continue;
        }

        // L'ultimo abbonamento copre questa data: da 150 giorni fa a 30 giorni da oggi
        let ultima_data_coperta = oggi - Days::new(150) + Days::new(casuale.tra(0, 180));
        let mut data_inizio = data_iscrizione;
        let mut pagamento_precedente_id: Option<i64> = None;

        while data_inizio <= ultima_data_coperta {
            let tipologia = casuale.scegli(&tipologie);
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

            let giorni_di_pausa = if casuale.tra(1, 100) <= 20 {
                casuale.tra(1, 30)
            } else {
                0
            };
            data_inizio = data_scadenza + Days::new(1 + giorni_di_pausa);
        }
    }

    transazione.commit()?;
    Ok(())
}
