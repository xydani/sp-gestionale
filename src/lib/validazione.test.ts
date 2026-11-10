import { describe, expect, it } from "vitest";
import {
  aggiungiMesiIso,
  erroreEmail,
  erroreImportoEuro,
  erroreNomeProprio,
  erroreTelefono,
  limitiDataNascita,
  limitiDataPagamento,
  limitiFinePeriodo,
  limitiScadenzaCertificato,
} from "./validazione";

describe("erroreNomeProprio", () => {
  const verifica = (testo: string) => erroreNomeProprio("Il luogo di nascita", testo);

  it("accetta nomi con accenti, apostrofi, trattini e punti", () => {
    for (const nome of [
      "Nicolò",
      "D'Angelo",
      "Reggio nell’Emilia",
      "De Rosa-Bianchi",
      "S. Severo",
    ]) {
      expect(verifica(nome)).toBeNull();
    }
  });

  it("rifiuta numeri, simboli e testi senza lettere", () => {
    for (const nome of ["123", "Roma1", "Mario!", "--"]) {
      expect(verifica(nome)).toContain("Il luogo di nascita");
    }
  });

  it("lascia il campo vuoto al controllo di obbligatorietà", () => {
    expect(verifica("  ")).toBeNull();
  });
});

describe("erroreTelefono", () => {
  it("accetta i formati comuni", () => {
    for (const telefono of ["3331234567", "+39 333-123 4567", "(06) 1234.567", ""]) {
      expect(erroreTelefono(telefono)).toBeNull();
    }
  });

  it("rifiuta lettere, poche cifre o troppe", () => {
    for (const telefono of ["abc", "12345", "333+1234567", "+39 333 1234567 89012"]) {
      expect(erroreTelefono(telefono)).not.toBeNull();
    }
  });
});

describe("erroreEmail", () => {
  it("vuole utente, dominio ed estensione", () => {
    expect(erroreEmail("mario.rossi@esempio.it")).toBeNull();
    for (const email of ["mario", "mario@esempio", "@esempio.it", "ma rio@esempio.it"]) {
      expect(erroreEmail(email)).not.toBeNull();
    }
  });
});

describe("aggiungiMesiIso", () => {
  it("somma e sottrae mesi cambiando anno", () => {
    expect(aggiungiMesiIso("2026-11-10", 13)).toBe("2027-12-10");
    expect(aggiungiMesiIso("2026-11-10", -120)).toBe("2016-11-10");
  });

  it("a fine mese si ferma all'ultimo giorno valido", () => {
    expect(aggiungiMesiIso("2026-01-31", 1)).toBe("2026-02-28");
    expect(aggiungiMesiIso("2028-02-29", -12)).toBe("2027-02-28");
  });
});

describe("limiti delle date", () => {
  const oggi = "2026-11-10";

  it("la nascita è tra 120 anni fa e oggi", () => {
    expect(limitiDataNascita(oggi)).toEqual({ minimaIso: "1906-11-10", massimaIso: oggi });
  });

  it("il certificato scade entro 13 mesi", () => {
    expect(limitiScadenzaCertificato(oggi).massimaIso).toBe("2027-12-10");
  });

  it("il pagamento è tra 10 anni fa e oggi", () => {
    expect(limitiDataPagamento(oggi)).toEqual({ minimaIso: "2016-11-10", massimaIso: oggi });
  });

  it("la fine del periodo dipende dall'inizio", () => {
    expect(limitiFinePeriodo("2026-09-01", 12)).toEqual({
      minimaIso: "2026-09-01",
      massimaIso: "2027-09-01",
    });
    expect(limitiFinePeriodo("", 12)).toEqual({ minimaIso: null, massimaIso: null });
  });
});

describe("erroreImportoEuro", () => {
  it("accetta importi da zero a 10.000 €", () => {
    for (const importo of ["0", "30", "30,50", "10000"])
      expect(erroreImportoEuro(importo)).toBeNull();
  });

  it("rifiuta testi e importi esagerati", () => {
    expect(erroreImportoEuro("trenta")).toContain("non valido");
    expect(erroreImportoEuro("10000,01")).toContain("troppo alto");
  });
});
