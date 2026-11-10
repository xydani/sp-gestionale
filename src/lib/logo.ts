/** Il logo non è nel repository: si prova l'SVG, poi il PNG; se mancano entrambi non si mostra. */
export const PERCORSI_LOGO = ["/logo-palestra.svg", "/logo-palestra.png"] as const;

/** Il prossimo percorso da provare dopo un errore di caricamento, o null se finiti. */
export function prossimoPercorsoLogo(attuale: string): string | null {
  const indice = PERCORSI_LOGO.findIndex((percorso) => attuale.endsWith(percorso));
  return PERCORSI_LOGO[indice + 1] ?? null;
}
