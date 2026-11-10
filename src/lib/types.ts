/** Stato del database restituito dal backend. */
export type StatoDatabase = "da_creare" | "bloccato" | "sbloccato";

/** Errore restituito dai comandi del backend. */
export interface ErroreApp {
  codice: string;
  messaggio: string;
}
