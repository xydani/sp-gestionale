import { describe, expect, it } from "vitest";
import { descriviIntervallo, numeroPagine } from "./paginazione";

describe("numeroPagine", () => {
  it("arrotonda per eccesso e vale almeno 1", () => {
    expect(numeroPagine({ totaleElementi: 250, elementiPerPagina: 50 })).toBe(5);
    expect(numeroPagine({ totaleElementi: 251, elementiPerPagina: 50 })).toBe(6);
    expect(numeroPagine({ totaleElementi: 0, elementiPerPagina: 50 })).toBe(1);
  });
});

describe("descriviIntervallo", () => {
  it("descrive gli elementi mostrati", () => {
    expect(descriviIntervallo({ numero: 2, elementiPerPagina: 50, totaleElementi: 250 })).toBe(
      "Da 51 a 100 di 250",
    );
    expect(descriviIntervallo({ numero: 3, elementiPerPagina: 50, totaleElementi: 120 })).toBe(
      "Da 101 a 120 di 120",
    );
    expect(descriviIntervallo({ numero: 1, elementiPerPagina: 50, totaleElementi: 0 })).toBe(
      "Nessun risultato",
    );
  });
});
