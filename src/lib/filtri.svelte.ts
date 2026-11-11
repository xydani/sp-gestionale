/**
 * Filtri delle pagine con elenco: restano in memoria finché il programma è aperto,
 * così tornando su una pagina si ritrova la ricerca lasciata.
 */
import { meseCorrente } from "./periodo";
import type { MetodoPagamento, StatoValidita } from "./types";

export interface FiltriPaginaIscritti {
  testo: string;
  statoAbbonamento: StatoValidita | "";
  statoIscrizione: StatoValidita | "";
  statoCertificato: StatoValidita | "";
  includiArchiviati: boolean;
  soloDaCompletare: boolean;
}

export interface FiltriPaginaPagamenti {
  dal: string;
  al: string;
  metodo: MetodoPagamento | "";
}

export function filtriIscrittiIniziali(): FiltriPaginaIscritti {
  return {
    testo: "",
    statoAbbonamento: "",
    statoIscrizione: "",
    statoCertificato: "",
    includiArchiviati: false,
    soloDaCompletare: false,
  };
}

/** Di partenza i pagamenti del mese in corso. */
export function filtriPagamentiIniziali(): FiltriPaginaPagamenti {
  const mese = meseCorrente();
  return { dal: mese.dal, al: mese.al, metodo: "" };
}

export const filtriIscritti = $state<FiltriPaginaIscritti>(filtriIscrittiIniziali());
export const filtriPagamenti = $state<FiltriPaginaPagamenti>(filtriPagamentiIniziali());

/** Vero se l'elenco iscritti mostra tutto, senza ricerca né filtri attivi. */
export function senzaFiltriIscritti(filtri: FiltriPaginaIscritti): boolean {
  const iniziali = filtriIscrittiIniziali();
  return (Object.keys(iniziali) as (keyof FiltriPaginaIscritti)[]).every(
    (chiave) => filtri[chiave] === iniziali[chiave],
  );
}
