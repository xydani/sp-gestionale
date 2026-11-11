import { describe, expect, it } from "vitest";
import { limitiScadenzaPregressa, trovaOmonimi } from "./gia-iscritto";
import type { Iscritto, TipologiaAbbonamento } from "./types";

const OGGI = "2026-11-10";

function tipologia(durataValore: number, durataUnita: "mesi" | "giorni"): TipologiaAbbonamento {
  return { id: 1, nome: "Prova", durataValore, durataUnita, prezzoCent: 5000, attiva: true };
}

function iscritto(nome: string, cognome: string): Iscritto {
  return { nome, cognome } as Iscritto;
}

describe("limitiScadenzaPregressa", () => {
  it("non accetta scadenze di più di un anno fa", () => {
    expect(limitiScadenzaPregressa(undefined, OGGI)).toEqual({
      minimaIso: "2025-11-10",
      massimaIso: null,
    });
  });

  it("con un mensile la scadenza è al massimo quella di chi inizia oggi", () => {
    expect(limitiScadenzaPregressa(tipologia(1, "mesi"), OGGI).massimaIso).toBe("2026-12-09");
  });

  it("con una durata in giorni conta anche il giorno di inizio", () => {
    expect(limitiScadenzaPregressa(tipologia(10, "giorni"), OGGI).massimaIso).toBe("2026-11-19");
  });
});

describe("trovaOmonimi", () => {
  const iscritti = [iscritto("Nicolò", "De Rosa"), iscritto("Mario", "Rossi")];

  it("trova lo stesso nome ignorando maiuscole, accenti e spazi", () => {
    expect(trovaOmonimi(iscritti, " nicolo ", "de  rosa")).toEqual([iscritti[0]]);
  });

  it("non considera omonimi nomi solo simili", () => {
    expect(trovaOmonimi(iscritti, "Mario", "Rossini")).toEqual([]);
  });
});
