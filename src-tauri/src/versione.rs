/// Versione dell'applicazione, letta da Cargo.toml in fase di compilazione.
pub fn versione_app() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn versione_app_corrisponde_a_cargo_toml() {
        assert_eq!(versione_app(), "0.1.0");
    }
}
