import { oggiIso } from "./format";

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
