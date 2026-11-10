import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  INTERVALLO_MINIMO_RIAVVIO_MS,
  avviaTimerInattivita,
  etichettaMinutiBlocco,
} from "./blocco-automatico.svelte";

const MINUTO = 60_000;

describe("avviaTimerInattivita", () => {
  beforeEach(() => vi.useFakeTimers());
  afterEach(() => vi.useRealTimers());

  it("blocca dopo i minuti senza attività", () => {
    const onScaduto = vi.fn();
    avviaTimerInattivita(5, onScaduto);
    vi.advanceTimersByTime(5 * MINUTO - 1);
    expect(onScaduto).not.toHaveBeenCalled();
    vi.advanceTimersByTime(1);
    expect(onScaduto).toHaveBeenCalledOnce();
  });

  it("ogni attività fa ripartire il conto alla rovescia", () => {
    const onScaduto = vi.fn();
    const timer = avviaTimerInattivita(5, onScaduto);
    vi.advanceTimersByTime(4 * MINUTO);
    timer.registraAttivita();
    vi.advanceTimersByTime(4 * MINUTO);
    expect(onScaduto).not.toHaveBeenCalled();
    vi.advanceTimersByTime(MINUTO);
    expect(onScaduto).toHaveBeenCalledOnce();
  });

  it("le attività ravvicinate non riavviano a ogni evento", () => {
    const onScaduto = vi.fn();
    const timer = avviaTimerInattivita(1, onScaduto);
    vi.advanceTimersByTime(INTERVALLO_MINIMO_RIAVVIO_MS / 2);
    timer.registraAttivita();
    vi.advanceTimersByTime(MINUTO - INTERVALLO_MINIMO_RIAVVIO_MS / 2);
    expect(onScaduto).toHaveBeenCalledOnce();
  });

  it("fermato o impostato a zero non blocca mai", () => {
    const onScaduto = vi.fn();
    avviaTimerInattivita(0, onScaduto).registraAttivita();
    avviaTimerInattivita(5, onScaduto).ferma();
    vi.advanceTimersByTime(60 * MINUTO);
    expect(onScaduto).not.toHaveBeenCalled();
  });
});

describe("etichettaMinutiBlocco", () => {
  it("descrive le scelte in italiano", () => {
    expect(etichettaMinutiBlocco(0)).toBe("Mai");
    expect(etichettaMinutiBlocco(15)).toBe("Dopo 15 minuti");
    expect(etichettaMinutiBlocco(60)).toBe("Dopo 1 ora");
  });
});
