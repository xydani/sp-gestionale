/** Avvisi globali mostrati in basso a destra (conferme, problemi, errori). */

export type TipoAvviso = "successo" | "avviso" | "errore";

/** Pulsante dentro l'avviso, per esempio «Annulla» dopo un'eliminazione. */
export interface AzioneAvviso {
  etichetta: string;
  esegui: () => void;
}

export interface Avviso {
  id: number;
  tipo: TipoAvviso;
  testo: string;
  azione?: AzioneAvviso;
}

export interface OpzioniAvviso {
  azione?: AzioneAvviso;
  /** Se assente, la durata predefinita del tipo. */
  durataMs?: number;
}

/** Le conferme si leggono al volo; avvisi ed errori restano di più per dare il tempo di leggerli. */
export const DURATA_AVVISO_MS: Record<TipoAvviso, number> = {
  successo: 4_000,
  avviso: 8_000,
  errore: 10_000,
};

let prossimoId = 1;

export const avvisi = $state<Avviso[]>([]);

/** Restituisce l'id dell'avviso, per chiuderlo prima del tempo. */
export function mostraAvviso(tipo: TipoAvviso, testo: string, opzioni: OpzioniAvviso = {}): number {
  const id = prossimoId++;
  avvisi.push({ id, tipo, testo, azione: opzioni.azione });
  setTimeout(() => chiudiAvviso(id), opzioni.durataMs ?? DURATA_AVVISO_MS[tipo]);
  return id;
}

export function chiudiAvviso(id: number): void {
  const indice = avvisi.findIndex((avviso) => avviso.id === id);
  if (indice >= 0) avvisi.splice(indice, 1);
}
