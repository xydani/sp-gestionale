import { oggiIso } from "./format";
import type { Pagamento } from "./types";

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

export function sommaImporti(pagamenti: Pick<Pagamento, "importoCent">[]): number {
  return pagamenti.reduce((totale, pagamento) => totale + pagamento.importoCent, 0);
}
