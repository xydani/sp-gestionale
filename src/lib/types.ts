/** Stato del database restituito dal backend. */
export type StatoDatabase = "da_creare" | "bloccato" | "sbloccato";

/** Errore restituito dai comandi del backend. */
export interface ErroreApp {
  codice: string;
  messaggio: string;
}

/** Esito dello sblocco: un backup automatico fallito non impedisce di lavorare. */
export interface EsitoSblocco {
  avvisoBackup: string | null;
}
