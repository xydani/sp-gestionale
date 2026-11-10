/// Versione dell'applicazione, letta da Cargo.toml in fase di compilazione.
pub fn versione_app() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod test {
    use super::*;

    fn versione_in(json: &str) -> String {
        let valore: serde_json::Value = serde_json::from_str(json).unwrap();
        valore["version"].as_str().unwrap().to_string()
    }

    /// Al rilascio la versione va cambiata in tre file: devono restare allineati.
    #[test]
    fn la_versione_coincide_in_cargo_tauri_e_npm() {
        assert_eq!(
            versione_in(include_str!("../tauri.conf.json")),
            versione_app()
        );
        assert_eq!(
            versione_in(include_str!("../../package.json")),
            versione_app()
        );
    }
}
