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

export type UnitaDurata = "giorni" | "mesi";

export interface TipologiaAbbonamento {
  id: number;
  nome: string;
  durataValore: number;
  durataUnita: UnitaDurata;
  prezzoCent: number;
  attiva: boolean;
}

/** Dati inseriti dall'utente per creare o modificare una tipologia. */
export type DatiTipologia = Omit<TipologiaAbbonamento, "id" | "attiva">;

export type StatoAbbonamento = "attivo" | "in_scadenza" | "scaduto" | "nessuno";

export interface Iscritto {
  id: number;
  nome: string;
  cognome: string;
  telefono: string | null;
  /** Data ISO YYYY-MM-DD */
  dataIscrizione: string;
  note: string | null;
  archiviato: boolean;
  ultimaDataScadenza: string | null;
  stato: StatoAbbonamento;
}

/** Dati inseriti dall'utente per creare o modificare un iscritto. */
export type DatiIscritto = Pick<
  Iscritto,
  "nome" | "cognome" | "telefono" | "dataIscrizione" | "note"
>;

export interface FiltroIscritti {
  testo: string;
  includiArchiviati: boolean;
  stato: StatoAbbonamento | null;
}
