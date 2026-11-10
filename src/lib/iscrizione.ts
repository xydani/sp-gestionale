import type { Iscritto } from "./types";

/** Vero se l'iscrizione annuale non è valida: l'abbonamento si registra comunque, con un avviso. */
export function iscrizioneNonValida(iscritto: Pick<Iscritto, "statoIscrizione">): boolean {
  return iscritto.statoIscrizione === "scaduto" || iscritto.statoIscrizione === "nessuno";
}
