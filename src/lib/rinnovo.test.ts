import { describe, expect, it } from "vitest";
import { eRinnovabile, sceltePrecedenti } from "./rinnovo";
import type { Pagamento, TipologiaAbbonamento } from "./types";

const MENSILE = { id: 1, attiva: true } as TipologiaAbbonamento;
const ANNUALE = { id: 4, attiva: true } as TipologiaAbbonamento;

function pagamento(dati: Partial<Pagamento>): Pagamento {
  return { tipologiaId: 1, metodo: "contanti", pregresso: false, ...dati } as Pagamento;
}

describe("sceltePrecedenti", () => {
  it("ripropone tipologia e metodo dell'ultimo pagamento", () => {
    const storico = [
      pagamento({ tipologiaId: 4, metodo: "carta" }),
      pagamento({ tipologiaId: 1, metodo: "contanti" }),
    ];
    expect(sceltePrecedenti(storico, [MENSILE, ANNUALE])).toEqual({
      tipologiaId: 4,
      metodo: "carta",
    });
  });

  it("non propone una tipologia tolta dal listino", () => {
    const storico = [pagamento({ tipologiaId: 9 })];
    expect(sceltePrecedenti(storico, [MENSILE]).tipologiaId).toBeNull();
  });

  it("dopo una voce pregressa usa il metodo di un incasso vero o i contanti", () => {
    expect(sceltePrecedenti([pagamento({ pregresso: true, metodo: "altro" })], [MENSILE])).toEqual({
      tipologiaId: 1,
      metodo: "contanti",
    });
  });

  it("senza storico non propone nulla", () => {
    expect(sceltePrecedenti([], [MENSILE])).toEqual({ tipologiaId: null, metodo: "contanti" });
  });
});

describe("eRinnovabile", () => {
  it("abbonamento e iscrizione si rinnovano, il certificato no", () => {
    expect(eRinnovabile("abbonamento")).toBe(true);
    expect(eRinnovabile("iscrizione")).toBe(true);
    expect(eRinnovabile("certificato")).toBe(false);
  });
});
