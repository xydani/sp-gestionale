import { describe, expect, it } from "vitest";
import {
  centesimiInTestoEuro,
  descriviDurata,
  descriviScadenza,
  giorniTra,
  nomeMese,
  nomeMeseDaNumero,
  variazionePercentuale,
  euroInCentesimi,
  formattaData,
  formattaDataBreve,
  formattaEuro,
  formattaPercentuale,
  formattaPeriodo,
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

  it("legge il punto come decimale quando è seguito da una o due cifre", () => {
    expect(euroInCentesimi("30.50")).toBe(3050);
    expect(euroInCentesimi("30.5")).toBe(3050);
    expect(euroInCentesimi("45.00")).toBe(4500);
    expect(euroInCentesimi("1.000")).toBe(100000);
    expect(euroInCentesimi("1.000.000")).toBe(100000000);
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

describe("giorniTra", () => {
  it("conta i giorni anche attraverso il cambio dell'ora legale", () => {
    expect(giorniTra("2026-10-20", "2026-10-30")).toBe(10);
    expect(giorniTra("2026-03-25", "2026-04-01")).toBe(7);
    expect(giorniTra("2026-11-10", "2026-11-05")).toBe(-5);
  });
});

describe("descriviScadenza", () => {
  it("descrive la distanza dalla scadenza", () => {
    expect(descriviScadenza("2026-11-10", "2026-11-10")).toBe("oggi");
    expect(descriviScadenza("2026-11-11", "2026-11-10")).toBe("domani");
    expect(descriviScadenza("2026-11-15", "2026-11-10")).toBe("tra 5 giorni");
    expect(descriviScadenza("2026-11-09", "2026-11-10")).toBe("ieri");
    expect(descriviScadenza("2026-10-29", "2026-11-10")).toBe("da 12 giorni");
  });
});

describe("nomeMese", () => {
  it("restituisce il nome del mese in italiano", () => {
    expect(nomeMese("2026-11-10")).toBe("novembre");
    expect(nomeMese("2026-01-31")).toBe("gennaio");
  });
});

describe("variazionePercentuale", () => {
  it("calcola la variazione arrotondata con il segno", () => {
    expect(variazionePercentuale(11200, 10000)).toBe("+12%");
    expect(variazionePercentuale(9500, 10000)).toBe("−5%");
    expect(variazionePercentuale(10000, 10000)).toBe("invariato");
  });

  it("non calcola la variazione da zero", () => {
    expect(variazionePercentuale(5000, 0)).toBeNull();
  });
});

describe("formattaPeriodo", () => {
  it("formatta giorni, mesi e anni", () => {
    expect(formattaPeriodo("2026-11-10")).toBe("10/11/2026");
    expect(formattaPeriodo("2026-11")).toBe("novembre 2026");
    expect(formattaPeriodo("2026")).toBe("2026");
  });
});

describe("formattaPercentuale", () => {
  it("arrotonda la quota", () => {
    expect(formattaPercentuale(1, 3)).toBe("33%");
    expect(formattaPercentuale(5, 5)).toBe("100%");
  });

  it("restituisce 0% con totale zero", () => {
    expect(formattaPercentuale(0, 0)).toBe("0%");
  });
});

describe("nomeMeseDaNumero", () => {
  it("restituisce il nome italiano del mese", () => {
    expect(nomeMeseDaNumero(1)).toBe("gennaio");
    expect(nomeMeseDaNumero(9)).toBe("settembre");
    expect(nomeMeseDaNumero(12)).toBe("dicembre");
  });
});

describe("formattaDataBreve", () => {
  it("nell'anno in corso omette l'anno", () => {
    expect(formattaDataBreve("2026-11-05", "2026-11-13")).toBe("05/11");
  });

  it("di un altro anno resta completa", () => {
    expect(formattaDataBreve("2025-12-20", "2026-01-10")).toBe("20/12/2025");
    expect(formattaDataBreve("2027-01-02", "2026-12-30")).toBe("02/01/2027");
  });
});
