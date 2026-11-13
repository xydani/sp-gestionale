import type {
  ConteggioInScadenza,
  FiltroIscritti,
  Iscritto,
  OrdinamentoIscritti,
  RiepilogoDashboard,
  TipoScadenza,
} from "./types";

/** Stati mostrati nella pagina Scadenze: "nessuno" si usa solo per i certificati mancanti. */
export type StatoScadenza = "in_scadenza" | "scaduto" | "nessuno";

interface RegoleScadenza {
  etichetta: string;
  chiaveConteggio: keyof ConteggioInScadenza;
  ordinamentoCrescente: OrdinamentoIscritti;
  ordinamentoDecrescente: OrdinamentoIscritti;
  dataScadenza: (iscritto: Iscritto) => string | null;
}

export const REGOLE_SCADENZA: Record<TipoScadenza, RegoleScadenza> = {
  abbonamento: {
    etichetta: "Abbonamenti",
    chiaveConteggio: "abbonamenti",
    ordinamentoCrescente: "scadenza_abbonamento_crescente",
    ordinamentoDecrescente: "scadenza_abbonamento_decrescente",
    dataScadenza: (iscritto) => iscritto.ultimaDataScadenzaAbbonamento,
  },
  iscrizione: {
    etichetta: "Iscrizioni annuali",
    chiaveConteggio: "iscrizioni",
    ordinamentoCrescente: "fine_iscrizione_crescente",
    ordinamentoDecrescente: "fine_iscrizione_decrescente",
    dataScadenza: (iscritto) => iscritto.ultimaDataFineIscrizione,
  },
  certificato: {
    etichetta: "Certificati medici",
    chiaveConteggio: "certificati",
    ordinamentoCrescente: "scadenza_certificato_crescente",
    ordinamentoDecrescente: "scadenza_certificato_decrescente",
    dataScadenza: (iscritto) => iscritto.dataScadenzaCertificato,
  },
};

export const TIPI_SCADENZA = Object.keys(REGOLE_SCADENZA) as TipoScadenza[];

/** Sezioni della pagina per ogni tipo: per i certificati anche quelli mai presentati. */
export function statiDaMostrare(tipo: TipoScadenza): StatoScadenza[] {
  return tipo === "certificato"
    ? ["in_scadenza", "scaduto", "nessuno"]
    : ["in_scadenza", "scaduto"];
}

/**
 * Filtro dell'elenco: in scadenza le più vicine prima, scaduti i più recenti prima,
 * mancanti in ordine alfabetico.
 */
export function filtroScadenze(tipo: TipoScadenza, stato: StatoScadenza): FiltroIscritti {
  const regole = REGOLE_SCADENZA[tipo];
  const ordinamento: Record<StatoScadenza, OrdinamentoIscritti> = {
    in_scadenza: regole.ordinamentoCrescente,
    scaduto: regole.ordinamentoDecrescente,
    nessuno: "alfabetico",
  };
  return {
    testo: "",
    includiArchiviati: false,
    statoAbbonamento: tipo === "abbonamento" ? stato : null,
    statoIscrizione: tipo === "iscrizione" ? stato : null,
    statoCertificato: tipo === "certificato" ? stato : null,
    ordinamento: ordinamento[stato],
  };
}

export interface GruppoScaduti {
  tipo: TipoScadenza;
  numero: number;
}

/** Tipi con almeno uno scaduto, in ordine fisso: in dashboard si mostrano solo questi. */
export function gruppiConScaduti(riepilogo: RiepilogoDashboard): GruppoScaduti[] {
  const numeri: Record<TipoScadenza, number> = {
    abbonamento: riepilogo.abbonamenti.scaduti,
    iscrizione: riepilogo.iscrizioni.scaduti,
    certificato: riepilogo.certificati.scaduti,
  };
  return TIPI_SCADENZA.map((tipo) => ({ tipo, numero: numeri[tipo] })).filter(
    (gruppo) => gruppo.numero > 0,
  );
}

/** Il gruppo scelto se ha ancora scaduti, altrimenti il primo che ne ha (o nessuno). */
export function gruppoDaMostrare(
  gruppi: GruppoScaduti[],
  scelto: TipoScadenza | null,
): TipoScadenza | null {
  if (gruppi.some((gruppo) => gruppo.tipo === scelto)) return scelto;
  return gruppi[0]?.tipo ?? null;
}
