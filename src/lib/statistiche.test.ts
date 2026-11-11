import { describe, expect, it } from "vitest";
import {
  bastanoPerUnTasso,
  calcolaPeriodoStatistiche,
  formattaTasso,
  meseBreve,
  tabellaVenditePerMese,
} from "./statistiche";
import type { VenditeTipologiaMese } from "./types";

const OGGI = "2026-11-10";
const SETTEMBRE = 9;

describe("calcolaPeriodoStatistiche", () => {
  it("gli ultimi 12 mesi finiscono oggi", () => {
    expect(calcolaPeriodoStatistiche("ultimi_12_mesi", SETTEMBRE, OGGI)).toEqual({
      dal: "2025-11-11",
      al: "2026-11-10",
    });
  });

  it("l'anno sportivo segue il mese di inizio impostato", () => {
    expect(calcolaPeriodoStatistiche("anno_sportivo", SETTEMBRE, OGGI)).toEqual({
      dal: "2026-09-01",
      al: "2027-08-31",
    });
    expect(calcolaPeriodoStatistiche("anno_sportivo_precedente", SETTEMBRE, OGGI)).toEqual({
      dal: "2025-09-01",
      al: "2026-08-31",
    });
  });

  it("prima del mese di inizio l'anno sportivo è quello partito l'anno prima", () => {
    expect(calcolaPeriodoStatistiche("anno_sportivo", 12, OGGI).dal).toBe("2025-12-01");
  });
});

describe("tassi", () => {
  it("si mostrano in percentuale", () => {
    expect(formattaTasso(0.8775)).toBe("88%");
    expect(formattaTasso(null)).toBe("—");
  });

  it("servono abbastanza scadenze decise", () => {
    expect(bastanoPerUnTasso({ rinnovati: 3, persi: 1, inAttesa: 20 })).toBe(false);
    expect(bastanoPerUnTasso({ rinnovati: 4, persi: 1, inAttesa: 0 })).toBe(true);
  });
});

describe("tabellaVenditePerMese", () => {
  it("mette una colonna per tipologia e zero dove non si è venduto", () => {
    const vendite = [
      { mese: "2026-10", tipologiaNome: "Mensile", numero: 3 },
      { mese: "2026-11", tipologiaNome: "Annuale", numero: 1 },
    ] as VenditeTipologiaMese[];
    expect(tabellaVenditePerMese(vendite)).toEqual({
      tipologie: ["Annuale", "Mensile"],
      righe: [
        { mese: "2026-10", numeri: { Annuale: 0, Mensile: 3 } },
        { mese: "2026-11", numeri: { Annuale: 1, Mensile: 0 } },
      ],
    });
  });
});

describe("meseBreve", () => {
  it("abbrevia il nome del mese", () => {
    expect(meseBreve("2026-11")).toBe("nov");
  });
});
