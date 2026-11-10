import { describe, expect, it } from "vitest";
import { descriviPasso, eUltimoPasso } from "./configurazione-guidata";

describe("configurazione guidata", () => {
  it("descrive il passo con numero e titolo", () => {
    expect(descriviPasso(0)).toBe("Passo 1 di 2 · Opzioni della palestra");
    expect(descriviPasso(1)).toBe("Passo 2 di 2 · Listino abbonamenti");
  });

  it("il listino è l'ultimo passo", () => {
    expect(eUltimoPasso(0)).toBe(false);
    expect(eUltimoPasso(1)).toBe(true);
  });
});
