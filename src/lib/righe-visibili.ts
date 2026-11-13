/**
 * Quante righe di una tabella stanno nella finestra senza scorrere: le pagine degli elenchi
 * si adattano a schermo, risoluzione e dimensione del testo.
 */

/** Anche su una finestra molto bassa un elenco mostra almeno qualche riga. */
export const RIGHE_MINIME = 3;
/** Il programma non restituisce pagine più lunghe di così. */
export const RIGHE_MASSIME = 50;
/** Altezza di una riga prima di poterla misurare (righe compatte con pulsante). */
export const ALTEZZA_RIGA_STIMATA_PX = 42;

export interface MisureTabella {
  /** Altezza visibile della finestra. */
  altezzaFinestra: number;
  /** Dove inizia la tabella, dall'alto della pagina. */
  inizioTabella: number;
  /** Intestazione ed eventuale riga dei totali: ci sono comunque, con qualsiasi numero di righe. */
  altezzaFissa: number;
  altezzaRiga: number;
  /** Spazio da lasciare sotto la tabella: pagine, collegamenti, margine della pagina. */
  spazioSotto: number;
}

export function calcolaRigheVisibili(misure: MisureTabella): number {
  const spazioPerLeRighe =
    misure.altezzaFinestra - misure.inizioTabella - misure.altezzaFissa - misure.spazioSotto;
  const righe = Math.floor(spazioPerLeRighe / Math.max(misure.altezzaRiga, 1));
  return Math.min(Math.max(righe, RIGHE_MINIME), RIGHE_MASSIME);
}

/** Cambiando le righe per pagina si resta sulla pagina che contiene la prima riga che si vedeva. */
export function paginaDopoCambioRighe(
  numeroPagina: number,
  righePrima: number,
  righeDopo: number,
): number {
  const primaRigaVista = (numeroPagina - 1) * righePrima;
  return Math.floor(primaRigaVista / righeDopo) + 1;
}

/** Misure di una tabella già disegnata; senza righe se ne usa una stimata. */
export function misuraTabella(tabella: HTMLTableElement, spazioSotto: number): MisureTabella {
  const intestazione = tabella.tHead?.getBoundingClientRect().height ?? 0;
  const piede = tabella.tFoot?.getBoundingClientRect().height ?? 0;
  const primaRiga = tabella.tBodies[0]?.rows[0];
  const altezzaRiga = primaRiga?.getBoundingClientRect().height || ALTEZZA_RIGA_STIMATA_PX;
  return {
    altezzaFinestra: window.innerHeight,
    // Dall'alto della pagina, anche se in quel momento è scorsa
    inizioTabella: tabella.getBoundingClientRect().top + window.scrollY,
    altezzaFissa: intestazione + piede,
    altezzaRiga,
    spazioSotto,
  };
}

/**
 * Segue una tabella e chiama `onCambia` con le righe che entrano, all'inizio e ogni volta che
 * cambiano finestra o dimensioni (es. testo più grande). Restituisce la funzione per smettere.
 */
export function osservaRigheVisibili(
  tabella: HTMLTableElement,
  spazioSotto: number,
  onCambia: (righe: number) => void,
): () => void {
  let ultime: number | null = null;
  const ricalcola = () => {
    const righe = calcolaRigheVisibili(misuraTabella(tabella, spazioSotto));
    if (righe === ultime) return;
    ultime = righe;
    onCambia(righe);
  };
  const osservatore = new ResizeObserver(ricalcola);
  osservatore.observe(tabella);
  osservatore.observe(document.documentElement);
  window.addEventListener("resize", ricalcola);
  ricalcola();
  return () => {
    osservatore.disconnect();
    window.removeEventListener("resize", ricalcola);
  };
}

/** Cosa serve a un elenco a pagine per seguire la finestra. */
export interface ElencoAPagine {
  /** La pagina che si sta guardando. */
  numeroPagina: () => number;
  /** Carica la pagina indicata con quel numero di righe. */
  carica: (numeroPagina: number, righePerPagina: number) => void;
}

/**
 * Collega la misura delle righe a un elenco a pagine: al primo calcolo carica la prima pagina,
 * ai successivi resta sulla pagina con la prima riga che si vedeva. Restituisce la funzione per
 * smettere di seguire la tabella.
 */
export function seguiFinestra(
  tabella: HTMLTableElement,
  spazioSotto: number,
  elenco: ElencoAPagine,
): () => void {
  let righePrima: number | null = null;
  return osservaRigheVisibili(tabella, spazioSotto, (righe) => {
    const numero =
      righePrima === null ? 1 : paginaDopoCambioRighe(elenco.numeroPagina(), righePrima, righe);
    righePrima = righe;
    elenco.carica(numero, righe);
  });
}
