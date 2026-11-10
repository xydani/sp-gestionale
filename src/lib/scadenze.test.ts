import { describe, expect, it } from "vitest";
import { filtroScadenze, statiDaMostrare } from "./scadenze";

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
