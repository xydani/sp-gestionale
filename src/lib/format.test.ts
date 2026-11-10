import { describe, expect, it } from "vitest";
import {
  centesimiInTestoEuro,
  descriviDurata,
  euroInCentesimi,
  formattaData,
  formattaEuro,
  oggiIso,
} from "./format";

// Intl separa il simbolo € con uno spazio non separabile
const SPAZIO_NON_SEPARABILE = " ";

describe("formattaEuro", () => {
  it("formatta i centesimi in euro con punto delle migliaia e virgola decimale", () => {
    expect(formattaEuro(12345678)).toBe(`123.456,78${SPAZIO_NON_SEPARABILE}€`);
  });

  it("formatta lo zero", () => {
    expect(formattaEuro(0)).toBe(`0,00${SPAZIO_NON_SEPARABILE}€`);
  });
});

describe("euroInCentesimi", () => {
  it("accetta interi, decimali con virgola e separatore delle migliaia", () => {
    expect(euroInCentesimi("45")).toBe(4500);
    expect(euroInCentesimi("45,5")).toBe(4550);
    expect(euroInCentesimi("1.234,56")).toBe(123456);
    expect(euroInCentesimi(" 50 € ")).toBe(5000);
    expect(euroInCentesimi("0")).toBe(0);
  });

  it("evita errori di arrotondamento", () => {
    expect(euroInCentesimi("0,29")).toBe(29);
    expect(euroInCentesimi("1,15")).toBe(115);
  });

  it("rifiuta testo non valido, negativi e più di due decimali", () => {
    expect(euroInCentesimi("")).toBeNull();
    expect(euroInCentesimi("abc")).toBeNull();
    expect(euroInCentesimi("-5")).toBeNull();
    expect(euroInCentesimi("4,555")).toBeNull();
  });
});

describe("centesimiInTestoEuro", () => {
  it("prepara l'importo per un campo modificabile", () => {
    expect(centesimiInTestoEuro(4550)).toBe("45,50");
    expect(centesimiInTestoEuro(0)).toBe("0,00");
  });
});

describe("descriviDurata", () => {
  it("usa singolare e plurale", () => {
    expect(descriviDurata(1, "mesi")).toBe("1 mese");
    expect(descriviDurata(3, "mesi")).toBe("3 mesi");
    expect(descriviDurata(1, "giorni")).toBe("1 giorno");
    expect(descriviDurata(10, "giorni")).toBe("10 giorni");
  });
});

describe("formattaData", () => {
  it("converte la data ISO nel formato italiano", () => {
    expect(formattaData("2026-11-05")).toBe("05/11/2026");
  });
});

describe("oggiIso", () => {
  it("usa la data locale con zeri iniziali", () => {
    expect(oggiIso(new Date(2026, 0, 5, 23, 30))).toBe("2026-01-05");
  });
});
