import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { DURATA_AVVISO_MS, avvisi, chiudiAvviso, mostraAvviso } from "./avvisi.svelte";

describe("avvisi", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    avvisi.splice(0);
  });

  afterEach(() => vi.useRealTimers());

  it("aggiunge un avviso con un id univoco", () => {
    mostraAvviso("errore", "primo");
    mostraAvviso("successo", "secondo");
    expect(avvisi.map((avviso) => avviso.testo)).toEqual(["primo", "secondo"]);
    expect(avvisi[0].id).not.toBe(avvisi[1].id);
  });

  it("chiude solo l'avviso indicato", () => {
    mostraAvviso("avviso", "da tenere");
    mostraAvviso("avviso", "da chiudere");
    chiudiAvviso(avvisi[1].id);
    expect(avvisi.map((avviso) => avviso.testo)).toEqual(["da tenere"]);
  });

  it("toglie le conferme da sole dopo pochi secondi", () => {
    mostraAvviso("successo", "salvato");
    vi.advanceTimersByTime(DURATA_AVVISO_MS.successo - 1);
    expect(avvisi).toHaveLength(1);
    vi.advanceTimersByTime(1);
    expect(avvisi).toHaveLength(0);
  });

  it("lascia gli errori visibili più a lungo delle conferme", () => {
    mostraAvviso("errore", "non riuscito");
    mostraAvviso("successo", "salvato");
    vi.advanceTimersByTime(DURATA_AVVISO_MS.successo);
    expect(avvisi.map((avviso) => avviso.testo)).toEqual(["non riuscito"]);
    vi.advanceTimersByTime(DURATA_AVVISO_MS.errore);
    expect(avvisi).toHaveLength(0);
  });

  it("non tocca gli altri se un avviso era già stato chiuso a mano", () => {
    mostraAvviso("successo", "chiuso");
    chiudiAvviso(avvisi[0].id);
    mostraAvviso("errore", "resta");
    vi.advanceTimersByTime(DURATA_AVVISO_MS.successo);
    expect(avvisi.map((avviso) => avviso.testo)).toEqual(["resta"]);
  });
});
