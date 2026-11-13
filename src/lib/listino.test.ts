import { describe, expect, it } from "vitest";
import {
  descriviAnnoSportivo,
  descriviPrezzoAlMese,
  dividiTipologie,
  prezzoAlMeseCent,
} from "./listino";
import type { TipologiaAbbonamento } from "./types";

function tipologia(dati: Partial<TipologiaAbbonamento>): TipologiaAbbonamento {
  return {
    id: 1,
    nome: "Trimestrale",
    durataValore: 3,
    durataUnita: "mesi",
    prezzoCent: 13500,
    attiva: true,
    ...dati,
  };
}

describe("prezzo al mese", () => {
  it("divide il prezzo per i mesi, arrotondando al centesimo", () => {
    expect(prezzoAlMeseCent(tipologia({}))).toBe(4500);
    expect(prezzoAlMeseCent(tipologia({ durataValore: 12, prezzoCent: 45000 }))).toBe(3750);
    expect(prezzoAlMeseCent(tipologia({ durataValore: 6, prezzoCent: 25000 }))).toBe(4167);
    expect(descriviPrezzoAlMese(tipologia({}))).toBe("45,00 €/mese");
  });

  it("non lo indica per un mese solo o per le durate in giorni", () => {
    expect(prezzoAlMeseCent(tipologia({ durataValore: 1 }))).toBeNull();
    expect(descriviPrezzoAlMese(tipologia({ durataValore: 10, durataUnita: "giorni" }))).toBeNull();
  });
});

describe("dividiTipologie", () => {
  it("separa le attive dalle disattivate mantenendo l'ordine", () => {
    const elenco = [
      tipologia({ id: 1, attiva: true }),
      tipologia({ id: 2, attiva: false }),
      tipologia({ id: 3, attiva: true }),
    ];
    const { attive, disattivate } = dividiTipologie(elenco);
    expect(attive.map((voce) => voce.id)).toEqual([1, 3]);
    expect(disattivate.map((voce) => voce.id)).toEqual([2]);
  });
});

describe("descriviAnnoSportivo", () => {
  it("va dal mese di inizio alla fine del mese prima", () => {
    expect(descriviAnnoSportivo(9)).toBe("da settembre a fine agosto");
    expect(descriviAnnoSportivo(1)).toBe("da gennaio a fine dicembre");
  });
});
