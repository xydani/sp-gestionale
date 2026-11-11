import { describe, expect, it } from "vitest";
import { ICONE } from "./icone";

describe("icone", () => {
  it("ogni icona è un tracciato che parte con uno spostamento", () => {
    for (const [nome, tracciato] of Object.entries(ICONE)) {
      expect(tracciato, nome).toMatch(/^M[\d.]/);
    }
  });

  it("contiene solo comandi SVG validi", () => {
    for (const [nome, tracciato] of Object.entries(ICONE)) {
      expect(tracciato, nome).toMatch(/^[MmLlHhVvAaZzSsCcQq\d.\s,-]+$/);
    }
  });
});
