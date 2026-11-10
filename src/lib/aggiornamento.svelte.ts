import { relaunch } from "@tauri-apps/plugin-process";
import { check, type DownloadEvent, type Update } from "@tauri-apps/plugin-updater";
import { eseguiBackupPreAggiornamento, messaggioErrore } from "./api";

/** Ogni quanto ricontrollare se c'è una nuova versione mentre il programma è aperto. */
export const INTERVALLO_CONTROLLO_AGGIORNAMENTI_MS = 24 * 60 * 60 * 1000;

export type FaseAggiornamento =
  | "inattivo"
  | "controllo"
  | "nessuno"
  | "disponibile"
  | "backup"
  | "download"
  | "installazione"
  | "errore";

export interface StatoAggiornamento {
  fase: FaseAggiornamento;
  versione: string | null;
  note: string | null;
  scaricatiByte: number;
  totaleByte: number | null;
  errore: string | null;
  /** "Più tardi": il banner non si mostra fino al prossimo controllo automatico. */
  rimandato: boolean;
}

export const aggiornamento = $state<StatoAggiornamento>({
  fase: "inattivo",
  versione: null,
  note: null,
  scaricatiByte: 0,
  totaleByte: null,
  errore: null,
  rimandato: false,
});

let aggiornamentoTrovato: Update | null = null;

/** Applica un evento di download allo stato: avanzamento in byte. */
export function applicaEventoDownload(stato: StatoAggiornamento, evento: DownloadEvent): void {
  switch (evento.event) {
    case "Started":
      stato.scaricatiByte = 0;
      stato.totaleByte = evento.data.contentLength ?? null;
      break;
    case "Progress":
      stato.scaricatiByte += evento.data.chunkLength;
      break;
    case "Finished":
      stato.fase = "installazione";
      break;
  }
}

/** Percentuale scaricata (0-100), oppure null se la dimensione non è nota. */
export function percentualeScaricata(stato: StatoAggiornamento): number | null {
  if (!stato.totaleByte) return null;
  return Math.min(100, Math.round((stato.scaricatiByte / stato.totaleByte) * 100));
}

/** Chiede a GitHub se c'è una versione nuova. Gli errori restano nello stato, non interrompono. */
export async function verificaAggiornamenti(automatico = false): Promise<void> {
  if (["backup", "download", "installazione"].includes(aggiornamento.fase)) return;
  aggiornamento.fase = "controllo";
  aggiornamento.errore = null;
  try {
    await aggiornamentoTrovato?.close();
    aggiornamentoTrovato = await check();
    if (aggiornamentoTrovato) {
      aggiornamento.fase = "disponibile";
      aggiornamento.versione = aggiornamentoTrovato.version;
      aggiornamento.note = aggiornamentoTrovato.body ?? null;
      if (automatico) aggiornamento.rimandato = false;
    } else {
      aggiornamento.fase = "nessuno";
    }
  } catch (errore) {
    aggiornamento.fase = "errore";
    aggiornamento.errore = `Impossibile verificare gli aggiornamenti: ${messaggioErrore(errore)}`;
  }
}

/** Backup, download con avanzamento, verifica della firma, installazione e riavvio. */
export async function installaAggiornamento(): Promise<void> {
  if (!aggiornamentoTrovato) return;
  try {
    aggiornamento.fase = "backup";
    await eseguiBackupPreAggiornamento();
    aggiornamento.fase = "download";
    await aggiornamentoTrovato.downloadAndInstall((evento) =>
      applicaEventoDownload(aggiornamento, evento),
    );
    await relaunch();
  } catch (errore) {
    aggiornamento.fase = "errore";
    aggiornamento.errore = `Aggiornamento non riuscito, il programma attuale resta in uso: ${messaggioErrore(errore)}`;
  }
}

export function rimandaAggiornamento(): void {
  aggiornamento.rimandato = true;
}
