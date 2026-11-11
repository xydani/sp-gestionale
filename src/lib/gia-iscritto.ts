import { oggiIso } from "./format";
import type { Iscritto, TipologiaAbbonamento } from "./types";
import { aggiungiMesiIso, type LimitiData } from "./validazione";

/** Un abbonamento scaduto da più di così non è più in corso (stesso limite del programma). */
export const MESI_MASSIMI_SCADENZA_PREGRESSA_PASSATA = 12;

/**
 * Scadenza ammessa per un abbonamento già in corso: l'abbonamento deve essere già iniziato,
 * quindi scade al più tardi come uno che parte oggi.
 */
export function limitiScadenzaPregressa(
  tipologia: TipologiaAbbonamento | undefined,
  oggi: string = oggiIso(),
): LimitiData {
  const minimaIso = aggiungiMesiIso(oggi, -MESI_MASSIMI_SCADENZA_PREGRESSA_PASSATA);
  if (!tipologia) return { minimaIso, massimaIso: null };
  return { minimaIso, massimaIso: scadenzaDiChiIniziaOggi(tipologia, oggi) };
}

function scadenzaDiChiIniziaOggi(tipologia: TipologiaAbbonamento, oggi: string): string {
  if (tipologia.durataUnita === "giorni") {
    return aggiungiGiorniIso(oggi, tipologia.durataValore - 1);
  }
  const stessoGiornoDopo = aggiungiMesiIso(oggi, tipologia.durataValore);
  return aggiungiGiorniIso(stessoGiornoDopo, -1);
}

function aggiungiGiorniIso(dataIso: string, giorni: number): string {
  const [anno, mese, giorno] = dataIso.split("-").map(Number);
  return oggiIso(new Date(anno, mese - 1, giorno + giorni));
}

/** Confronto senza maiuscole, accenti e spazi in più: «Nicolò  Rossi» = «nicolo rossi». */
function normalizzaNome(testo: string): string {
  return testo
    .normalize("NFD")
    .replace(/\p{Diacritic}/gu, "")
    .toLowerCase()
    .trim()
    .replace(/\s+/g, " ");
}

/** Iscritti già presenti con lo stesso nome e cognome: forse è la stessa persona. */
export function trovaOmonimi(iscritti: Iscritto[], nome: string, cognome: string): Iscritto[] {
  const nomeCercato = normalizzaNome(nome);
  const cognomeCercato = normalizzaNome(cognome);
  return iscritti.filter(
    (iscritto) =>
      normalizzaNome(iscritto.nome) === nomeCercato &&
      normalizzaNome(iscritto.cognome) === cognomeCercato,
  );
}
