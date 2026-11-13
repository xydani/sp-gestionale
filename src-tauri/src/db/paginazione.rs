use serde::{Deserialize, Serialize};

pub const ELEMENTI_PER_PAGINA: u32 = 50;

/// Pagina richiesta dall'interfaccia, numerata da 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichiestaPagina {
    pub numero: u32,
    /// Pagine più corte per gli elenchi affiancati; mai più di [`ELEMENTI_PER_PAGINA`].
    #[serde(default = "elementi_per_pagina_predefiniti")]
    pub elementi_per_pagina: u32,
}

fn elementi_per_pagina_predefiniti() -> u32 {
    ELEMENTI_PER_PAGINA
}

impl Default for RichiestaPagina {
    fn default() -> Self {
        Self::numero(1)
    }
}

impl RichiestaPagina {
    /// La pagina indicata, con il numero di elementi predefinito.
    pub fn numero(numero: u32) -> Self {
        Self {
            numero,
            elementi_per_pagina: ELEMENTI_PER_PAGINA,
        }
    }

    /// Quanti elementi per pagina, tra 1 e il massimo.
    pub fn dimensione(self) -> u32 {
        self.elementi_per_pagina.clamp(1, ELEMENTI_PER_PAGINA)
    }

    /// Una pagina oltre l'ultima (es. dopo aver eliminato l'unico elemento dell'ultima pagina)
    /// diventa l'ultima che contiene elementi.
    pub fn limitata_a(self, totale_elementi: u32) -> Self {
        let ultima_pagina = totale_elementi.div_ceil(self.dimensione()).max(1);
        Self {
            numero: self.numero.clamp(1, ultima_pagina),
            ..self
        }
    }

    /// Quanti elementi saltare: una pagina 0 viene trattata come la prima.
    pub fn scarto(self) -> u32 {
        self.numero
            .max(1)
            .saturating_sub(1)
            .saturating_mul(self.dimensione())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagina<T> {
    pub elementi: Vec<T>,
    pub numero: u32,
    pub elementi_per_pagina: u32,
    /// Elementi totali che corrispondono al filtro, su tutte le pagine.
    pub totale_elementi: u32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn una_pagina_oltre_l_ultima_diventa_l_ultima() {
        let pagina = RichiestaPagina::numero;
        assert_eq!(pagina(3).limitata_a(100), pagina(2));
        assert_eq!(pagina(3).limitata_a(101), pagina(3));
        assert_eq!(pagina(5).limitata_a(0), pagina(1));
        assert_eq!(pagina(0).limitata_a(10), pagina(1));
    }

    #[test]
    fn la_prima_pagina_non_salta_nulla() {
        assert_eq!(RichiestaPagina::numero(1).scarto(), 0);
        assert_eq!(RichiestaPagina::numero(0).scarto(), 0);
        assert_eq!(RichiestaPagina::numero(3).scarto(), 100);
    }

    fn pagina_da(numero: u32, elementi_per_pagina: u32) -> RichiestaPagina {
        RichiestaPagina {
            numero,
            elementi_per_pagina,
        }
    }

    #[test]
    fn le_pagine_corte_saltano_e_limitano_secondo_la_loro_dimensione() {
        assert_eq!(pagina_da(3, 10).scarto(), 20);
        assert_eq!(pagina_da(9, 10).limitata_a(25), pagina_da(3, 10));
    }

    #[test]
    fn la_dimensione_resta_tra_uno_e_il_massimo() {
        assert_eq!(pagina_da(1, 0).dimensione(), 1);
        assert_eq!(pagina_da(1, 500).dimensione(), ELEMENTI_PER_PAGINA);
    }

    #[test]
    fn senza_dimensione_la_pagina_e_quella_predefinita() {
        let richiesta: RichiestaPagina = serde_json::from_str(r#"{"numero": 2}"#).unwrap();
        assert_eq!(richiesta, RichiestaPagina::numero(2));
    }
}
