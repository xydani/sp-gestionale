import { describe, expect, it } from "vitest";
import { fuoriIntervallo, grigliaMese, meseIniziale, spostaMese } from "./calendario";

describe("grigliaMese", () => {
  it("parte dal lunedì e copre sei settimane", () => {
    // Il 1° novembre 2026 è domenica: la griglia inizia lunedì 26 ottobre
    const griglia = grigliaMese({ anno: 2026, mese: 11 });
    expect(griglia).toHaveLength(42);
    expect(griglia[0]).toEqual({ iso: "2026-10-26", numero: 26, delMese: false });
    expect(griglia[6]).toEqual({ iso: "2026-11-01", numero: 1, delMese: true });
  });

  it("segna solo i giorni del mese", () => {
    const giorniDelMese = grigliaMese({ anno: 2028, mese: 2 }).filter((giorno) => giorno.delMese);
    expect(giorniDelMese).toHaveLength(29);
  });

  it("se il mese inizia di lunedì non mostra giorni prima", () => {
    expect(grigliaMese({ anno: 2026, mese: 6 })[0].iso).toBe("2026-06-01");
  });
});

describe("spostaMese", () => {
  it("attraversa il cambio d'anno", () => {
    expect(spostaMese({ anno: 2026, mese: 12 }, 1)).toEqual({ anno: 2027, mese: 1 });
    expect(spostaMese({ anno: 2026, mese: 1 }, -1)).toEqual({ anno: 2025, mese: 12 });
  });
});

describe("meseIniziale", () => {
  it("usa la data scelta, altrimenti oggi", () => {
    expect(meseIniziale("2025-03-15", "2026-11-10")).toEqual({ anno: 2025, mese: 3 });
    expect(meseIniziale("", "2026-11-10")).toEqual({ anno: 2026, mese: 11 });
  });
});

describe("fuoriIntervallo", () => {
  it("include gli estremi", () => {
    expect(fuoriIntervallo("2026-11-10", "2026-11-10", "2026-11-10")).toBe(false);
    expect(fuoriIntervallo("2026-11-09", "2026-11-10", null)).toBe(true);
    expect(fuoriIntervallo("2026-11-11", null, "2026-11-10")).toBe(true);
    expect(fuoriIntervallo("2026-11-11", null, null)).toBe(false);
  });
});
