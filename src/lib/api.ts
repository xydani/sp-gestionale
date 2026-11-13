import { invoke } from "@tauri-apps/api/core";
import type {
  ConteggioInScadenza,
  DatiIscritto,
  DatiIscrittoGiaAttivo,
  DatiIscrizione,
  DatiNuovoIscritto,
  DatiPagamento,
  DatiTipologia,
  ErroreApp,
  EsitoPromemoria,
  EsitoSblocco,
  FiltroIscritti,
  FiltroPagamenti,
  Impostazioni,
  Iscritto,
  Iscrizione,
  Pagamento,
  Pagina,
  PaginaPagamenti,
  PropostaIscrizione,
  PropostaPagamento,
  Movimento,
  Raggruppamento,
  RiepilogoDashboard,
  RiepilogoIncassi,
  Statistiche,
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
/** Copia criptata dei dati prima di installare un aggiornamento; restituisce il percorso. */
export function eseguiBackupPreAggiornamento(): Promise<string> {
  return invoke<string>("esegui_backup_pre_aggiornamento");
}

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

/** Senza `elementiPerPagina` le pagine sono da 50, il massimo. */
export function elencaIscritti(
  filtro: FiltroIscritti,
  numeroPagina = 1,
  elementiPerPagina?: number,
): Promise<Pagina<Iscritto>> {
  const pagina = { numero: numeroPagina, elementiPerPagina };
  return invoke<Pagina<Iscritto>>("elenca_iscritti", { filtro, pagina });
}

export function trovaIscritto(id: number): Promise<Iscritto> {
  return invoke<Iscritto>("trova_iscritto", { id });
}

export function creaIscritto(dati: DatiIscritto): Promise<Iscritto> {
  return invoke<Iscritto>("crea_iscritto", { dati });
}

/** Crea l'iscritto con iscrizione annuale e primo abbonamento: tutto o niente. */
export function iscriviNuovoIscritto(dati: DatiNuovoIscritto): Promise<Iscritto> {
  return invoke<Iscritto>("iscrivi_nuovo_iscritto", { dati });
}

/** Inserisce chi era già iscritto, con l'abbonamento in corso e la quota come incassi. */
export function registraIscrittoGiaAttivo(dati: DatiIscrittoGiaAttivo): Promise<Iscritto> {
  return invoke<Iscritto>("registra_iscritto_gia_attivo", { dati });
}

export function modificaIscritto(id: number, dati: DatiIscritto): Promise<Iscritto> {
  return invoke<Iscritto>("modifica_iscritto", { id, dati });
}

export function impostaIscrittoArchiviato(id: number, archiviato: boolean): Promise<Iscritto> {
  return invoke<Iscritto>("imposta_iscritto_archiviato", { id, archiviato });
}

/** Cancella per sempre l'anagrafica di un iscritto archiviato; i pagamenti restano anonimi. */
export function eliminaIscrittoDefinitivamente(id: number): Promise<void> {
  return invoke<void>("elimina_iscritto_definitivamente", { id });
}

/** Propone quota e periodo dell'anno sportivo (il successivo se l'iscrizione è ancora valida). */
/** Con `iscrittoId` null (iscritto non ancora creato) propone l'anno sportivo in corso. */
export function proponiIscrizione(iscrittoId: number | null): Promise<PropostaIscrizione> {
  return invoke<PropostaIscrizione>("proponi_iscrizione", { iscrittoId });
}

export function registraIscrizione(dati: DatiIscrizione): Promise<Iscrizione> {
  return invoke<Iscrizione>("registra_iscrizione", { dati });
}

export function elencaIscrizioni(iscrittoId: number): Promise<Iscrizione[]> {
  return invoke<Iscrizione[]>("elenca_iscrizioni", { iscrittoId });
}

export function eliminaIscrizione(id: number): Promise<void> {
  return invoke("elimina_iscrizione", { id });
}

/** Propone importo e date; con `dataInizio` ricalcola la scadenza da quella data. */
/** Con `iscrittoId` null (iscritto non ancora creato) l'abbonamento parte da oggi. */
export function proponiPagamento(
  iscrittoId: number | null,
  tipologiaId: number,
  dataInizio: string | null = null,
): Promise<PropostaPagamento> {
  return invoke<PropostaPagamento>("proponi_pagamento", { iscrittoId, tipologiaId, dataInizio });
}

export function registraPagamento(dati: DatiPagamento): Promise<Pagamento> {
  return invoke<Pagamento>("registra_pagamento", { dati });
}

export function elencaPagamenti(
  filtro: FiltroPagamenti,
  numeroPagina = 1,
): Promise<PaginaPagamenti> {
  return invoke<PaginaPagamenti>("elenca_pagamenti", { filtro, pagina: { numero: numeroPagina } });
}

export function eliminaPagamento(id: number): Promise<void> {
  return invoke("elimina_pagamento", { id });
}

/** Segna come notificate le nuove scadenze nel preavviso e restituisce i conteggi. */
export function controllaPromemoria(): Promise<EsitoPromemoria> {
  return invoke<EsitoPromemoria>("controlla_promemoria");
}

export function contaInScadenza(): Promise<ConteggioInScadenza> {
  return invoke<ConteggioInScadenza>("conta_in_scadenza");
}

export function leggiRiepilogoDashboard(): Promise<RiepilogoDashboard> {
  return invoke<RiepilogoDashboard>("riepilogo_dashboard");
}

export function leggiRiepilogoIncassi(
  dal: string,
  al: string,
  raggruppamento: Raggruppamento,
): Promise<RiepilogoIncassi> {
  return invoke<RiepilogoIncassi>("riepilogo_incassi", { dal, al, raggruppamento });
}

/** Incassi del periodo uno per uno, nell'ordine in cui sono stati registrati. */
export function elencaMovimenti(dal: string, al: string): Promise<Movimento[]> {
  return invoke<Movimento[]>("elenca_movimenti", { dal, al });
}

/** Rinnovi, abbandoni, valore medio, previsione e vendite del periodo. */
export function leggiStatistiche(dal: string, al: string): Promise<Statistiche> {
  return invoke<Statistiche>("leggi_statistiche", { dal, al });
}

/** Scrive nel file indicato i movimenti del periodo in formato CSV. */
export function esportaIncassiCsv(dal: string, al: string, percorso: string): Promise<void> {
  return invoke<void>("esporta_incassi_csv", { dal, al, percorso });
}

export function leggiImpostazioni(): Promise<Impostazioni> {
  return invoke<Impostazioni>("leggi_impostazioni");
}

/** Salva le impostazioni e restituisce i valori come registrati (es. nome senza spazi ai lati). */
export function salvaImpostazioni(impostazioni: Impostazioni): Promise<Impostazioni> {
  return invoke<Impostazioni>("salva_impostazioni", { impostazioni });
}

export function configurazioneInizialeCompletata(): Promise<boolean> {
  return invoke<boolean>("configurazione_iniziale_completata");
}

/** Ultimo passo della configurazione guidata: da qui il programma è utilizzabile. */
export function completaConfigurazioneIniziale(): Promise<void> {
  return invoke<void>("completa_configurazione_iniziale");
}

/** Il logo come indirizzo `data:`, oppure null se non è stato scelto. */
export function leggiLogoPalestra(): Promise<string | null> {
  return invoke<string | null>("leggi_logo_palestra");
}

export function impostaLogoPalestra(percorso: string): Promise<void> {
  return invoke<void>("imposta_logo_palestra", { percorso });
}

export function rimuoviLogoPalestra(): Promise<void> {
  return invoke<void>("rimuovi_logo_palestra");
}

export function cambiaPassword(
  passwordAttuale: string,
  nuovaPassword: string,
  confermaPassword: string,
): Promise<void> {
  return invoke<void>("cambia_password", { passwordAttuale, nuovaPassword, confermaPassword });
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
