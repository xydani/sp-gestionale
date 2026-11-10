import { describe, expect, it } from "vitest";
import { validaNuovaPassword, valutaRobustezzaPassword } from "./password";

describe("validaNuovaPassword", () => {
  it("accetta una password lunga abbastanza e confermata", () => {
    expect(validaNuovaPassword("palestra2026", "palestra2026")).toBeNull();
  });

  it("rifiuta una password troppo corta", () => {
    expect(validaNuovaPassword("corta", "corta")).toBe(
      "La password deve avere almeno 12 caratteri",
    );
  });

  it("conta i caratteri accentati come singoli caratteri", () => {
    // 11 e 12 caratteri, ma più byte: conta la lunghezza percepita
    expect(validaNuovaPassword("àèìòùàèìòùà", "àèìòùàèìòùà")).not.toBeNull();
    expect(validaNuovaPassword("àèìòùàèìòùàè", "àèìòùàèìòùàè")).toBeNull();
  });

  it("rifiuta una conferma diversa", () => {
    expect(validaNuovaPassword("palestra2026", "palestra2025")).toBe(
      "Le due password non coincidono",
    );
  });
});

describe("valutaRobustezzaPassword", () => {
  const livello = (password: string) => valutaRobustezzaPassword(password).livello;

  it("sotto il minimo è troppo corta", () => {
    expect(valutaRobustezzaPassword("Ab1!")).toMatchObject({
      livello: 0,
      etichetta: "Troppo corta",
    });
  });

  it("le sole minuscole bastano appena, i caratteri vari la rendono buona", () => {
    expect(livello("zebratrombag")).toBe(2);
    expect(livello("Zebr4Tromb4g")).toBe(3);
  });

  it("una frase lunga con simboli è ottima e non dà suggerimenti", () => {
    expect(valutaRobustezzaPassword("Zebra verde sul tetto, 1987!")).toEqual({
      livello: 4,
      etichetta: "Ottima",
      suggerimento: null,
    });
  });

  it("ripetizioni e sequenze valgono poco anche se lunghe", () => {
    expect(livello("aaaaaaaaaaaaaaaa")).toBe(1);
    expect(livello("abcdefghijklmnop")).toBe(1);
    expect(livello("1234567890123")).toBe(1);
  });

  it("le parole comuni abbassano il livello e lo segnalano", () => {
    const valutazione = valutaRobustezzaPassword("password2026");
    expect(valutazione.livello).toBe(1);
    expect(valutazione.suggerimento).toContain("parole comuni");
  });
});
