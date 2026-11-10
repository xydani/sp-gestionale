//! Logo scelto dallo staff: una copia sta nella cartella dati, accanto all'archivio.
//! Non è criptato (non contiene dati personali) così si vede già nella schermata della password.

use std::fs;
use std::path::{Path, PathBuf};

use base64::Engine;
use base64::engine::general_purpose::STANDARD;

use crate::errore::ErroreApp;

const NOME_FILE_LOGO: &str = "logo-palestra";
pub const DIMENSIONE_MASSIMA_LOGO_BYTE: u64 = 2 * 1024 * 1024;

/// Formati accettati, con il tipo MIME con cui l'immagine viene mostrata.
const FORMATI_LOGO: [(&str, &str); 5] = [
    ("png", "image/png"),
    ("jpg", "image/jpeg"),
    ("jpeg", "image/jpeg"),
    ("webp", "image/webp"),
    ("svg", "image/svg+xml"),
];

/// Copia l'immagine scelta come nuovo logo, sostituendo quello precedente.
pub fn salva_logo(cartella_dati: &Path, origine: &Path) -> Result<(), ErroreApp> {
    let estensione = estensione_accettata(origine)?;
    let dimensione = fs::metadata(origine).map_err(errore_file)?.len();
    if dimensione > DIMENSIONE_MASSIMA_LOGO_BYTE {
        return Err(ErroreApp::nuovo(
            "logo_troppo_grande",
            "L'immagine è troppo grande: il massimo è 2 MB",
        ));
    }
    fs::create_dir_all(cartella_dati).map_err(errore_file)?;
    rimuovi_logo(cartella_dati)?;
    fs::copy(origine, percorso_logo(cartella_dati, estensione)).map_err(errore_file)?;
    Ok(())
}

/// Il logo come indirizzo `data:` pronto per un `<img>`, oppure `None` se non è stato scelto.
pub fn leggi_logo(cartella_dati: &Path) -> Result<Option<String>, ErroreApp> {
    for (estensione, tipo_mime) in FORMATI_LOGO {
        let percorso = percorso_logo(cartella_dati, estensione);
        if percorso.exists() {
            let contenuto = fs::read(percorso).map_err(errore_file)?;
            return Ok(Some(format!(
                "data:{tipo_mime};base64,{}",
                STANDARD.encode(contenuto)
            )));
        }
    }
    Ok(None)
}

/// Torna allo stemma predefinito.
pub fn rimuovi_logo(cartella_dati: &Path) -> Result<(), ErroreApp> {
    for (estensione, _) in FORMATI_LOGO {
        let percorso = percorso_logo(cartella_dati, estensione);
        if percorso.exists() {
            fs::remove_file(percorso).map_err(errore_file)?;
        }
    }
    Ok(())
}

fn estensione_accettata(origine: &Path) -> Result<&'static str, ErroreApp> {
    let estensione = origine
        .extension()
        .and_then(|estensione| estensione.to_str())
        .map(str::to_lowercase)
        .unwrap_or_default();
    FORMATI_LOGO
        .iter()
        .map(|(accettata, _)| *accettata)
        .find(|accettata| *accettata == estensione)
        .ok_or_else(|| {
            ErroreApp::nuovo(
                "formato_logo_non_valido",
                "Formato non supportato: scegliere un'immagine PNG, JPG, WEBP o SVG",
            )
        })
}

fn percorso_logo(cartella_dati: &Path, estensione: &str) -> PathBuf {
    cartella_dati.join(format!("{NOME_FILE_LOGO}.{estensione}"))
}

fn errore_file(errore: std::io::Error) -> ErroreApp {
    ErroreApp::nuovo("errore_file", format!("Errore sul file del logo: {errore}"))
}

#[cfg(test)]
mod test {
    use super::*;

    fn immagine(cartella: &Path, nome: &str, contenuto: &[u8]) -> PathBuf {
        let percorso = cartella.join(nome);
        fs::write(&percorso, contenuto).unwrap();
        percorso
    }

    #[test]
    fn senza_logo_scelto_non_restituisce_nulla() {
        let cartella = tempfile::tempdir().unwrap();
        assert_eq!(leggi_logo(cartella.path()).unwrap(), None);
    }

    #[test]
    fn il_logo_salvato_si_rilegge_come_data_url() {
        let cartella = tempfile::tempdir().unwrap();
        let origine = immagine(cartella.path(), "Stemma.PNG", b"png");
        let dati = cartella.path().join("dati");

        salva_logo(&dati, &origine).unwrap();

        assert_eq!(
            leggi_logo(&dati).unwrap().as_deref(),
            Some("data:image/png;base64,cG5n")
        );
    }

    #[test]
    fn un_nuovo_logo_sostituisce_il_precedente_anche_di_altro_formato() {
        let cartella = tempfile::tempdir().unwrap();
        salva_logo(cartella.path(), &immagine(cartella.path(), "a.png", b"a")).unwrap();
        salva_logo(
            cartella.path(),
            &immagine(cartella.path(), "b.svg", b"<svg/>"),
        )
        .unwrap();

        let logo = leggi_logo(cartella.path()).unwrap().unwrap();
        assert!(logo.starts_with("data:image/svg+xml;base64,"));
        assert!(!percorso_logo(cartella.path(), "png").exists());
    }

    #[test]
    fn rifiuta_formati_non_immagine_e_file_troppo_grandi() {
        let cartella = tempfile::tempdir().unwrap();
        let documento = immagine(cartella.path(), "logo.pdf", b"pdf");
        assert_eq!(
            salva_logo(cartella.path(), &documento).unwrap_err().codice,
            "formato_logo_non_valido"
        );

        let grande = immagine(
            cartella.path(),
            "grande.png",
            &vec![0; DIMENSIONE_MASSIMA_LOGO_BYTE as usize + 1],
        );
        assert_eq!(
            salva_logo(cartella.path(), &grande).unwrap_err().codice,
            "logo_troppo_grande"
        );
        assert_eq!(leggi_logo(cartella.path()).unwrap(), None);
    }

    #[test]
    fn dopo_la_rimozione_torna_lo_stemma() {
        let cartella = tempfile::tempdir().unwrap();
        salva_logo(cartella.path(), &immagine(cartella.path(), "a.jpg", b"a")).unwrap();
        rimuovi_logo(cartella.path()).unwrap();
        assert_eq!(leggi_logo(cartella.path()).unwrap(), None);
    }
}
