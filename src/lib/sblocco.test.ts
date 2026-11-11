import { describe, expect, it } from "vitest";
import { maiuscoleAttive, motivoBloccoInattivita } from "./sblocco";

describe("schermata di sblocco", () => {
  it("spiega il blocco per inattività con i minuti giusti", () => {
    expect(motivoBloccoInattivita(15)).toContain("dopo 15 minuti senza attività");
    expect(motivoBloccoInattivita(1)).toContain("dopo 1 minuto senza attività");
  });

  it("riconosce il blocco maiuscole", () => {
    const evento = (attivo: boolean) => ({
      getModifierState: (tasto: string) => attivo && tasto === "CapsLock",
    });
    expect(maiuscoleAttive(evento(true))).toBe(true);
    expect(maiuscoleAttive(evento(false))).toBe(false);
  });
});
