/// Testo in minuscolo e senza accenti, per confronti e ricerche ("Nicolò" → "nicolo").
pub fn normalizza_per_ricerca(testo: &str) -> String {
    testo
        .chars()
        .flat_map(char::to_lowercase)
        .map(togli_accento)
        .collect()
}

/// Divide la ricerca in parole già normalizzate, ignorando gli spazi in eccesso.
pub fn parole_di_ricerca(testo: &str) -> Vec<String> {
    normalizza_per_ricerca(testo)
        .split_whitespace()
        .map(str::to_string)
        .collect()
}

fn togli_accento(carattere: char) -> char {
    match carattere {
        'à' | 'á' | 'â' | 'ä' | 'ã' | 'å' => 'a',
        'è' | 'é' | 'ê' | 'ë' => 'e',
        'ì' | 'í' | 'î' | 'ï' => 'i',
        'ò' | 'ó' | 'ô' | 'ö' | 'õ' => 'o',
        'ù' | 'ú' | 'û' | 'ü' => 'u',
        'ç' => 'c',
        'ñ' => 'n',
        altro => altro,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rende_minuscolo_e_toglie_gli_accenti() {
        assert_eq!(normalizza_per_ricerca("Nicolò"), "nicolo");
        assert_eq!(normalizza_per_ricerca("D'ANGELO"), "d'angelo");
        assert_eq!(normalizza_per_ricerca("Pérez Núñez"), "perez nunez");
        assert_eq!(normalizza_per_ricerca("ÈLÌA"), "elia");
    }

    #[test]
    fn divide_la_ricerca_in_parole() {
        assert_eq!(parole_di_ricerca("  Rossi   Mario "), ["rossi", "mario"]);
        assert!(parole_di_ricerca("   ").is_empty());
    }
}
