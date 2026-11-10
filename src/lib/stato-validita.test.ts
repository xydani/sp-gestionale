import { describe, expect, it } from "vitest";
import { opzioniStato } from "./stato-validita";

describe("opzioniStato", () => {
  it("parte da 'tutti' e prosegue con ogni stato", () => {
    const opzioni = opzioniStato("Abbonamento", "tutti");
    expect(opzioni[0]).toEqual({ valore: "", etichetta: "Abbonamento: tutti" });
    expect(opzioni.map((opzione) => opzione.valore)).toEqual([
      "",
      "attivo",
      "in_scadenza",
      "scaduto",
      "nessuno",
    ]);
    expect(opzioni[2].etichetta).toBe("Abbonamento: in scadenza");
  });
});
