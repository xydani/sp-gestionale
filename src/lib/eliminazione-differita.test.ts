import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { avvisi } from "./avvisi.svelte";
import {
  ATTESA_ANNULLA_MS,
  completaEliminazioniInSospeso,
  eliminaConAnnulla,
} from "./eliminazione-differita";

function richiesta(elimina = vi.fn().mockResolvedValue(undefined)) {
  return {
    testo: "Pagamento eliminato",
    elimina,
    onEliminato: vi.fn(),
    onRipristinato: vi.fn(),
    messaggioErrore: () => "Errore",
  };
}

describe("eliminaConAnnulla", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    avvisi.splice(0);
  });

  afterEach(() => vi.useRealTimers());

  it("elimina davvero solo dopo l'attesa", async () => {
    const eliminazione = richiesta();
    eliminaConAnnulla(eliminazione);
    expect(avvisi[0].azione?.etichetta).toBe("Annulla");
    await vi.advanceTimersByTimeAsync(ATTESA_ANNULLA_MS - 1);
    expect(eliminazione.elimina).not.toHaveBeenCalled();
    await vi.advanceTimersByTimeAsync(1);
    expect(eliminazione.elimina).toHaveBeenCalledOnce();
    expect(eliminazione.onEliminato).toHaveBeenCalledOnce();
  });

  it("con «Annulla» non elimina nulla e rimette l'elemento", async () => {
    const eliminazione = richiesta();
    eliminaConAnnulla(eliminazione);
    avvisi[0].azione!.esegui();
    await vi.advanceTimersByTimeAsync(ATTESA_ANNULLA_MS);
    expect(eliminazione.elimina).not.toHaveBeenCalled();
    expect(eliminazione.onRipristinato).toHaveBeenCalledOnce();
    expect(avvisi).toHaveLength(0);
  });

  it("se l'eliminazione non riesce mostra l'errore e rimette l'elemento", async () => {
    const eliminazione = richiesta(vi.fn().mockRejectedValue(new Error("no")));
    eliminaConAnnulla(eliminazione);
    await vi.advanceTimersByTimeAsync(ATTESA_ANNULLA_MS);
    expect(eliminazione.onRipristinato).toHaveBeenCalledOnce();
    expect(avvisi.map((avviso) => avviso.tipo)).toEqual(["errore"]);
  });

  it("prima del blocco completa subito le eliminazioni in attesa, una volta sola", async () => {
    const eliminazione = richiesta();
    eliminaConAnnulla(eliminazione);
    await completaEliminazioniInSospeso();
    await vi.advanceTimersByTimeAsync(ATTESA_ANNULLA_MS);
    expect(eliminazione.elimina).toHaveBeenCalledOnce();
  });
});
