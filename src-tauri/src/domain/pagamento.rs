use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetodoPagamento {
    Contanti,
    Carta,
    Bonifico,
    Altro,
}

impl MetodoPagamento {
    pub const TUTTI: [MetodoPagamento; 4] =
        [Self::Contanti, Self::Carta, Self::Bonifico, Self::Altro];

    /// Valore salvato nel database (vincolo CHECK della tabella pagamenti).
    pub fn come_testo(self) -> &'static str {
        match self {
            Self::Contanti => "contanti",
            Self::Carta => "carta",
            Self::Bonifico => "bonifico",
            Self::Altro => "altro",
        }
    }

    pub fn da_testo(testo: &str) -> Option<Self> {
        Self::TUTTI
            .into_iter()
            .find(|metodo| metodo.come_testo() == testo)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testo_andata_e_ritorno() {
        for metodo in MetodoPagamento::TUTTI {
            assert_eq!(MetodoPagamento::da_testo(metodo.come_testo()), Some(metodo));
        }
        assert_eq!(MetodoPagamento::da_testo("assegno"), None);
    }

    #[test]
    fn serializzazione_uguale_al_testo_del_database() {
        for metodo in MetodoPagamento::TUTTI {
            assert_eq!(
                serde_json::to_string(&metodo).unwrap(),
                format!("\"{}\"", metodo.come_testo())
            );
        }
    }
}
