/**
 * Controlli immediati sui campi, gli stessi di src-tauri/src/domain/validazione.rs:
 * il programma segnala l'errore sul campo prima ancora di salvare.
 */
import { euroInCentesimi, oggiIso } from "./format";

export const LUNGHEZZA_MASSIMA_NOME = 60;
export const LUNGHEZZA_MASSIMA_LUOGO = 80;
export const LUNGHEZZA_MASSIMA_NOTE = 1000;
const CIFRE_TELEFONO_MINIME = 6;
const CIFRE_TELEFONO_MASSIME = 15;
const ETA_MASSIMA_ANNI = 120;
const VALIDITA_MASSIMA_CERTIFICATO_MESI = 13;
const ANNI_MASSIMI_PAGAMENTO_PASSATO = 10;
export const DURATA_MASSIMA_ABBONAMENTO_MESI = 36;
export const DURATA_ISCRIZIONE_MESI = 12;

/** Limite oltre il quale un importo è sicuramente un errore di battitura (10.000 €). */
export const IMPORTO_MASSIMO_CENT = 1_000_000;

export type Verifica = (valore: string) => string | null;

/** Lettere (anche accentate), spazi, apostrofi, trattini e punti, con almeno una lettera. */
export function erroreNomeProprio(campo: string, testo: string): string | null {
  const pulito = testo.trim();
  if (pulito === "") return null;
  if (!/^[\p{L} '’.-]+$/u.test(pulito) || !/\p{L}/u.test(pulito)) {
    return `${campo} può contenere solo lettere, spazi, apostrofi e trattini`;
  }
  return null;
}

export const erroreTelefono: Verifica = (testo) => {
  const pulito = testo.trim();
  if (pulito === "") return null;
  const senzaPrefisso = pulito.startsWith("+") ? pulito.slice(1) : pulito;
  const cifre = senzaPrefisso.replace(/\D/g, "").length;
  if (
    !/^[\d .\-/()]+$/.test(senzaPrefisso) ||
    cifre < CIFRE_TELEFONO_MINIME ||
    cifre > CIFRE_TELEFONO_MASSIME
  ) {
    return `Telefono non valido: servono da ${CIFRE_TELEFONO_MINIME} a ${CIFRE_TELEFONO_MASSIME} cifre (es. 333 1234567)`;
  }
  return null;
};

export const erroreEmail: Verifica = (testo) => {
  const pulito = testo.trim();
  if (pulito === "") return null;
  return /^[^\s@]+@[^\s@.]+(\.[^\s@.]+)+$/.test(pulito)
    ? null
    : "Email non valida (es. nome@esempio.it)";
};

/** Importo in euro scritto a mano: un numero da 0 al massimo, con al più due decimali. */
export const erroreImportoEuro: Verifica = (testo) => {
  if (testo.trim() === "") return null;
  const cent = euroInCentesimi(testo);
  if (cent === null) return "Importo non valido (es. 30 oppure 30,50)";
  if (cent > IMPORTO_MASSIMO_CENT) {
    return `Importo troppo alto: il massimo è ${IMPORTO_MASSIMO_CENT / 100} €`;
  }
  return null;
};

/** Somma mesi a una data ISO; se il giorno non esiste (31 febbraio) usa l'ultimo del mese. */
export function aggiungiMesiIso(dataIso: string, mesi: number): string {
  const [anno, mese, giorno] = dataIso.split("-").map(Number);
  const indiceMese = anno * 12 + (mese - 1) + mesi;
  const nuovoAnno = Math.floor(indiceMese / 12);
  const nuovoMese = (indiceMese % 12) + 1;
  const ultimoGiorno = new Date(Date.UTC(nuovoAnno, nuovoMese, 0)).getUTCDate();
  const giornoValido = Math.min(giorno, ultimoGiorno);
  return `${String(nuovoAnno).padStart(4, "0")}-${String(nuovoMese).padStart(2, "0")}-${String(giornoValido).padStart(2, "0")}`;
}

export interface LimitiData {
  minimaIso: string | null;
  massimaIso: string | null;
}

export function limitiDataNascita(oggi: string = oggiIso()): LimitiData {
  return { minimaIso: aggiungiMesiIso(oggi, -ETA_MASSIMA_ANNI * 12), massimaIso: oggi };
}

export function limitiScadenzaCertificato(oggi: string = oggiIso()): LimitiData {
  return { minimaIso: null, massimaIso: aggiungiMesiIso(oggi, VALIDITA_MASSIMA_CERTIFICATO_MESI) };
}

/** Si registra un incasso già avvenuto: non nel futuro e non anni fa. */
export function limitiDataPagamento(oggi: string = oggiIso()): LimitiData {
  return {
    minimaIso: aggiungiMesiIso(oggi, -ANNI_MASSIMI_PAGAMENTO_PASSATO * 12),
    massimaIso: oggi,
  };
}

/** La fine del periodo non precede l'inizio e non lo supera di più della durata massima. */
export function limitiFinePeriodo(dataInizio: string, durataMassimaMesi: number): LimitiData {
  if (!dataInizio) return { minimaIso: null, massimaIso: null };
  return { minimaIso: dataInizio, massimaIso: aggiungiMesiIso(dataInizio, durataMassimaMesi) };
}

/**
 * Azione Svelte: il messaggio (o null) blocca l'invio del modulo e compare sul campo.
 * Riceve il messaggio già calcolato dal valore, così vale anche quando il programma riempie il campo.
 */
export function validita(campo: HTMLInputElement, messaggio: string | null) {
  campo.setCustomValidity(messaggio ?? "");
  return {
    update(nuovo: string | null) {
      campo.setCustomValidity(nuovo ?? "");
    },
  };
}
