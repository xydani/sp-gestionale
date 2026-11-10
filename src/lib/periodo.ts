import { oggiIso } from "./format";
import type { Raggruppamento } from "./types";

export interface Periodo {
  /** Date ISO, estremi inclusi. */
  dal: string;
  al: string;
}

/** Dal primo all'ultimo giorno del mese della data indicata. */
export function meseCorrente(adesso: Date = new Date()): Periodo {
  const primoGiorno = new Date(adesso.getFullYear(), adesso.getMonth(), 1);
  const ultimoGiorno = new Date(adesso.getFullYear(), adesso.getMonth() + 1, 0);
  return { dal: oggiIso(primoGiorno), al: oggiIso(ultimoGiorno) };
}

export type PeriodoRapido = "oggi" | "mese" | "mese_scorso" | "anno" | "anno_scorso";

export const ETICHETTE_PERIODO_RAPIDO: Record<PeriodoRapido, string> = {
  oggi: "Oggi",
  mese: "Questo mese",
  mese_scorso: "Mese scorso",
  anno: "Quest'anno",
  anno_scorso: "Anno scorso",
};

/** Le date di un periodo rapido rispetto al giorno indicato. */
export function calcolaPeriodoRapido(scelta: PeriodoRapido, adesso: Date = new Date()): Periodo {
  const anno = adesso.getFullYear();
  switch (scelta) {
    case "oggi":
      return { dal: oggiIso(adesso), al: oggiIso(adesso) };
    case "mese":
      return meseCorrente(adesso);
    case "mese_scorso":
      return meseCorrente(new Date(anno, adesso.getMonth() - 1, 1));
    case "anno":
      return { dal: `${anno}-01-01`, al: `${anno}-12-31` };
    case "anno_scorso":
      return { dal: `${anno - 1}-01-01`, al: `${anno - 1}-12-31` };
  }
}

const GIORNI_MASSIMI_PER_GIORNO = 31;
const GIORNI_MASSIMI_PER_MESE = 731;

/** Raggruppamento leggibile per la lunghezza del periodo: giorni, mesi o anni. */
export function raggruppamentoAdatto(periodo: Periodo): Raggruppamento {
  const millisecondiAlGiorno = 86_400_000;
  const giorni = (Date.parse(periodo.al) - Date.parse(periodo.dal)) / millisecondiAlGiorno + 1;
  if (giorni <= GIORNI_MASSIMI_PER_GIORNO) return "giorno";
  if (giorni <= GIORNI_MASSIMI_PER_MESE) return "mese";
  return "anno";
}
