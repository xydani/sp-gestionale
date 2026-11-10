export interface OpzioneTendina<T> {
  valore: T;
  etichetta: string;
  /** Testo secondario a destra, es. il prezzo. */
  dettaglio?: string;
  disabilitata?: boolean;
}

/** Prossima opzione attivabile nella direzione indicata; resta ferma ai bordi. */
export function indiceSuccessivo<T>(
  opzioni: OpzioneTendina<T>[],
  attuale: number,
  direzione: 1 | -1,
): number {
  for (
    let indice = attuale + direzione;
    indice >= 0 && indice < opzioni.length;
    indice += direzione
  ) {
    if (!opzioni[indice].disabilitata) return indice;
  }
  return attuale;
}

/** Prima o ultima opzione attivabile (tasti Home e Fine). */
export function indiceEstremo<T>(opzioni: OpzioneTendina<T>[], ultimo: boolean): number {
  const partenza = ultimo ? opzioni.length : -1;
  return indiceSuccessivo(opzioni, partenza, ultimo ? -1 : 1);
}

/**
 * Prossima opzione la cui etichetta inizia con la lettera, dopo quella attuale e ricominciando
 * dall'inizio: premendo più volte la stessa lettera si passa tra le voci che la condividono.
 */
export function indicePerLettera<T>(
  opzioni: OpzioneTendina<T>[],
  lettera: string,
  attuale: number,
): number {
  const cercata = lettera.toLocaleLowerCase("it");
  for (let passo = 1; passo <= opzioni.length; passo++) {
    const indice = (attuale + passo) % opzioni.length;
    const opzione = opzioni[indice];
    if (!opzione.disabilitata && opzione.etichetta.toLocaleLowerCase("it").startsWith(cercata)) {
      return indice;
    }
  }
  return attuale;
}
