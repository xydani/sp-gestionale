import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";
import { contaInScadenza, controllaPromemoria } from "./api";
import type { ConteggioInScadenza } from "./types";

/** Ogni quanto ricontrollare le scadenze mentre il programma è aperto. */
export const INTERVALLO_CONTROLLO_MS = 60 * 60 * 1000;

/** Conteggio mostrato nel badge del menu, condiviso tra i componenti. */
export const scadenzeInArrivo = $state<ConteggioInScadenza>({ abbonamenti: 0, iscrizioni: 0 });

/** Testo della notifica, oppure null se non c'è nulla di nuovo. */
export function testoNotifica(nuovi: ConteggioInScadenza, giorniPreavviso?: number): string | null {
  const parti: string[] = [];
  if (nuovi.abbonamenti > 0) {
    parti.push(`${nuovi.abbonamenti} ${nuovi.abbonamenti === 1 ? "abbonamento" : "abbonamenti"}`);
  }
  if (nuovi.iscrizioni > 0) {
    parti.push(`${nuovi.iscrizioni} ${nuovi.iscrizioni === 1 ? "iscrizione" : "iscrizioni"}`);
  }
  if (parti.length === 0) return null;
  const periodo = giorniPreavviso ? ` nei prossimi ${giorniPreavviso} giorni` : "";
  return `In scadenza${periodo}: ${parti.join(" e ")}. Apri "Scadenze" per i dettagli.`;
}

/** Controlla le scadenze, invia una notifica riassuntiva se ce ne sono di nuove e aggiorna il badge. */
export async function eseguiControlloPromemoria(): Promise<void> {
  const esito = await controllaPromemoria();
  Object.assign(scadenzeInArrivo, esito.inScadenza);
  const testo = testoNotifica(esito.nuovi);
  if (testo && (await permessoNotifiche())) {
    sendNotification({ title: "SP Gestionale – Scadenze", body: testo });
  }
}

/** Aggiorna solo il badge (es. dopo un rinnovo), senza notifiche. */
export async function aggiornaConteggioScadenze(): Promise<void> {
  Object.assign(scadenzeInArrivo, await contaInScadenza());
}

async function permessoNotifiche(): Promise<boolean> {
  if (await isPermissionGranted()) return true;
  return (await requestPermission()) === "granted";
}
