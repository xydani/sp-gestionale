import type { StatoValidita } from "./types";

export const ETICHETTE_STATO: Record<StatoValidita, string> = {
  attivo: "Attivo",
  in_scadenza: "In scadenza",
  scaduto: "Scaduto",
  nessuno: "Assente",
};

export const STATI_VALIDITA = Object.keys(ETICHETTE_STATO) as StatoValidita[];
