import { formattaEuro, nomeMeseDaNumero } from "./format";
import type { TipologiaAbbonamento } from "./types";

/** Quanto costa al mese un abbonamento di più mesi; null se non ha senso dirlo. */
export function prezzoAlMeseCent(
  tipologia: Pick<TipologiaAbbonamento, "prezzoCent" | "durataValore" | "durataUnita">,
): number | null {
  if (tipologia.durataUnita !== "mesi" || tipologia.durataValore <= 1) return null;
  return Math.round(tipologia.prezzoCent / tipologia.durataValore);
}

/** «45,00 €/mese», oppure null per gli abbonamenti di un mese o a giorni. */
export function descriviPrezzoAlMese(
  tipologia: Pick<TipologiaAbbonamento, "prezzoCent" | "durataValore" | "durataUnita">,
): string | null {
  const alMese = prezzoAlMeseCent(tipologia);
  return alMese === null ? null : `${formattaEuro(alMese)}/mese`;
}

/** Le attive si vendono; le disattivate restano solo per lo storico. */
export function dividiTipologie(tipologie: TipologiaAbbonamento[]): {
  attive: TipologiaAbbonamento[];
  disattivate: TipologiaAbbonamento[];
} {
  return {
    attive: tipologie.filter((tipologia) => tipologia.attiva),
    disattivate: tipologie.filter((tipologia) => !tipologia.attiva),
  };
}

/** «da settembre a fine agosto»: il periodo coperto dalla quota annuale. */
export function descriviAnnoSportivo(meseInizio: number): string {
  const meseFine = meseInizio === 1 ? 12 : meseInizio - 1;
  return `da ${nomeMeseDaNumero(meseInizio)} a fine ${nomeMeseDaNumero(meseFine)}`;
}
