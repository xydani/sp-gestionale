import { describe, expect, it } from "vitest";
import { iscrizioneNonValida } from "./iscrizione";

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
