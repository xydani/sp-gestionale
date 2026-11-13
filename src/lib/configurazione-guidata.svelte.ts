import { configurazioneInizialeCompletata } from "./api";

/** Passi della configurazione guidata di un nuovo archivio, in ordine. */
export const PASSI_CONFIGURAZIONE = [
  { titolo: "Opzioni della palestra" },
  { titolo: "Listino prezzi" },
] as const;

/** "Passo 1 di 2 · Opzioni della palestra" (indice da 0). */
export function descriviPasso(indice: number): string {
  const passo = PASSI_CONFIGURAZIONE[indice];
  return `Passo ${indice + 1} di ${PASSI_CONFIGURAZIONE.length} · ${passo.titolo}`;
}

export function eUltimoPasso(indice: number): boolean {
  return indice === PASSI_CONFIGURAZIONE.length - 1;
}

export const PERCORSO_CONFIGURAZIONE = "/benvenuto" as const;

/**
 * Stato condiviso: finché la configurazione non è completata il resto del programma è bloccato.
 * `null` finché non si è letto dall'archivio.
 */
export const configurazione = $state<{ completata: boolean | null }>({ completata: null });

export async function verificaConfigurazione(): Promise<void> {
  configurazione.completata = await configurazioneInizialeCompletata();
}

/**
 * Dove mandare l'utente, o null se può restare dove si trova: senza configurazione completata
 * si può stare solo nella guida, e a guida finita la guida non serve più.
 */
export function percorsoObbligato(
  completata: boolean,
  percorso: string,
): typeof PERCORSO_CONFIGURAZIONE | "/" | null {
  const nellaGuida = percorso === PERCORSO_CONFIGURAZIONE;
  if (!completata && !nellaGuida) return PERCORSO_CONFIGURAZIONE;
  if (completata && nellaGuida) return "/";
  return null;
}
