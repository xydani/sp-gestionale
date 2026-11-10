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

/** Stato di validità di iscrizione o abbonamento. */
export type StatoValidita = "attivo" | "in_scadenza" | "scaduto" | "nessuno";

export interface Iscritto {
  id: number;
  nome: string;
  cognome: string;
  /** Date ISO YYYY-MM-DD. Assenti solo per iscritti inseriti prima che fossero obbligatorie. */
  dataNascita: string | null;
  luogoNascita: string | null;
  telefono: string | null;
  email: string | null;
  note: string | null;
  archiviato: boolean;
  ultimaDataFineIscrizione: string | null;
  statoIscrizione: StatoValidita;
  ultimaDataScadenzaAbbonamento: string | null;
  statoAbbonamento: StatoValidita;
  /** Assente se il certificato medico non è stato presentato. */
  dataScadenzaCertificato: string | null;
  statoCertificato: StatoValidita;
}

/** Dati inseriti dall'utente per creare o modificare un iscritto. */
export interface DatiIscritto {
  nome: string;
  cognome: string;
  dataNascita: string;
  luogoNascita: string;
  telefono: string | null;
  email: string | null;
  note: string | null;
  /** Scadenza del certificato medico; null se non è stato presentato. */
  dataScadenzaCertificato: string | null;
}

export type OrdinamentoIscritti =
  | "alfabetico"
  | "scadenza_abbonamento_crescente"
  | "scadenza_abbonamento_decrescente"
  | "fine_iscrizione_crescente"
  | "fine_iscrizione_decrescente"
  | "scadenza_certificato_crescente"
  | "scadenza_certificato_decrescente";

export interface FiltroIscritti {
  testo: string;
  includiArchiviati: boolean;
  statoAbbonamento: StatoValidita | null;
  statoIscrizione: StatoValidita | null;
  statoCertificato?: StatoValidita | null;
  /** Se assente, ordine alfabetico. */
  ordinamento?: OrdinamentoIscritti;
}

/** Iscrizione annuale (anno sportivo) con la quota pagata. */
export interface Iscrizione {
  id: number;
  iscrittoId: number;
  importoCent: number;
  dataPagamento: string;
  metodo: MetodoPagamento;
  dataInizio: string;
  dataFine: string;
  note: string | null;
}

export type DatiIscrizione = Omit<Iscrizione, "id">;

/** Quota e periodo proposti per una nuova iscrizione, tutti modificabili. */
export interface PropostaIscrizione {
  importoCent: number;
  dataInizio: string;
  dataFine: string;
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
  iscrittoEliminato: boolean;
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

/** Iscritti (non archiviati) con abbonamento o iscrizione in scadenza. */
export interface ConteggioInScadenza {
  abbonamenti: number;
  iscrizioni: number;
  certificati: number;
}

export interface EsitoPromemoria {
  /** Scadenze notificate ora per la prima volta. */
  nuovi: ConteggioInScadenza;
  inScadenza: ConteggioInScadenza;
}

export interface ConteggioStati {
  attivi: number;
  inScadenza: number;
  scaduti: number;
  assenti: number;
}

export interface Incassi {
  abbonamentiCent: number;
  iscrizioniCent: number;
  totaleCent: number;
}

export interface ProssimaScadenza {
  tipo: TipoScadenza;
  iscrittoId: number;
  nome: string;
  cognome: string;
  dataScadenza: string;
}

export interface RiepilogoDashboard {
  /** Iscritti non archiviati. */
  iscritti: number;
  abbonamenti: ConteggioStati;
  iscrizioni: ConteggioStati;
  /** "assenti" sono gli iscritti che non hanno presentato il certificato medico. */
  certificati: ConteggioStati;
  incassiMese: Incassi;
  incassiMesePrecedente: Incassi;
  prossimeScadenze: ProssimaScadenza[];
}

export type Raggruppamento = "giorno" | "mese" | "anno";

export interface IncassiPeriodo {
  /** "AAAA", "AAAA-MM" o "AAAA-MM-GG" secondo il raggruppamento. */
  periodo: string;
  incassi: Incassi;
}

export interface IncassiMetodo {
  metodo: MetodoPagamento;
  incassi: Incassi;
}

export interface RiepilogoIncassi {
  totale: Incassi;
  numeroMovimenti: number;
  perPeriodo: IncassiPeriodo[];
  perMetodo: IncassiMetodo[];
}

export interface Impostazioni {
  nomePalestra: string;
  giorniPreavviso: number;
  quotaIscrizioneCent: number;
  meseInizioAnnoSportivo: number;
  backupConservati: number;
  /** 0 = il programma non si blocca da solo. */
  minutiBloccoAutomatico: number;
}

/** Iscrizione completa di una persona nuova. `iscrittoId` di iscrizione e pagamento è ignorato. */
export interface DatiNuovoIscritto {
  anagrafica: DatiIscritto;
  iscrizione: DatiIscrizione;
  pagamento: DatiPagamento;
}

/** Cosa scade: abbonamento, iscrizione annuale o certificato medico. */
export type TipoScadenza = "abbonamento" | "iscrizione" | "certificato";
