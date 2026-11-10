import { euroInCentesimi } from "./format";
import type { Iscritto } from "./types";

/** Vero se l'iscrizione annuale non è valida: l'abbonamento si registra comunque, con un avviso. */
export function iscrizioneNonValida(iscritto: Pick<Iscritto, "statoIscrizione">): boolean {
  return iscritto.statoIscrizione === "scaduto" || iscritto.statoIscrizione === "nessuno";
}

/** Totale da incassare per un nuovo iscritto (quota + abbonamento), null se un importo non è valido. */
export function totaleNuovoIscritto(quota: string, abbonamento: string): number | null {
  const quotaCent = euroInCentesimi(quota);
  const abbonamentoCent = euroInCentesimi(abbonamento);
  if (quotaCent === null || abbonamentoCent === null) return null;
  return quotaCent + abbonamentoCent;
}
