import { describe, expect, it, vi } from "vitest";

const { ask } = vi.hoisted(() => ({ ask: vi.fn() }));
vi.mock("@tauri-apps/plugin-dialog", () => ({ ask }));

const {
  TESTO_AVVISO_ESPORTAZIONE,
  confermaEsportazioneCsv,
  testoConfermaEliminazione,
  testoConfermaEliminazioneIscritto,
} = await import("./conferma");

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

describe("testoConfermaEliminazione", () => {
  it("descrive il pagamento da eliminare", () => {
    const testo = testoConfermaEliminazione({
      id: 1,
      iscrittoId: 2,
      iscrittoNome: "Mario",
      iscrittoCognome: "Rossi",
      tipologiaId: 1,
      tipologiaNome: "Mensile",
      importoCent: 5000,
      dataPagamento: "2026-11-10",
      metodo: "contanti",
      dataInizio: "2026-11-10",
      dataScadenza: "2026-12-09",
      note: null,
      rinnovoDiId: null,
      iscrittoEliminato: false,
      pregresso: false,
    });
    expect(testo).toContain("50,00");
    expect(testo).toContain("10/11/2026");
    expect(testo).toContain("Rossi Mario");
    expect(testo).toContain("Mensile");
  });
});
