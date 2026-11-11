import { describe, expect, it } from "vitest";
import { sommaImporti, totaliPerMetodo } from "./cassa";
import type { MetodoPagamento, Movimento } from "./types";

function movimento(metodo: MetodoPagamento, importoCent: number): Movimento {
  return { metodo, importoCent } as Movimento;
}

describe("cassa del giorno", () => {
  const movimenti = [
    movimento("carta", 5000),
    movimento("contanti", 3000),
    movimento("carta", 4500),
  ];

  it("somma gli importi", () => {
    expect(sommaImporti(movimenti)).toBe(12500);
    expect(sommaImporti([])).toBe(0);
  });

  it("divide per metodo nell'ordine fisso, saltando quelli non usati", () => {
    expect(totaliPerMetodo(movimenti)).toEqual([
      { metodo: "contanti", totaleCent: 3000, numeroMovimenti: 1 },
      { metodo: "carta", totaleCent: 9500, numeroMovimenti: 2 },
    ]);
  });
});
