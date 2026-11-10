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

export type MetodoPagamento = "contanti" | "carta" | "bonifico" | "altro";

export interface Pagamento {
  id: number;
  iscrittoId: number;
  iscrittoNome: string;
  iscrittoCognome: string;
  tipologiaId: number | null;
  tipologiaNome: string;
  importoCent: number;
  dataPagamento: string;
  metodo: MetodoPagamento;
  dataInizio: string;
  dataScadenza: string;
  note: string | null;
  rinnovoDiId: number | null;
}

/** Dati inseriti dall'utente per registrare un pagamento. */
export interface DatiPagamento {
  iscrittoId: number;
  tipologiaId: number;
  importoCent: number;
  dataPagamento: string;
  metodo: MetodoPagamento;
  dataInizio: string;
  dataScadenza: string;
  note: string | null;
  rinnovoDiId: number | null;
}

/** Valori proposti dal programma per un nuovo pagamento, tutti modificabili. */
export interface PropostaPagamento {
  importoCent: number;
  dataInizio: string;
  dataScadenza: string;
  rinnovoDiId: number | null;
}

export interface FiltroPagamenti {
  dal: string | null;
  al: string | null;
  metodo: MetodoPagamento | null;
  iscrittoId: number | null;
}

/** Una pagina di risultati: al massimo `elementiPerPagina` elementi. */
export interface Pagina<T> {
  elementi: T[];
  numero: number;
  elementiPerPagina: number;
  /** Elementi totali che corrispondono al filtro, su tutte le pagine. */
  totaleElementi: number;
}

export interface PaginaPagamenti extends Pagina<Pagamento> {
  /** Totale incassato di tutti i pagamenti filtrati, non solo della pagina. */
  totaleImportoCent: number;
}
