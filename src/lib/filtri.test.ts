import { describe, expect, it } from "vitest";
import { filtriIscrittiIniziali, senzaFiltriIscritti } from "./filtri.svelte";

describe("senzaFiltriIscritti", () => {
  it("vale per i filtri di partenza", () => {
    expect(senzaFiltriIscritti(filtriIscrittiIniziali())).toBe(true);
  });

  it("basta una ricerca o un filtro per non valere più", () => {
    expect(senzaFiltriIscritti({ ...filtriIscrittiIniziali(), testo: "ross" })).toBe(false);
    expect(senzaFiltriIscritti({ ...filtriIscrittiIniziali(), soloDaCompletare: true })).toBe(
      false,
    );
  });
});
