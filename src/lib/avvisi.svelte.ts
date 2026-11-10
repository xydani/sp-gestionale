/** Avvisi globali mostrati in cima all'app (es. backup automatico non riuscito). */

export type TipoAvviso = "successo" | "avviso" | "errore";

export interface Avviso {
  id: number;
  tipo: TipoAvviso;
  testo: string;
}

let prossimoId = 1;

export const avvisi = $state<Avviso[]>([]);

export function mostraAvviso(tipo: TipoAvviso, testo: string): void {
  avvisi.push({ id: prossimoId++, tipo, testo });
}

export function chiudiAvviso(id: number): void {
  const indice = avvisi.findIndex((avviso) => avviso.id === id);
  if (indice >= 0) avvisi.splice(indice, 1);
}
