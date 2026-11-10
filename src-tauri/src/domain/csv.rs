//! Testo CSV leggibile da Excel in italiano: separatore `;`, virgola decimale, BOM UTF-8.

const SEPARATORE: char = ';';
const BOM_UTF8: &str = "\u{feff}";
const FINE_RIGA: &str = "\r\n";

/// Unisce intestazione e righe in un testo CSV.
pub fn componi_csv(intestazione: &[&str], righe: &[Vec<String>]) -> String {
    let mut testo = String::from(BOM_UTF8);
    testo.push_str(&componi_riga(intestazione));
    for riga in righe {
        testo.push_str(&componi_riga(riga));
    }
    testo
}

fn componi_riga(campi: &[impl AsRef<str>]) -> String {
    let protetti: Vec<String> = campi
        .iter()
        .map(|campo| proteggi_campo(campo.as_ref()))
        .collect();
    let mut riga = protetti.join(&SEPARATORE.to_string());
    riga.push_str(FINE_RIGA);
    riga
}

/// Racchiude tra virgolette i campi con separatori, virgolette o a capo,
/// e disinnesca quelli che Excel eseguirebbe come formula.
fn proteggi_campo(campo: &str) -> String {
    let mut testo = campo.to_string();
    if sembra_una_formula(campo) {
        testo.insert(0, '\'');
    }
    if testo.contains([SEPARATORE, '"', '\n', '\r']) {
        testo = format!("\"{}\"", testo.replace('"', "\"\""));
    }
    testo
}

/// Un testo che inizia con = + @ (o - non seguito da una cifra) verrebbe calcolato da Excel:
/// un nome nel listino come "=HYPERLINK(...)" diventerebbe un collegamento eseguibile.
fn sembra_una_formula(campo: &str) -> bool {
    let mut caratteri = campo.chars();
    match caratteri.next() {
        Some('=' | '+' | '@' | '\t' | '\r') => true,
        Some('-') => !caratteri.next().is_some_and(|c| c.is_ascii_digit()),
        _ => false,
    }
}

/// Importo in centesimi come numero con virgola decimale, senza simbolo: `1234,50`.
pub fn importo_per_csv(importo_cent: i64) -> String {
    let segno = if importo_cent < 0 { "-" } else { "" };
    let assoluto = importo_cent.unsigned_abs();
    format!("{segno}{},{:02}", assoluto / 100, assoluto % 100)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compone_intestazione_e_righe_con_bom() {
        let testo = componi_csv(&["a", "b"], &[vec!["1".into(), "2".into()]]);
        assert_eq!(testo, "\u{feff}a;b\r\n1;2\r\n");
    }

    #[test]
    fn protegge_separatori_e_virgolette() {
        assert_eq!(proteggi_campo("D'Angelo; Rossi"), "\"D'Angelo; Rossi\"");
        assert_eq!(proteggi_campo("detto \"Gino\""), "\"detto \"\"Gino\"\"\"");
        assert_eq!(proteggi_campo("semplice"), "semplice");
    }

    #[test]
    fn disinnesca_le_formule_ma_non_i_numeri_negativi() {
        assert_eq!(proteggi_campo("=SOMMA(A1)"), "'=SOMMA(A1)");
        assert_eq!(proteggi_campo("+39 333"), "'+39 333");
        assert_eq!(proteggi_campo("@cmd"), "'@cmd");
        assert_eq!(proteggi_campo("-cmd"), "'-cmd");
        assert_eq!(proteggi_campo("-12,50"), "-12,50");
        assert_eq!(proteggi_campo("Mensile"), "Mensile");
    }

    #[test]
    fn importi_con_virgola_decimale() {
        assert_eq!(importo_per_csv(5000), "50,00");
        assert_eq!(importo_per_csv(123_405), "1234,05");
        assert_eq!(importo_per_csv(7), "0,07");
        assert_eq!(importo_per_csv(-250), "-2,50");
    }
}
