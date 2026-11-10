import { describe, expect, it, vi } from "vitest";

vi.mock("@tauri-apps/plugin-dialog", () => ({ ask: vi.fn() }));

const { testoConfermaEliminazione } = await import("./conferma");

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
    });
    expect(testo).toContain("50,00");
    expect(testo).toContain("10/11/2026");
    expect(testo).toContain("Rossi Mario");
    expect(testo).toContain("Mensile");
  });
});
