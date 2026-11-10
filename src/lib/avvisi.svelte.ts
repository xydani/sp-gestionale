/** Avvisi globali mostrati in basso a destra (conferme, problemi, errori). */

export type TipoAvviso = "successo" | "avviso" | "errore";

export interface Avviso {
  id: number;
  tipo: TipoAvviso;
  testo: string;
}

/** Le conferme si leggono al volo; avvisi ed errori restano di più per dare il tempo di leggerli. */
export const DURATA_AVVISO_MS: Record<TipoAvviso, number> = {
  successo: 4_000,
  avviso: 8_000,
  errore: 10_000,
};

let prossimoId = 1;

export const avvisi = $state<Avviso[]>([]);

export function mostraAvviso(tipo: TipoAvviso, testo: string): void {
  const id = prossimoId++;
  avvisi.push({ id, tipo, testo });
  setTimeout(() => chiudiAvviso(id), DURATA_AVVISO_MS[tipo]);
}

export function chiudiAvviso(id: number): void {
  const indice = avvisi.findIndex((avviso) => avviso.id === id);
  if (indice >= 0) avvisi.splice(indice, 1);
}
