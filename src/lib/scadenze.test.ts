import { describe, expect, it } from "vitest";
import { filtroScadenze, gruppiConScaduti, gruppoDaMostrare, statiDaMostrare } from "./scadenze";
import type { ConteggioStati, RiepilogoDashboard } from "./types";

describe("filtroScadenze", () => {
  it("filtra solo il tipo richiesto", () => {
    const filtro = filtroScadenze("certificato", "scaduto");
    expect(filtro.statoCertificato).toBe("scaduto");
    expect(filtro.statoAbbonamento).toBeNull();
    expect(filtro.statoIscrizione).toBeNull();
  });

  it("ordina per vicinanza, recenza o nome", () => {
    expect(filtroScadenze("abbonamento", "in_scadenza").ordinamento).toBe(
      "scadenza_abbonamento_crescente",
    );
    expect(filtroScadenze("iscrizione", "scaduto").ordinamento).toBe("fine_iscrizione_decrescente");
    expect(filtroScadenze("certificato", "nessuno").ordinamento).toBe("alfabetico");
  });
});

describe("statiDaMostrare", () => {
  it("mostra i mancanti solo per i certificati", () => {
    expect(statiDaMostrare("certificato")).toContain("nessuno");
    expect(statiDaMostrare("abbonamento")).not.toContain("nessuno");
  });
});

function stati(scaduti: number): ConteggioStati {
  return { attivi: 0, inScadenza: 0, scaduti, assenti: 0 };
}

function riepilogo(
  abbonamenti: number,
  iscrizioni: number,
  certificati: number,
): RiepilogoDashboard {
  const zero = { abbonamentiCent: 0, iscrizioniCent: 0, totaleCent: 0 };
  return {
    iscritti: 10,
    abbonamenti: stati(abbonamenti),
    iscrizioni: stati(iscrizioni),
    certificati: stati(certificati),
    incassiMese: zero,
    incassiMesePrecedente: zero,
    prossimeScadenze: [],
  };
}

describe("gruppiConScaduti", () => {
  it("tiene solo i tipi con qualcuno scaduto, nell'ordine fisso", () => {
    expect(gruppiConScaduti(riepilogo(3, 0, 1))).toEqual([
      { tipo: "abbonamento", numero: 3 },
      { tipo: "certificato", numero: 1 },
    ]);
  });

  it("senza scaduti non c'è nulla da mostrare", () => {
    expect(gruppiConScaduti(riepilogo(0, 0, 0))).toEqual([]);
  });
});

describe("gruppoDaMostrare", () => {
  const gruppi = gruppiConScaduti(riepilogo(0, 2, 1));

  it("all'inizio mostra il primo gruppo con scaduti", () => {
    expect(gruppoDaMostrare(gruppi, null)).toBe("iscrizione");
  });

  it("resta sul gruppo scelto finché ha scaduti", () => {
    expect(gruppoDaMostrare(gruppi, "certificato")).toBe("certificato");
  });

  it("se il gruppo scelto si svuota passa al primo rimasto", () => {
    expect(gruppoDaMostrare(gruppi, "abbonamento")).toBe("iscrizione");
    expect(gruppoDaMostrare([], "abbonamento")).toBeNull();
  });
});
