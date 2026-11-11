import { nomeMese, oggiIso } from "./format";
import type { Periodo } from "./periodo";
import { aggiungiMesiIso } from "./validazione";
import type { ConteggioRinnovi, VenditeTipologiaMese } from "./types";

export type PeriodoStatistiche = "ultimi_12_mesi" | "anno_sportivo" | "anno_sportivo_precedente";

export const ETICHETTE_PERIODO_STATISTICHE: Record<PeriodoStatistiche, string> = {
  ultimi_12_mesi: "Ultimi 12 mesi",
  anno_sportivo: "Anno sportivo in corso",
  anno_sportivo_precedente: "Anno sportivo precedente",
};

/** Settembre, come nel programma, se le impostazioni non si leggono. */
export const MESE_INIZIO_ANNO_SPORTIVO_PREDEFINITO = 9;

/** Sotto questo numero di scadenze decise una percentuale direbbe poco o niente. */
export const SCADENZE_MINIME_PER_TASSO = 5;

export function calcolaPeriodoStatistiche(
  scelta: PeriodoStatistiche,
  meseInizioAnnoSportivo: number,
  oggi: string = oggiIso(),
): Periodo {
  if (scelta === "ultimi_12_mesi") {
    return { dal: giornoDopo(aggiungiMesiIso(oggi, -12)), al: oggi };
  }
  const inizioInCorso = inizioAnnoSportivo(oggi, meseInizioAnnoSportivo);
  const inizio = scelta === "anno_sportivo" ? inizioInCorso : aggiungiMesiIso(inizioInCorso, -12);
  return { dal: inizio, al: giornoPrima(aggiungiMesiIso(inizio, 12)) };
}

function inizioAnnoSportivo(oggi: string, meseInizio: number): string {
  const [anno, mese] = oggi.split("-").map(Number);
  const annoInizio = mese >= meseInizio ? anno : anno - 1;
  return `${annoInizio}-${String(meseInizio).padStart(2, "0")}-01`;
}

function giornoDopo(dataIso: string): string {
  const [anno, mese, giorno] = dataIso.split("-").map(Number);
  return oggiIso(new Date(anno, mese - 1, giorno + 1));
}

function giornoPrima(dataIso: string): string {
  const [anno, mese, giorno] = dataIso.split("-").map(Number);
  return oggiIso(new Date(anno, mese - 1, giorno - 1));
}

export function scadenzeDecise(conteggio: ConteggioRinnovi): number {
  return conteggio.rinnovati + conteggio.persi;
}

export function bastanoPerUnTasso(conteggio: ConteggioRinnovi): boolean {
  return scadenzeDecise(conteggio) >= SCADENZE_MINIME_PER_TASSO;
}

/** "88%", oppure "—" se il tasso non si può calcolare. */
export function formattaTasso(tasso: number | null): string {
  if (tasso === null) return "—";
  return `${Math.round(tasso * 100)}%`;
}

export interface TabellaVendite {
  tipologie: string[];
  righe: { mese: string; numeri: Record<string, number> }[];
}

/** Vendite mese per mese in forma di tabella: una colonna per tipologia. */
export function tabellaVenditePerMese(vendite: VenditeTipologiaMese[]): TabellaVendite {
  const tipologie = [...new Set(vendite.map((vendita) => vendita.tipologiaNome))].sort();
  const mesi = [...new Set(vendite.map((vendita) => vendita.mese))].sort();
  const righe = mesi.map((mese) => {
    const numeri: Record<string, number> = {};
    for (const tipologia of tipologie) numeri[tipologia] = 0;
    for (const vendita of vendite.filter((vendita) => vendita.mese === mese)) {
      numeri[vendita.tipologiaNome] = vendita.numero;
    }
    return { mese, numeri };
  });
  return { tipologie, righe };
}

/** Etichetta breve del mese sotto una colonna: "nov". */
export function meseBreve(chiaveMese: string): string {
  return nomeMese(`${chiaveMese}-01`).slice(0, 3);
}
