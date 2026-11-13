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
  const minimaIso = scadenzaMinimaPregressa(oggi);
  if (!tipologia) return { minimaIso, massimaIso: null };
  return { minimaIso, massimaIso: calcolaDataScadenza(oggi, tipologia) };
}

/** Inizio ammesso: non dopo oggi, e non così indietro da essere scaduto da troppo. */
export function limitiInizioPregresso(
  tipologia: TipologiaAbbonamento | undefined,
  oggi: string = oggiIso(),
): LimitiData {
  if (!tipologia) return { minimaIso: null, massimaIso: oggi };
  const minimaIso = calcolaDataInizioDaScadenza(scadenzaMinimaPregressa(oggi), tipologia);
  return { minimaIso, massimaIso: oggi };
}

function scadenzaMinimaPregressa(oggi: string): string {
  return aggiungiMesiIso(oggi, -MESI_MASSIMI_SCADENZA_PREGRESSA_PASSATA);
}

/**
 * Come nel programma: inizio + durata − 1 giorno. Se il mese d'arrivo è più corto
 * (31 gennaio + 1 mese) la scadenza è l'ultimo giorno di quel mese.
 */
export function calcolaDataScadenza(inizioIso: string, tipologia: TipologiaAbbonamento): string {
  if (tipologia.durataUnita === "giorni") {
    return aggiungiGiorniIso(inizioIso, tipologia.durataValore - 1);
  }
  const stessoGiornoDopo = aggiungiMesiIso(inizioIso, tipologia.durataValore);
  const troncatoAFineMese = giornoDelMese(stessoGiornoDopo) < giornoDelMese(inizioIso);
  return troncatoAFineMese ? stessoGiornoDopo : aggiungiGiorniIso(stessoGiornoDopo, -1);
}

/** L'inverso, come nel programma: il giorno dopo la scadenza meno la durata. */
export function calcolaDataInizioDaScadenza(
  scadenzaIso: string,
  tipologia: TipologiaAbbonamento,
): string {
  if (tipologia.durataUnita === "giorni") {
    return aggiungiGiorniIso(scadenzaIso, -(tipologia.durataValore - 1));
  }
  return aggiungiMesiIso(aggiungiGiorniIso(scadenzaIso, 1), -tipologia.durataValore);
}

function giornoDelMese(dataIso: string): number {
  return Number(dataIso.slice(8, 10));
}

function aggiungiGiorniIso(dataIso: string, giorni: number): string {
  const [anno, mese, giorno] = dataIso.split("-").map(Number);
  return oggiIso(new Date(anno, mese - 1, giorno + giorni));
}

/** La data dell'abbonamento scritta a mano: l'altra si calcola. */
export type DataIndicata = "inizio" | "scadenza";

export interface DateAbbonamento {
  inizioIso: string;
  scadenzaIso: string;
}

/** Ricalcola la data non indicata a mano, ad esempio dopo un cambio di tipologia. */
export function completaDateAbbonamento(
  date: DateAbbonamento,
  indicata: DataIndicata | null,
  tipologia: TipologiaAbbonamento | undefined,
): DateAbbonamento {
  if (!tipologia) return date;
  if (indicata === "inizio" && date.inizioIso) {
    return { ...date, scadenzaIso: calcolaDataScadenza(date.inizioIso, tipologia) };
  }
  if (indicata === "scadenza" && date.scadenzaIso) {
    return { ...date, inizioIso: calcolaDataInizioDaScadenza(date.scadenzaIso, tipologia) };
  }
  return date;
}

/** Al programma va solo la data scritta a mano: l'altra la ricava lui con la stessa regola. */
export function dateDaInviare(
  date: DateAbbonamento,
  indicata: DataIndicata | null,
): { dataInizioAbbonamento: string | null; dataScadenzaAbbonamento: string | null } {
  return {
    dataInizioAbbonamento: indicata === "inizio" ? date.inizioIso || null : null,
    dataScadenzaAbbonamento: indicata === "scadenza" ? date.scadenzaIso || null : null,
  };
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
