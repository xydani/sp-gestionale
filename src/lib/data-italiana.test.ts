import { describe, expect, it } from "vitest";
import {
  erroreData,
  formattaMentreSiScrive,
  isoInTestoItaliano,
  testoItalianoInIso,
} from "./data-italiana";

describe("testoItalianoInIso", () => {
  it("converte GG/MM/AAAA in ISO", () => {
    expect(testoItalianoInIso("10/11/2026")).toBe("2026-11-10");
    expect(testoItalianoInIso(" 01/02/1990 ")).toBe("1990-02-01");
  });

  it("rifiuta date inesistenti", () => {
    expect(testoItalianoInIso("31/02/2026")).toBeNull();
    expect(testoItalianoInIso("29/02/2026")).toBeNull();
    expect(testoItalianoInIso("00/01/2026")).toBeNull();
    expect(testoItalianoInIso("10/13/2026")).toBeNull();
  });

  it("accetta il 29 febbraio negli anni bisestili", () => {
    expect(testoItalianoInIso("29/02/2028")).toBe("2028-02-29");
  });

  it("rifiuta formati diversi", () => {
    expect(testoItalianoInIso("2026-11-10")).toBeNull();
    expect(testoItalianoInIso("1/11/2026")).toBeNull();
    expect(testoItalianoInIso("10/11/26")).toBeNull();
  });
});

describe("isoInTestoItaliano", () => {
  it("converte ISO in GG/MM/AAAA", () => {
    expect(isoInTestoItaliano("2026-11-10")).toBe("10/11/2026");
    expect(isoInTestoItaliano(null)).toBe("");
  });
});

describe("formattaMentreSiScrive", () => {
  it("inserisce le barre e ignora i caratteri non numerici", () => {
    expect(formattaMentreSiScrive("1")).toBe("1");
    expect(formattaMentreSiScrive("101")).toBe("10/1");
    expect(formattaMentreSiScrive("10112026")).toBe("10/11/2026");
    expect(formattaMentreSiScrive("10-11-2026")).toBe("10/11/2026");
    expect(formattaMentreSiScrive("101120269")).toBe("10/11/2026");
  });
});

describe("erroreData", () => {
  it("richiede la data solo se obbligatoria", () => {
    expect(erroreData("", { obbligatoria: true })).toBe("Inserisci la data (GG/MM/AAAA)");
    expect(erroreData("")).toBeNull();
  });

  it("segnala date non valide e fuori dai limiti", () => {
    expect(erroreData("31/02/2026")).toContain("non valida");
    expect(erroreData("01/01/2026", { minimaIso: "2026-02-01" })).toBe(
      "La data non può essere prima del 01/02/2026",
    );
    expect(erroreData("01/03/2026", { massimaIso: "2026-02-01" })).toBe(
      "La data non può essere dopo il 01/02/2026",
    );
    expect(
      erroreData("01/02/2026", { minimaIso: "2026-02-01", massimaIso: "2026-02-01" }),
    ).toBeNull();
  });
});
