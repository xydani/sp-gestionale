import { describe, expect, it } from "vitest";
import { iscrizioneNonValida, totaleNuovoIscritto } from "./iscrizione";

describe("iscrizioneNonValida", () => {
  it("segnala iscrizione scaduta o assente", () => {
    expect(iscrizioneNonValida({ statoIscrizione: "scaduto" })).toBe(true);
    expect(iscrizioneNonValida({ statoIscrizione: "nessuno" })).toBe(true);
  });

  it("considera valida anche l'iscrizione in scadenza", () => {
    expect(iscrizioneNonValida({ statoIscrizione: "attivo" })).toBe(false);
    expect(iscrizioneNonValida({ statoIscrizione: "in_scadenza" })).toBe(false);
  });
});

describe("totaleNuovoIscritto", () => {
  it("somma quota e abbonamento in centesimi", () => {
    expect(totaleNuovoIscritto("30", "45,50")).toBe(7550);
  });

  it("è null se un importo non è valido", () => {
    expect(totaleNuovoIscritto("trenta", "45")).toBeNull();
    expect(totaleNuovoIscritto("30", "")).toBeNull();
  });
});
