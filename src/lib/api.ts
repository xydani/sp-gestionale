import { invoke } from "@tauri-apps/api/core";
import type {
  DatiIscritto,
  DatiTipologia,
  ErroreApp,
  EsitoSblocco,
  FiltroIscritti,
  Iscritto,
  StatoDatabase,
  TipologiaAbbonamento,
} from "./types";

/** Unico punto di accesso al backend: ogni comando Tauri ha qui il suo wrapper tipizzato. */

export function leggiStatoDatabase(): Promise<StatoDatabase> {
  return invoke<StatoDatabase>("stato_database");
}

export function creaDatabase(password: string, confermaPassword: string): Promise<void> {
  return invoke("crea_database", { password, confermaPassword });
}

export function sbloccaDatabase(password: string): Promise<EsitoSblocco> {
  return invoke<EsitoSblocco>("sblocca_database", { password });
}

export function bloccaDatabase(): Promise<void> {
  return invoke("blocca_database");
}

/** Salva un backup criptato nella cartella indicata e ne restituisce il percorso. */
export function eseguiBackupManuale(cartellaDestinazione: string): Promise<string> {
  return invoke<string>("esegui_backup_manuale", { cartellaDestinazione });
}

export function ripristinaBackup(percorsoBackup: string, passwordBackup: string): Promise<void> {
  return invoke("ripristina_backup", { percorsoBackup, passwordBackup });
}

export function leggiCartellaBackupAutomatici(): Promise<string> {
  return invoke<string>("cartella_backup_automatici");
}

export function elencaTipologie(soloAttive: boolean): Promise<TipologiaAbbonamento[]> {
  return invoke<TipologiaAbbonamento[]>("elenca_tipologie", { soloAttive });
}

export function creaTipologia(dati: DatiTipologia): Promise<TipologiaAbbonamento> {
  return invoke<TipologiaAbbonamento>("crea_tipologia", { dati });
}

export function modificaTipologia(id: number, dati: DatiTipologia): Promise<TipologiaAbbonamento> {
  return invoke<TipologiaAbbonamento>("modifica_tipologia", { id, dati });
}

export function impostaTipologiaAttiva(id: number, attiva: boolean): Promise<TipologiaAbbonamento> {
  return invoke<TipologiaAbbonamento>("imposta_tipologia_attiva", { id, attiva });
}

export function elencaIscritti(filtro: FiltroIscritti): Promise<Iscritto[]> {
  return invoke<Iscritto[]>("elenca_iscritti", { filtro });
}

export function trovaIscritto(id: number): Promise<Iscritto> {
  return invoke<Iscritto>("trova_iscritto", { id });
}

export function creaIscritto(dati: DatiIscritto): Promise<Iscritto> {
  return invoke<Iscritto>("crea_iscritto", { dati });
}

export function modificaIscritto(id: number, dati: DatiIscritto): Promise<Iscritto> {
  return invoke<Iscritto>("modifica_iscritto", { id, dati });
}

export function impostaIscrittoArchiviato(id: number, archiviato: boolean): Promise<Iscritto> {
  return invoke<Iscritto>("imposta_iscritto_archiviato", { id, archiviato });
}

/** Estrae un messaggio leggibile da un errore del backend o imprevisto. */
export function messaggioErrore(errore: unknown): string {
  if (typeof errore === "object" && errore !== null && "messaggio" in errore) {
    return (errore as ErroreApp).messaggio;
  }
  if (typeof errore === "string") {
    return errore;
  }
  return "Si è verificato un errore imprevisto";
}
