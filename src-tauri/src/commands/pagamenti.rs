use chrono::NaiveDate;
use tauri::State;

use crate::db::pagamenti::{
    self, DatiPagamento, FiltroPagamenti, Pagamento, PaginaPagamenti, PropostaPagamento,
};
use crate::db::paginazione::RichiestaPagina;
use crate::errore::ErroreApp;
use crate::stato_app::StatoApp;

#[tauri::command]
pub fn proponi_pagamento(
    stato: State<'_, StatoApp>,
    iscritto_id: i64,
    tipologia_id: i64,
    data_inizio: Option<NaiveDate>,
) -> Result<PropostaPagamento, ErroreApp> {
    let oggi = stato.oggi();
    stato.con_connessione(|connessione| {
        pagamenti::proponi_pagamento(connessione, iscritto_id, tipologia_id, data_inizio, oggi)
    })
}

#[tauri::command]
pub fn registra_pagamento(
    stato: State<'_, StatoApp>,
    dati: DatiPagamento,
) -> Result<Pagamento, ErroreApp> {
    stato.con_connessione(|connessione| pagamenti::registra_pagamento(connessione, &dati))
}

#[tauri::command]
pub fn elenca_pagamenti(
    stato: State<'_, StatoApp>,
    filtro: FiltroPagamenti,
    pagina: RichiestaPagina,
) -> Result<PaginaPagamenti, ErroreApp> {
    stato.con_connessione(|connessione| {
        pagamenti::elenca_pagamenti_paginati(connessione, &filtro, pagina)
    })
}

#[tauri::command]
pub fn elimina_pagamento(stato: State<'_, StatoApp>, id: i64) -> Result<(), ErroreApp> {
    stato.con_connessione(|connessione| pagamenti::elimina_pagamento(connessione, id))
}
