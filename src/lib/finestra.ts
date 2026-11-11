import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";
import { LogicalSize, Window, getCurrentWindow } from "@tauri-apps/api/window";

/**
 * L'app ha due finestre (vedi tauri.conf.json):
 * - "accesso": compatta, arrotondata e senza bordi, per password e primo avvio;
 * - "principale": normale, con bordi e pulsanti, massimizzata.
 * Usare due finestre è più affidabile che togliere e rimettere i bordi a una sola.
 */
export const ETICHETTA_FINESTRA_ACCESSO = "accesso";
export const ETICHETTA_FINESTRA_PRINCIPALE = "principale";

export const DIMENSIONE_SBLOCCO = { larghezza: 400, altezza: 560 };
export const DIMENSIONE_PRIMO_AVVIO = { larghezza: 440, altezza: 740 };

/** Evento inviato a tutte le finestre quando il database viene sbloccato o bloccato. */
const EVENTO_STATO_DATABASE_CAMBIATO = "stato-database-cambiato";

export function eFinestraDiAccesso(): boolean {
  return getCurrentWindow().label === ETICHETTA_FINESTRA_ACCESSO;
}

/** Dati dell'evento: un avviso per la finestra principale o il motivo di un blocco. */
export interface CambioStatoDatabase {
  avviso: string | null;
  /** Perché il programma si è bloccato (es. inattività), mostrato sopra la password. */
  motivoBlocco?: string | null;
}

/** Dopo lo sblocco: mostra la finestra principale e nasconde quella di accesso. */
export async function passaAllaFinestraPrincipale(avviso: string | null = null): Promise<void> {
  await emit<CambioStatoDatabase>(EVENTO_STATO_DATABASE_CAMBIATO, { avviso });
  const principale = await trovaFinestra(ETICHETTA_FINESTRA_PRINCIPALE);
  await principale.show();
  await principale.maximize();
  await principale.setFocus();
  await (await trovaFinestra(ETICHETTA_FINESTRA_ACCESSO)).hide();
}

/** Dopo il blocco: torna alla finestra di accesso e nasconde quella principale. */
export async function passaAllaFinestraDiAccesso(
  motivoBlocco: string | null = null,
): Promise<void> {
  await emit<CambioStatoDatabase>(EVENTO_STATO_DATABASE_CAMBIATO, { avviso: null, motivoBlocco });
  const principale = await trovaFinestra(ETICHETTA_FINESTRA_PRINCIPALE);
  await principale.setFullscreen(false);
  const accesso = await trovaFinestra(ETICHETTA_FINESTRA_ACCESSO);
  await accesso.show();
  await accesso.setFocus();
  await principale.hide();
}

/** La finestra di accesso è più alta al primo avvio, per l'avviso e la robustezza della password. */
export async function adattaFinestraDiAccesso(primoAvvio: boolean): Promise<void> {
  const dimensione = primoAvvio ? DIMENSIONE_PRIMO_AVVIO : DIMENSIONE_SBLOCCO;
  const finestra = getCurrentWindow();
  await finestra.setSize(new LogicalSize(dimensione.larghezza, dimensione.altezza));
  await finestra.center();
}

export function quandoCambiaStatoDatabase(
  azione: (cambio: CambioStatoDatabase) => void,
): Promise<UnlistenFn> {
  return listen<CambioStatoDatabase>(EVENTO_STATO_DATABASE_CAMBIATO, (evento) =>
    azione(evento.payload),
  );
}

/** Chiudere una finestra chiude tutto il programma (gestito nel backend). */
export function chiudiApp(): Promise<void> {
  return getCurrentWindow().close();
}

async function trovaFinestra(etichetta: string): Promise<Window> {
  const finestra = await Window.getByLabel(etichetta);
  if (!finestra) throw new Error(`Finestra "${etichetta}" non trovata`);
  return finestra;
}
