import type { StatoAbbonamento } from "./types";

export const ETICHETTE_STATO: Record<StatoAbbonamento, string> = {
  attivo: "Attivo",
  in_scadenza: "In scadenza",
  scaduto: "Scaduto",
  nessuno: "Nessun abbonamento",
};

export const STATI_ABBONAMENTO = Object.keys(ETICHETTE_STATO) as StatoAbbonamento[];
