import { describe, expect, it } from "vitest";
import { descriviIntervallo, numeroPagine, paginaDiElenco } from "./paginazione";

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

describe("paginaDiElenco", () => {
  const giorni = Array.from({ length: 31 }, (_, indice) => indice + 1);

  it("ritaglia la pagina richiesta", () => {
    const seconda = paginaDiElenco(giorni, 2, 10);
    expect(seconda.elementi).toEqual([11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
    expect(seconda.totaleElementi).toBe(31);
  });

  it("l'ultima pagina ha solo gli elementi rimasti", () => {
    expect(paginaDiElenco(giorni, 4, 10).elementi).toEqual([31]);
  });

  it("una pagina oltre l'ultima diventa l'ultima", () => {
    expect(paginaDiElenco(giorni, 9, 10).numero).toBe(4);
    expect(paginaDiElenco([], 3, 10)).toMatchObject({ numero: 1, elementi: [] });
  });
});
