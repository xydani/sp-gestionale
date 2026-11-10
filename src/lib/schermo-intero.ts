import { getCurrentWindow } from "@tauri-apps/api/window";

/** Passa da finestra a schermo intero e viceversa. */
export async function alternaSchermoIntero(): Promise<void> {
  const finestra = getCurrentWindow();
  await finestra.setFullscreen(!(await finestra.isFullscreen()));
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
