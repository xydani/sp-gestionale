import { describe, expect, it } from "vitest";
import { calcolaPeriodoRapido, meseCorrente, raggruppamentoAdatto } from "./periodo";

describe("meseCorrente", () => {
  it("va dal primo all'ultimo giorno del mese", () => {
    expect(meseCorrente(new Date(2026, 10, 10))).toEqual({ dal: "2026-11-01", al: "2026-11-30" });
  });

  it("gestisce febbraio negli anni bisestili", () => {
    expect(meseCorrente(new Date(2028, 1, 15))).toEqual({ dal: "2028-02-01", al: "2028-02-29" });
  });

  it("gestisce dicembre", () => {
    expect(meseCorrente(new Date(2026, 11, 31)).al).toBe("2026-12-31");
  });
});

describe("calcolaPeriodoRapido", () => {
  const adesso = new Date(2026, 0, 15);

  it("calcola oggi, mese e anno", () => {
    expect(calcolaPeriodoRapido("oggi", adesso)).toEqual({ dal: "2026-01-15", al: "2026-01-15" });
    expect(calcolaPeriodoRapido("mese", adesso)).toEqual({ dal: "2026-01-01", al: "2026-01-31" });
    expect(calcolaPeriodoRapido("anno", adesso)).toEqual({ dal: "2026-01-01", al: "2026-12-31" });
  });

  it("il mese scorso di gennaio è dicembre dell'anno prima", () => {
    expect(calcolaPeriodoRapido("mese_scorso", adesso)).toEqual({
      dal: "2025-12-01",
      al: "2025-12-31",
    });
  });

  it("calcola l'anno scorso", () => {
    expect(calcolaPeriodoRapido("anno_scorso", adesso)).toEqual({
      dal: "2025-01-01",
      al: "2025-12-31",
    });
  });
});

describe("raggruppamentoAdatto", () => {
  it("usa i giorni fino a un mese", () => {
    expect(raggruppamentoAdatto({ dal: "2026-11-01", al: "2026-11-30" })).toBe("giorno");
    expect(raggruppamentoAdatto({ dal: "2026-01-01", al: "2026-01-31" })).toBe("giorno");
  });

  it("usa i mesi fino a due anni", () => {
    expect(raggruppamentoAdatto({ dal: "2026-01-01", al: "2026-12-31" })).toBe("mese");
  });

  it("usa gli anni oltre", () => {
    expect(raggruppamentoAdatto({ dal: "2020-01-01", al: "2026-12-31" })).toBe("anno");
  });
});
