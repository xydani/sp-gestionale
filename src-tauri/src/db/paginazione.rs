use serde::{Deserialize, Serialize};

pub const ELEMENTI_PER_PAGINA: u32 = 50;

/// Pagina richiesta dall'interfaccia, numerata da 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichiestaPagina {
    pub numero: u32,
}

impl Default for RichiestaPagina {
    fn default() -> Self {
        Self { numero: 1 }
    }
}

impl RichiestaPagina {
    /// Una pagina oltre l'ultima (es. dopo aver eliminato l'unico elemento dell'ultima pagina)
    /// diventa l'ultima che contiene elementi.
    pub fn limitata_a(self, totale_elementi: u32) -> Self {
        let ultima_pagina = totale_elementi.div_ceil(ELEMENTI_PER_PAGINA).max(1);
        Self {
            numero: self.numero.clamp(1, ultima_pagina),
        }
    }

    /// Quanti elementi saltare: una pagina 0 viene trattata come la prima.
    pub fn scarto(self) -> u32 {
        self.numero
            .max(1)
            .saturating_sub(1)
            .saturating_mul(ELEMENTI_PER_PAGINA)
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
        let pagina = |numero| RichiestaPagina { numero };
        assert_eq!(pagina(3).limitata_a(100), pagina(2));
        assert_eq!(pagina(3).limitata_a(101), pagina(3));
        assert_eq!(pagina(5).limitata_a(0), pagina(1));
        assert_eq!(pagina(0).limitata_a(10), pagina(1));
    }

    #[test]
    fn la_prima_pagina_non_salta_nulla() {
        assert_eq!(RichiestaPagina { numero: 1 }.scarto(), 0);
        assert_eq!(RichiestaPagina { numero: 0 }.scarto(), 0);
        assert_eq!(RichiestaPagina { numero: 3 }.scarto(), 100);
    }
}
