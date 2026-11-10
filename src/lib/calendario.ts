import { oggiIso } from "./format";

export interface MeseCalendario {
  anno: number;
  /** 1 = gennaio. */
  mese: number;
}

export interface GiornoCalendario {
  iso: string;
  numero: number;
  /** Falso per i giorni del mese prima o dopo, mostrati per completare le settimane. */
  delMese: boolean;
}

/** Settimana all'italiana: si parte dal lunedì. */
export const GIORNI_SETTIMANA = ["L", "M", "M", "G", "V", "S", "D"] as const;
const GIORNI_GRIGLIA = 42;

/** Sei settimane complete che contengono il mese, dal lunedì. */
export function grigliaMese({ anno, mese }: MeseCalendario): GiornoCalendario[] {
  const primo = new Date(anno, mese - 1, 1);
  const giorniPrima = (primo.getDay() + 6) % 7;
  return Array.from({ length: GIORNI_GRIGLIA }, (_, indice) => {
    const giorno = new Date(anno, mese - 1, 1 - giorniPrima + indice);
    return {
      iso: oggiIso(giorno),
      numero: giorno.getDate(),
      delMese: giorno.getMonth() === mese - 1,
    };
  });
}

export function spostaMese({ anno, mese }: MeseCalendario, delta: number): MeseCalendario {
  const data = new Date(anno, mese - 1 + delta, 1);
  return { anno: data.getFullYear(), mese: data.getMonth() + 1 };
}

/** Il mese da mostrare all'apertura: quello della data scelta, altrimenti quello di oggi. */
export function meseIniziale(dataIso: string, oggi: string = oggiIso()): MeseCalendario {
  const [anno, mese] = (dataIso || oggi).split("-").map(Number);
  return { anno, mese };
}

/** Vero se la data è fuori dall'intervallo consentito (estremi inclusi). */
export function fuoriIntervallo(
  dataIso: string,
  minimaIso: string | null,
  massimaIso: string | null,
): boolean {
  return (
    (minimaIso !== null && dataIso < minimaIso) || (massimaIso !== null && dataIso > massimaIso)
  );
}
