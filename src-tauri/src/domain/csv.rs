//! Testo CSV leggibile da Excel in italiano: separatore `;`, virgola decimale, BOM UTF-8.

const SEPARATORE: char = ';';
const BOM_UTF8: &str = "\u{feff}";
const FINE_RIGA: &str = "\r\n";

/// Unisce intestazione e righe in un testo CSV.
pub fn componi_csv(intestazione: &[&str], righe: &[Vec<String>]) -> String {
    let mut testo = String::from(BOM_UTF8);
    let intestazione: Vec<String> = intestazione.iter().map(|campo| campo.to_string()).collect();
    for riga in std::iter::once(&intestazione).chain(righe) {
        let campi: Vec<String> = riga.iter().map(|campo| proteggi_campo(campo)).collect();
        testo.push_str(&campi.join(&SEPARATORE.to_string()));
        testo.push_str(FINE_RIGA);
    }
    testo
}

/// Racchiude tra virgolette i campi con separatori, virgolette o a capo.
fn proteggi_campo(campo: &str) -> String {
    if campo.contains([SEPARATORE, '"', '\n', '\r']) {
        format!("\"{}\"", campo.replace('"', "\"\""))
    } else {
        campo.to_string()
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
    fn importi_con_virgola_decimale() {
        assert_eq!(importo_per_csv(5000), "50,00");
        assert_eq!(importo_per_csv(123_405), "1234,05");
        assert_eq!(importo_per_csv(7), "0,07");
        assert_eq!(importo_per_csv(-250), "-2,50");
    }
}
