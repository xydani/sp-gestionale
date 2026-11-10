import { getCurrentWindow } from "@tauri-apps/api/window";

/** Passa da finestra a schermo intero e viceversa; restituisce il nuovo stato. */
export async function alternaSchermoIntero(): Promise<boolean> {
  const finestra = getCurrentWindow();
  const nuovoStato = !(await finestra.isFullscreen());
  await finestra.setFullscreen(nuovoStato);
  return nuovoStato;
}

export function leggiSchermoIntero(): Promise<boolean> {
  return getCurrentWindow().isFullscreen();
}

export async function esciDaSchermoIntero(): Promise<void> {
  const finestra = getCurrentWindow();
  if (await finestra.isFullscreen()) {
    await finestra.setFullscreen(false);
  }
}

/** Vero se il tasto premuto deve alternare lo schermo intero (F11). */
export function eTastoSchermoIntero(evento: Pick<KeyboardEvent, "key">): boolean {
  return evento.key === "F11";
}
