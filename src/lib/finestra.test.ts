import { beforeEach, describe, expect, it, vi } from "vitest";

const finestra = {
  setDecorations: vi.fn(),
  setResizable: vi.fn(),
  setMinSize: vi.fn(),
  maximize: vi.fn(),
  unmaximize: vi.fn(),
  setFullscreen: vi.fn(),
  setSize: vi.fn(),
  center: vi.fn(),
};

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => finestra,
  LogicalSize: class {
    constructor(
      public width: number,
      public height: number,
    ) {}
  },
}));

const { adattaFinestra, modalitaFinestraPerStato, DIMENSIONE_SBLOCCO, DIMENSIONE_PRIMO_AVVIO } =
  await import("./finestra");

describe("modalitaFinestraPerStato", () => {
  it("usa la finestra principale solo dopo lo sblocco", () => {
    expect(modalitaFinestraPerStato("sbloccato")).toBe("principale");
    expect(modalitaFinestraPerStato("bloccato")).toBe("sblocco");
    expect(modalitaFinestraPerStato("da_creare")).toBe("primo_avvio");
  });
});

describe("adattaFinestra", () => {
  beforeEach(() => Object.values(finestra).forEach((funzione) => funzione.mockReset()));

  it("dopo lo sblocco mostra i bordi e massimizza", async () => {
    await adattaFinestra("principale");
    expect(finestra.setDecorations).toHaveBeenCalledWith(true);
    expect(finestra.setResizable).toHaveBeenCalledWith(true);
    expect(finestra.maximize).toHaveBeenCalled();
  });

  it("allo sblocco usa una finestra compatta, senza bordi e centrata", async () => {
    await adattaFinestra("sblocco");
    expect(finestra.setFullscreen).toHaveBeenCalledWith(false);
    expect(finestra.unmaximize).toHaveBeenCalled();
    expect(finestra.setDecorations).toHaveBeenCalledWith(false);
    expect(finestra.setResizable).toHaveBeenCalledWith(false);
    expect(finestra.setSize).toHaveBeenCalledWith({
      width: DIMENSIONE_SBLOCCO.larghezza,
      height: DIMENSIONE_SBLOCCO.altezza,
    });
    expect(finestra.center).toHaveBeenCalled();
  });

  it("al primo avvio la finestra è più alta per l'avviso sulla password", async () => {
    await adattaFinestra("primo_avvio");
    expect(finestra.setSize).toHaveBeenCalledWith({
      width: DIMENSIONE_PRIMO_AVVIO.larghezza,
      height: DIMENSIONE_PRIMO_AVVIO.altezza,
    });
  });
});
