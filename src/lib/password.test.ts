import { describe, expect, it } from "vitest";
import { validaNuovaPassword } from "./password";

describe("validaNuovaPassword", () => {
  it("accetta una password lunga abbastanza e confermata", () => {
    expect(validaNuovaPassword("palestra2026", "palestra2026")).toBeNull();
  });

  it("rifiuta una password troppo corta", () => {
    expect(validaNuovaPassword("corta", "corta")).toBe("La password deve avere almeno 8 caratteri");
  });

  it("conta i caratteri accentati come singoli caratteri", () => {
    expect(validaNuovaPassword("àèìòùàè", "àèìòùàè")).not.toBeNull();
    expect(validaNuovaPassword("àèìòùàèé", "àèìòùàèé")).toBeNull();
  });

  it("rifiuta una conferma diversa", () => {
    expect(validaNuovaPassword("palestra2026", "palestra2025")).toBe(
      "Le due password non coincidono",
    );
  });
});
