import type { StatoValidita } from "./types";

export const ETICHETTE_STATO: Record<StatoValidita, string> = {
  attivo: "Attivo",
  in_scadenza: "In scadenza",
  scaduto: "Scaduto",
  nessuno: "Assente",
};

export const STATI_VALIDITA = Object.keys(ETICHETTE_STATO) as StatoValidita[];

/** Opzioni del filtro per stato: "Iscrizione: tutte", "Iscrizione: attivo", … */
export function opzioniStato(
  prefisso: string,
  testoTutti: string,
): { valore: StatoValidita | ""; etichetta: string }[] {
  return [
    { valore: "", etichetta: `${prefisso}: ${testoTutti}` },
    ...STATI_VALIDITA.map((stato) => ({
      valore: stato,
      etichetta: `${prefisso}: ${ETICHETTE_STATO[stato].toLowerCase()}`,
    })),
  ];
}
