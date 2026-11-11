import { describe, expect, it, vi } from "vitest";

const { ask } = vi.hoisted(() => ({ ask: vi.fn() }));
vi.mock("@tauri-apps/plugin-dialog", () => ({ ask }));

const { TESTO_AVVISO_ESPORTAZIONE, confermaEsportazioneCsv, testoConfermaEliminazioneIscritto } =
  await import("./conferma");

describe("testoConfermaEliminazioneIscritto", () => {
  it("nomina l'iscritto e spiega cosa resta", () => {
    const testo = testoConfermaEliminazioneIscritto({
      nome: "Giulia",
      cognome: "Esposito",
    } as Parameters<typeof testoConfermaEliminazioneIscritto>[0]);
    expect(testo).toContain("Giulia Esposito");
    expect(testo).toContain("non si possono recuperare");
    expect(testo).toContain("I pagamenti restano");
    expect(testo).toContain("backup");
  });
});

describe("confermaEsportazioneCsv", () => {
  it("avvisa che il file non è criptato e restituisce la scelta", async () => {
    ask.mockResolvedValueOnce(false);
    expect(await confermaEsportazioneCsv()).toBe(false);
    expect(ask).toHaveBeenCalledWith(
      TESTO_AVVISO_ESPORTAZIONE,
      expect.objectContaining({ kind: "warning", okLabel: "Esporta" }),
    );
    expect(TESTO_AVVISO_ESPORTAZIONE).toContain("non è criptato");
  });
});
