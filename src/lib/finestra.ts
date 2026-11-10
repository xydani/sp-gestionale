import { LogicalSize, getCurrentWindow } from "@tauri-apps/api/window";
import type { StatoDatabase } from "./types";

/** Dimensioni della finestra compatta, senza bordi, usata prima dello sblocco. */
export const DIMENSIONE_SBLOCCO = { larghezza: 420, altezza: 380 };
export const DIMENSIONE_PRIMO_AVVIO = { larghezza: 460, altezza: 600 };
/** Dimensione minima della finestra principale dopo lo sblocco. */
export const DIMENSIONE_MINIMA_PRINCIPALE = { larghezza: 900, altezza: 600 };

export type ModalitaFinestra = "sblocco" | "primo_avvio" | "principale";

export function modalitaFinestraPerStato(stato: StatoDatabase): ModalitaFinestra {
  if (stato === "sbloccato") return "principale";
  if (stato === "da_creare") return "primo_avvio";
  return "sblocco";
}

/** Adatta la finestra alla schermata: compatta e senza bordi prima dello sblocco, massimizzata dopo. */
export async function adattaFinestra(modalita: ModalitaFinestra): Promise<void> {
  const finestra = getCurrentWindow();

  if (modalita === "principale") {
    const minima = DIMENSIONE_MINIMA_PRINCIPALE;
    await finestra.setDecorations(true);
    await finestra.setResizable(true);
    await finestra.setMinSize(new LogicalSize(minima.larghezza, minima.altezza));
    await finestra.maximize();
    return;
  }

  const dimensione = modalita === "primo_avvio" ? DIMENSIONE_PRIMO_AVVIO : DIMENSIONE_SBLOCCO;
  await finestra.setFullscreen(false);
  await finestra.unmaximize();
  await finestra.setMinSize(null);
  await finestra.setDecorations(false);
  await finestra.setResizable(false);
  await finestra.setSize(new LogicalSize(dimensione.larghezza, dimensione.altezza));
  await finestra.center();
}

export function chiudiApp(): Promise<void> {
  return getCurrentWindow().close();
}
