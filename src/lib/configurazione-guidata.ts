/** Passi della configurazione guidata di un nuovo archivio, in ordine. */
export const PASSI_CONFIGURAZIONE = [
  { titolo: "Opzioni della palestra" },
  { titolo: "Listino abbonamenti" },
] as const;

/** "Passo 1 di 2 · Opzioni della palestra" (indice da 0). */
export function descriviPasso(indice: number): string {
  const passo = PASSI_CONFIGURAZIONE[indice];
  return `Passo ${indice + 1} di ${PASSI_CONFIGURAZIONE.length} · ${passo.titolo}`;
}

export function eUltimoPasso(indice: number): boolean {
  return indice === PASSI_CONFIGURAZIONE.length - 1;
}
