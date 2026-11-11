/**
 * Eliminazioni con «Annulla»: l'elemento sparisce subito, ma il programma lo cancella
 * davvero solo dopo qualche secondo, se nel frattempo nessuno ha annullato.
 */
import { chiudiAvviso, mostraAvviso } from "./avvisi.svelte";

export const ATTESA_ANNULLA_MS = 10_000;

export interface EliminazioneDifferita {
  /** Testo dell'avviso, per esempio «Pagamento eliminato». */
  testo: string;
  elimina: () => Promise<void>;
  /** Dopo l'eliminazione vera: l'elenco si ricarica. */
  onEliminato: () => void;
  /** Annullata o non riuscita: l'elemento va rimesso in elenco. */
  onRipristinato: () => void;
  messaggioErrore: (errore: unknown) => string;
}

interface InSospeso {
  timer: ReturnType<typeof setTimeout>;
  idAvviso: number;
  esegui: () => Promise<void>;
}

const inSospeso = new Set<InSospeso>();

export function eliminaConAnnulla(richiesta: EliminazioneDifferita): void {
  const voce: InSospeso = {
    timer: setTimeout(() => esegui(), ATTESA_ANNULLA_MS),
    idAvviso: 0,
    esegui,
  };

  async function esegui() {
    if (!inSospeso.delete(voce)) return;
    clearTimeout(voce.timer);
    chiudiAvviso(voce.idAvviso);
    try {
      await richiesta.elimina();
      richiesta.onEliminato();
    } catch (errore) {
      mostraAvviso("errore", richiesta.messaggioErrore(errore));
      richiesta.onRipristinato();
    }
  }

  function annulla() {
    if (!inSospeso.delete(voce)) return;
    clearTimeout(voce.timer);
    chiudiAvviso(voce.idAvviso);
    richiesta.onRipristinato();
  }

  voce.idAvviso = mostraAvviso("successo", richiesta.testo, {
    azione: { etichetta: "Annulla", esegui: annulla },
    durataMs: ATTESA_ANNULLA_MS,
  });
  inSospeso.add(voce);
}

/** Prima di bloccare l'archivio: le eliminazioni in attesa si completano subito. */
export async function completaEliminazioniInSospeso(): Promise<void> {
  const daCompletare = [...inSospeso];
  for (const voce of daCompletare) {
    await voce.esegui();
  }
}
