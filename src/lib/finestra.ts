import { LogicalSize, currentMonitor, getCurrentWindow } from "@tauri-apps/api/window";
import type { StatoDatabase } from "./types";

/** Dimensioni della finestra compatta, senza bordi, usata prima dello sblocco. */
export const DIMENSIONE_SBLOCCO = { larghezza: 400, altezza: 520 };
export const DIMENSIONE_PRIMO_AVVIO = { larghezza: 440, altezza: 680 };
/** Dimensione minima della finestra principale dopo lo sblocco. */
export const DIMENSIONE_MINIMA_PRINCIPALE = { larghezza: 900, altezza: 600 };
/**
 * Su Linux il gestore delle finestre applica il cambio "ridimensionabile" in modo asincrono:
 * una massimizzazione immediata verrebbe ignorata.
 */
const ATTESA_PRIMA_DI_MASSIMIZZARE_MS = 100;

export type ModalitaFinestra = "sblocco" | "primo_avvio" | "principale";

export function modalitaFinestraPerStato(stato: StatoDatabase): ModalitaFinestra {
  if (stato === "sbloccato") return "principale";
  if (stato === "da_creare") return "primo_avvio";
  return "sblocco";
}

/** Adatta la finestra alla schermata: compatta e senza bordi prima dello sblocco, massimizzata dopo. */
export async function adattaFinestra(modalita: ModalitaFinestra): Promise<void> {
  if (modalita === "principale") {
    await apriFinestraPrincipale();
  } else {
    await riduciAFinestraCompatta(
      modalita === "primo_avvio" ? DIMENSIONE_PRIMO_AVVIO : DIMENSIONE_SBLOCCO,
    );
  }
}

export function chiudiApp(): Promise<void> {
  return getCurrentWindow().close();
}

async function apriFinestraPrincipale(): Promise<void> {
  const finestra = getCurrentWindow();
  const minima = DIMENSIONE_MINIMA_PRINCIPALE;
  await finestra.setResizable(true);
  await finestra.setDecorations(true);
  await finestra.setShadow(true);
  await finestra.setMinSize(new LogicalSize(minima.larghezza, minima.altezza));
  await attendi(ATTESA_PRIMA_DI_MASSIMIZZARE_MS);
  await finestra.maximize();

  // Se il sistema ha ignorato la richiesta, la finestra occupa comunque l'area utile dello schermo
  if (!(await finestra.isMaximized())) {
    const schermo = await currentMonitor();
    if (schermo) {
      await finestra.setSize(schermo.workArea.size);
      await finestra.setPosition(schermo.workArea.position);
    }
  }
}

async function riduciAFinestraCompatta(dimensione: {
  larghezza: number;
  altezza: number;
}): Promise<void> {
  const finestra = getCurrentWindow();
  await finestra.setFullscreen(false);
  await finestra.unmaximize();
  await finestra.setMinSize(null);
  await finestra.setDecorations(false);
  // Senza ombra di sistema gli angoli arrotondati della schermata restano puliti
  await finestra.setShadow(false);
  await finestra.setResizable(false);
  await finestra.setSize(new LogicalSize(dimensione.larghezza, dimensione.altezza));
  await finestra.center();
}

function attendi(millisecondi: number): Promise<void> {
  return new Promise((risolvi) => setTimeout(risolvi, millisecondi));
}
