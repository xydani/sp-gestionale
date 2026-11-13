/**
 * Calendari, tendine e risultati di ricerca si aprono sotto il loro campo con `position: fixed`:
 * così nessun contenitore che scorre (come una finestra sovrapposta) li taglia.
 */

/** Distanza tra il campo e il riquadro che si apre. */
export const DISTANZA_DAL_CAMPO_PX = 6;
/** Spazio minimo da lasciare tra il riquadro e il bordo della finestra. */
export const MARGINE_FINESTRA_PX = 8;

export interface Rettangolo {
  top: number;
  bottom: number;
  left: number;
}

export interface Dimensioni {
  larghezza: number;
  altezza: number;
}

/** Sotto il campo se ci sta (o se sopra c'è ancora meno spazio), altrimenti sopra. */
export function calcolaPosizioneSospesa(
  campo: Rettangolo,
  riquadro: Dimensioni,
  finestra: Dimensioni,
): { top: number; left: number } {
  const spazioSotto = finestra.altezza - campo.bottom - DISTANZA_DAL_CAMPO_PX;
  const spazioSopra = campo.top - DISTANZA_DAL_CAMPO_PX;
  const sotto = spazioSotto >= riquadro.altezza || spazioSotto >= spazioSopra;
  const top = sotto
    ? campo.bottom + DISTANZA_DAL_CAMPO_PX
    : campo.top - DISTANZA_DAL_CAMPO_PX - riquadro.altezza;
  const leftMassimo = finestra.larghezza - riquadro.larghezza - MARGINE_FINESTRA_PX;
  const left = Math.max(MARGINE_FINESTRA_PX, Math.min(campo.left, leftMassimo));
  return { top, left };
}

export interface OpzioniSospeso {
  /** Il campo sotto cui aprirsi. */
  ancora: HTMLElement | undefined;
  /** «minima»: almeno largo quanto il campo; «uguale»: largo esattamente come il campo. */
  larghezza?: "libera" | "minima" | "uguale";
}

/** Azione Svelte: tiene il riquadro attaccato al campo anche se la pagina o la finestra scorrono. */
export function sospesoSottoCampo(riquadro: HTMLElement, opzioni: OpzioniSospeso) {
  let attuali = opzioni;
  riquadro.style.position = "fixed";

  function posiziona() {
    const ancora = attuali.ancora;
    if (!ancora) return;
    const campo = ancora.getBoundingClientRect();
    if (attuali.larghezza === "minima") riquadro.style.minWidth = `${campo.width}px`;
    if (attuali.larghezza === "uguale") riquadro.style.width = `${campo.width}px`;
    const { top, left } = calcolaPosizioneSospesa(
      campo,
      { larghezza: riquadro.offsetWidth, altezza: riquadro.offsetHeight },
      { larghezza: window.innerWidth, altezza: window.innerHeight },
    );
    riquadro.style.top = `${top}px`;
    riquadro.style.left = `${left}px`;
  }

  posiziona();
  // Anche il riquadro può cambiare altezza (es. un mese del calendario con una settimana in più)
  const osservatore = new ResizeObserver(posiziona);
  osservatore.observe(riquadro);
  window.addEventListener("scroll", posiziona, true);
  window.addEventListener("resize", posiziona);

  return {
    update(nuove: OpzioniSospeso) {
      attuali = nuove;
      posiziona();
    },
    destroy() {
      osservatore.disconnect();
      window.removeEventListener("scroll", posiziona, true);
      window.removeEventListener("resize", posiziona);
    },
  };
}
