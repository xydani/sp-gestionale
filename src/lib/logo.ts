/** Formati che si possono scegliere come logo (gli stessi accettati dal backend). */
export const ESTENSIONI_LOGO = ["png", "jpg", "jpeg", "webp", "svg"] as const;

/**
 * Senza un logo scelto in Impostazioni si provano i file locali (non nel repository,
 * quindi assenti negli installer) e infine lo stemma, che c'è sempre.
 */
export const PERCORSI_LOGO_PREDEFINITI = [
  "/logo-palestra.svg",
  "/logo-palestra.png",
  "/stemma.svg",
] as const;

/** Il prossimo percorso da provare dopo un errore di caricamento, o null se finiti. */
export function prossimoPercorsoLogo(attuale: string): string | null {
  const indice = PERCORSI_LOGO_PREDEFINITI.findIndex((percorso) => attuale.endsWith(percorso));
  return PERCORSI_LOGO_PREDEFINITI[indice + 1] ?? null;
}
