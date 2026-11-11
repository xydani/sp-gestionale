import { avvisoCertificato } from "./certificato";
import { formattaData } from "./format";
import type { Iscritto, StatoValidita } from "./types";

/** Dove si sistema ciò che manca: la sezione da aprire nella scheda. */
export type AzioneMancanza = "iscrizione" | "abbonamento" | "anagrafica";

export interface Mancanza {
  testo: string;
  azione: AzioneMancanza;
  /** Già scaduto o assente (rosso) oppure solo in scadenza (giallo). */
  urgente: boolean;
}

/** Tutto ciò che va sistemato per l'iscritto, in un unico elenco; vuoto se è in regola. */
export function cosaManca(iscritto: Iscritto): Mancanza[] {
  if (iscritto.archiviato) return [];
  const mancanze: Mancanza[] = [];
  aggiungiSeNonValido(mancanze, iscritto.statoIscrizione, "iscrizione", {
    nessuno: "Quota annuale mai pagata",
    scaduto: "Quota annuale scaduta",
    in_scadenza: `Quota annuale in scadenza il ${dataOppureVuoto(iscritto.ultimaDataFineIscrizione)}`,
  });
  aggiungiSeNonValido(mancanze, iscritto.statoAbbonamento, "abbonamento", {
    nessuno: "Nessun abbonamento",
    scaduto: "Abbonamento scaduto",
    in_scadenza: `Abbonamento in scadenza il ${dataOppureVuoto(iscritto.ultimaDataScadenzaAbbonamento)}`,
  });
  const certificato = avvisoCertificato(iscritto);
  if (certificato) {
    mancanze.push({
      testo: certificato,
      azione: "anagrafica",
      urgente: iscritto.statoCertificato !== "in_scadenza",
    });
  }
  if (iscritto.daCompletare) {
    mancanze.push({
      testo: "Mancano data o luogo di nascita",
      azione: "anagrafica",
      urgente: false,
    });
  }
  return mancanze;
}

type TestiStato = Record<Exclude<StatoValidita, "attivo">, string>;

function aggiungiSeNonValido(
  mancanze: Mancanza[],
  stato: StatoValidita,
  azione: AzioneMancanza,
  testi: TestiStato,
) {
  if (stato === "attivo") return;
  mancanze.push({ testo: testi[stato], azione, urgente: stato !== "in_scadenza" });
}

function dataOppureVuoto(data: string | null): string {
  return data ? formattaData(data) : "";
}
