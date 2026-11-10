import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart";

/** Se il programma parte da solo all'accesso al computer. */
export function avvioAutomaticoAttivo(): Promise<boolean> {
  return isEnabled();
}

export async function impostaAvvioAutomatico(attivo: boolean): Promise<void> {
  if (attivo) await enable();
  else await disable();
}
