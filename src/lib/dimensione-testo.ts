/**
 * Dimensione del testo della finestra principale: è una preferenza di questo computer,
 * quindi sta nel browser integrato e non nell'archivio criptato.
 */

export type DimensioneTesto = "normale" | "grande" | "molto_grande";

export interface SceltaDimensioneTesto {
  valore: DimensioneTesto;
  etichetta: string;
  /** Dimensione di base: tutto ciò che è in rem si ingrandisce in proporzione. */
  pixel: number;
}

export const SCELTE_DIMENSIONE_TESTO: SceltaDimensioneTesto[] = [
  { valore: "normale", etichetta: "Normale", pixel: 16 },
  { valore: "grande", etichetta: "Grande", pixel: 18 },
  { valore: "molto_grande", etichetta: "Molto grande", pixel: 20 },
];

const CHIAVE_ARCHIVIO = "dimensione-testo";
const DIMENSIONE_PREDEFINITA: DimensioneTesto = "normale";

function eDimensioneTesto(valore: string | null): valore is DimensioneTesto {
  return SCELTE_DIMENSIONE_TESTO.some((scelta) => scelta.valore === valore);
}

export function leggiDimensioneTesto(archivio: Storage = localStorage): DimensioneTesto {
  try {
    const salvata = archivio.getItem(CHIAVE_ARCHIVIO);
    return eDimensioneTesto(salvata) ? salvata : DIMENSIONE_PREDEFINITA;
  } catch {
    // Archivio non disponibile: si usa la dimensione normale
    return DIMENSIONE_PREDEFINITA;
  }
}

export function salvaDimensioneTesto(
  dimensione: DimensioneTesto,
  archivio: Storage = localStorage,
): void {
  try {
    archivio.setItem(CHIAVE_ARCHIVIO, dimensione);
  } catch {
    // Se non si può salvare la scelta vale solo fino alla chiusura
  }
}

export function pixelDimensioneTesto(dimensione: DimensioneTesto): number {
  const scelta = SCELTE_DIMENSIONE_TESTO.find((scelta) => scelta.valore === dimensione);
  return scelta?.pixel ?? SCELTE_DIMENSIONE_TESTO[0].pixel;
}

export function applicaDimensioneTesto(
  dimensione: DimensioneTesto,
  radice: HTMLElement = document.documentElement,
): void {
  radice.style.fontSize = `${pixelDimensioneTesto(dimensione)}px`;
}
