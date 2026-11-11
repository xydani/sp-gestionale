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
export const scadenzeInArrivo = $state<ConteggioInScadenza>({
  abbonamenti: 0,
  iscrizioni: 0,
  certificati: 0,
});

/** Totale delle scadenze in arrivo, per il badge del menu. */
export function totaleInScadenza(conteggio: ConteggioInScadenza): number {
  return conteggio.abbonamenti + conteggio.iscrizioni + conteggio.certificati;
}

/** "a", "a e b", "a, b e c". */
/** "1 abbonamento", "3 abbonamenti"; null se zero, così non compare nel testo. */
function descriviQuantita(numero: number, singolare: string, plurale: string): string | null {
  if (numero === 0) return null;
  const nome = numero === 1 ? singolare : plurale;
  return `${numero} ${nome}`;
}

function unisciElenco(parti: string[]): string {
  if (parti.length <= 1) return parti.join("");
  return `${parti.slice(0, -1).join(", ")} e ${parti[parti.length - 1]}`;
}

/** Testo della notifica, oppure null se non c'è nulla di nuovo. */
export function testoNotifica(nuovi: ConteggioInScadenza, giorniPreavviso?: number): string | null {
  const parti = [
    descriviQuantita(nuovi.abbonamenti, "abbonamento", "abbonamenti"),
    descriviQuantita(nuovi.iscrizioni, "iscrizione", "iscrizioni"),
    descriviQuantita(nuovi.certificati, "certificato medico", "certificati medici"),
  ].filter((parte) => parte !== null);
  if (parti.length === 0) return null;
  const periodo = giorniPreavviso ? ` nei prossimi ${giorniPreavviso} giorni` : "";
  return `In scadenza${periodo}: ${unisciElenco(parti)}. Apri "Scadenze" per i dettagli.`;
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
