//! Controlli di plausibilità sui dati inseriti a mano: ogni errore è un messaggio per l'utente.

use chrono::{Months, NaiveDate};

pub const LUNGHEZZA_MASSIMA_NOME: usize = 60;
pub const LUNGHEZZA_MASSIMA_LUOGO: usize = 80;
pub const LUNGHEZZA_MASSIMA_EMAIL: usize = 254;
pub const LUNGHEZZA_MASSIMA_NOTE: usize = 1000;
/// Le cifre di un numero di telefono, prefisso internazionale compreso (standard E.164).
pub const CIFRE_TELEFONO_MINIME: usize = 6;
pub const CIFRE_TELEFONO_MASSIME: usize = 15;
pub const ETA_MASSIMA_ANNI: u32 = 120;
/// Un certificato medico sportivo vale un anno: un mese di margine per le date arrotondate.
pub const VALIDITA_MASSIMA_CERTIFICATO_MESI: u32 = 13;
/// Limite oltre il quale un importo è sicuramente un errore di battitura (10.000 €).
pub const IMPORTO_MASSIMO_CENT: i64 = 1_000_000;
pub const DURATA_MASSIMA_ABBONAMENTO_MESI: u32 = 36;
/// L'inizio del periodo pagato non si discosta più di così dalla data di pagamento.
pub const DISTANZA_MASSIMA_INIZIO_DAL_PAGAMENTO_MESI: u32 = 12;
/// Pagamenti più vecchi di così sono sicuramente un errore di battitura dell'anno.
pub const ANNI_MASSIMI_PAGAMENTO_PASSATO: u32 = 10;

pub type EsitoControllo = Result<(), String>;

/// Nome, cognome o luogo: lettere (anche accentate), spazi, apostrofi, trattini e punti.
/// Serve almeno una lettera: "123" o "--" non sono un nome.
pub fn controlla_nome_proprio(
    testo: &str,
    campo: &str,
    lunghezza_massima: usize,
) -> EsitoControllo {
    if testo.chars().count() > lunghezza_massima {
        return Err(format!(
            "{campo} può avere al massimo {lunghezza_massima} caratteri"
        ));
    }
    let carattere_ammesso =
        |c: char| c.is_alphabetic() || c == ' ' || matches!(c, '\'' | '’' | '-' | '.');
    if !testo.chars().all(carattere_ammesso) || !testo.chars().any(char::is_alphabetic) {
        return Err(format!(
            "{campo} può contenere solo lettere, spazi, apostrofi e trattini"
        ));
    }
    Ok(())
}

pub fn controlla_data_nascita(data_nascita: NaiveDate, oggi: NaiveDate) -> EsitoControllo {
    if data_nascita > oggi {
        return Err("La data di nascita non può essere nel futuro".into());
    }
    let limite = oggi - Months::new(ETA_MASSIMA_ANNI * 12);
    if data_nascita < limite {
        return Err(format!(
            "La data di nascita non è plausibile: più di {ETA_MASSIMA_ANNI} anni fa"
        ));
    }
    Ok(())
}

/// Cifre con eventuali spazi, trattini, punti, barre, parentesi e un "+" iniziale.
pub fn controlla_telefono(telefono: &str) -> EsitoControllo {
    let senza_prefisso = telefono.strip_prefix('+').unwrap_or(telefono);
    let ammesso = |c: char| c.is_ascii_digit() || matches!(c, ' ' | '-' | '.' | '/' | '(' | ')');
    let cifre = senza_prefisso.chars().filter(char::is_ascii_digit).count();
    if !senza_prefisso.chars().all(ammesso)
        || !(CIFRE_TELEFONO_MINIME..=CIFRE_TELEFONO_MASSIME).contains(&cifre)
    {
        return Err(format!(
            "Il telefono non è valido: servono da {CIFRE_TELEFONO_MINIME} a {CIFRE_TELEFONO_MASSIME} cifre (es. 333 1234567 o +39 333 1234567)"
        ));
    }
    Ok(())
}

/// Controllo volutamente semplice: qualcosa@dominio.estensione, senza spazi.
pub fn controlla_email(email: &str) -> EsitoControllo {
    let valida = email.chars().count() <= LUNGHEZZA_MASSIMA_EMAIL
        && !email.contains(char::is_whitespace)
        && email.split_once('@').is_some_and(|(utente, dominio)| {
            !utente.is_empty()
                && !dominio.contains('@')
                && dominio.split('.').count() >= 2
                && dominio.split('.').all(|parte| !parte.is_empty())
        });
    if valida {
        Ok(())
    } else {
        Err("L'indirizzo email non è valido (es. nome@esempio.it)".into())
    }
}

pub fn controlla_note(note: &str) -> EsitoControllo {
    if note.chars().count() > LUNGHEZZA_MASSIMA_NOTE {
        return Err(format!(
            "Le note possono avere al massimo {LUNGHEZZA_MASSIMA_NOTE} caratteri"
        ));
    }
    Ok(())
}

/// Un certificato già scaduto si può registrare (resta l'avviso), uno che dura anni no.
pub fn controlla_scadenza_certificato(scadenza: NaiveDate, oggi: NaiveDate) -> EsitoControllo {
    if scadenza > oggi + Months::new(VALIDITA_MASSIMA_CERTIFICATO_MESI) {
        return Err(
            "La scadenza del certificato è troppo lontana: un certificato vale al massimo un anno"
                .into(),
        );
    }
    Ok(())
}

/// Importo da 0 (es. prova gratuita) fino al massimo plausibile.
pub fn controlla_importo(importo_cent: i64, campo: &str) -> EsitoControllo {
    if importo_cent < 0 {
        return Err(format!("{campo} non può essere negativo"));
    }
    if importo_cent > IMPORTO_MASSIMO_CENT {
        return Err(format!(
            "{campo} è troppo alto: il massimo è {} €",
            IMPORTO_MASSIMO_CENT / 100
        ));
    }
    Ok(())
}

/// Si registra un incasso già avvenuto: non nel futuro, e non anni fa per un anno sbagliato.
pub fn controlla_data_pagamento(data_pagamento: NaiveDate, oggi: NaiveDate) -> EsitoControllo {
    if data_pagamento > oggi {
        return Err("La data di pagamento non può essere nel futuro".into());
    }
    if data_pagamento < oggi - Months::new(ANNI_MASSIMI_PAGAMENTO_PASSATO * 12) {
        return Err(format!(
            "La data di pagamento è di più di {ANNI_MASSIMI_PAGAMENTO_PASSATO} anni fa: controllare l'anno"
        ));
    }
    Ok(())
}

/// Periodo pagato: fine non prima dell'inizio, durata entro il massimo e inizio vicino al pagamento.
pub fn controlla_periodo(
    data_inizio: NaiveDate,
    data_fine: NaiveDate,
    data_pagamento: NaiveDate,
    durata_massima_mesi: u32,
) -> EsitoControllo {
    if data_fine < data_inizio {
        return Err("La fine non può essere precedente all'inizio".into());
    }
    if data_fine > data_inizio + Months::new(durata_massima_mesi) {
        return Err(format!(
            "Il periodo è troppo lungo: al massimo {durata_massima_mesi} mesi"
        ));
    }
    let distanza = Months::new(DISTANZA_MASSIMA_INIZIO_DAL_PAGAMENTO_MESI);
    if data_inizio > data_pagamento + distanza || data_inizio < data_pagamento - distanza {
        return Err(
            "L'inizio del periodo è a più di un anno dalla data di pagamento: controllare le date"
                .into(),
        );
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn data(testo: &str) -> NaiveDate {
        testo.parse().unwrap()
    }

    const OGGI: &str = "2026-11-10";

    #[test]
    fn i_nomi_ammettono_lettere_accenti_apostrofi_e_trattini() {
        for nome in [
            "Nicolò",
            "D'Angelo",
            "De Rosa-Bianchi",
            "S. Giovanni Rotondo",
            "Pérez",
        ] {
            assert!(
                controlla_nome_proprio(nome, "Il nome", 60).is_ok(),
                "{nome}"
            );
        }
    }

    #[test]
    fn i_nomi_rifiutano_numeri_simboli_e_testi_senza_lettere() {
        for nome in ["123", "Roma1", "Mario!", "--", "a@b"] {
            assert!(
                controlla_nome_proprio(nome, "Il nome", 60).is_err(),
                "{nome}"
            );
        }
        assert!(controlla_nome_proprio(&"a".repeat(61), "Il nome", 60).is_err());
    }

    #[test]
    fn la_data_di_nascita_non_e_futura_ne_troppo_lontana() {
        let oggi = data(OGGI);
        assert!(controlla_data_nascita(oggi, oggi).is_ok());
        assert!(controlla_data_nascita(data("1990-05-01"), oggi).is_ok());
        assert!(controlla_data_nascita(data("2026-11-11"), oggi).is_err());
        assert!(controlla_data_nascita(data("1906-11-09"), oggi).is_err());
    }

    #[test]
    fn il_telefono_accetta_i_formati_comuni() {
        for telefono in [
            "3331234567",
            "333 123 4567",
            "+39 333-1234567",
            "(06) 1234.567",
            "0612/34567",
        ] {
            assert!(controlla_telefono(telefono).is_ok(), "{telefono}");
        }
        for telefono in [
            "abc",
            "12345",
            "+39 333 1234567 8901 2",
            "333+1234567",
            "tel 3331234567",
        ] {
            assert!(controlla_telefono(telefono).is_err(), "{telefono}");
        }
    }

    #[test]
    fn l_email_richiede_utente_e_dominio_con_estensione() {
        assert!(controlla_email("mario.rossi@esempio.it").is_ok());
        for email in [
            "mario",
            "mario@",
            "@esempio.it",
            "mario@esempio",
            "ma rio@esempio.it",
            "a@b@c.it",
            "a@b..it",
        ] {
            assert!(controlla_email(email).is_err(), "{email}");
        }
    }

    #[test]
    fn il_certificato_dura_al_massimo_poco_piu_di_un_anno() {
        let oggi = data(OGGI);
        assert!(controlla_scadenza_certificato(data("2025-01-01"), oggi).is_ok());
        assert!(controlla_scadenza_certificato(data("2027-11-10"), oggi).is_ok());
        assert!(controlla_scadenza_certificato(data("2028-11-10"), oggi).is_err());
    }

    #[test]
    fn gli_importi_vanno_da_zero_al_massimo() {
        assert!(controlla_importo(0, "L'importo").is_ok());
        assert!(controlla_importo(IMPORTO_MASSIMO_CENT, "L'importo").is_ok());
        assert!(controlla_importo(-1, "L'importo").is_err());
        assert!(controlla_importo(IMPORTO_MASSIMO_CENT + 1, "L'importo").is_err());
    }

    #[test]
    fn il_pagamento_non_e_futuro_ne_di_troppi_anni_fa() {
        let oggi = data(OGGI);
        assert!(controlla_data_pagamento(oggi, oggi).is_ok());
        assert!(controlla_data_pagamento(data("2026-11-11"), oggi).is_err());
        assert!(controlla_data_pagamento(data("2016-11-09"), oggi).is_err());
    }

    #[test]
    fn il_periodo_ha_durata_e_inizio_plausibili() {
        let pagamento = data(OGGI);
        let controlla = |inizio, fine| controlla_periodo(data(inizio), data(fine), pagamento, 12);
        assert!(controlla("2026-11-10", "2027-11-10").is_ok());
        assert!(controlla("2026-11-10", "2026-11-09").is_err());
        assert!(controlla("2026-11-10", "2027-11-11").is_err());
        assert!(controlla("2028-01-01", "2028-01-31").is_err());
        assert!(controlla("2026-09-01", "2027-08-31").is_ok());
        assert!(controlla("2025-09-01", "2026-08-31").is_err());
        assert!(controlla("2025-11-09", "2025-12-09").is_err());
    }
}
