import { beforeEach, describe, expect, it, vi } from "vitest";

const finestra = {
  setDecorations: vi.fn(),
  setResizable: vi.fn(),
  setShadow: vi.fn(),
  setMinSize: vi.fn(),
  maximize: vi.fn(),
  isMaximized: vi.fn(),
  unmaximize: vi.fn(),
  setFullscreen: vi.fn(),
  setSize: vi.fn(),
  setPosition: vi.fn(),
  center: vi.fn(),
};
const schermo = {
  workArea: { size: { width: 1920, height: 1040 }, position: { x: 0, y: 40 } },
};
const currentMonitor = vi.fn();

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => finestra,
  currentMonitor,
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
  beforeEach(() => {
    Object.values(finestra).forEach((funzione) => funzione.mockReset());
    currentMonitor.mockReset().mockResolvedValue(schermo);
  });

  it("dopo lo sblocco rende la finestra ridimensionabile prima di massimizzarla", async () => {
    finestra.isMaximized.mockResolvedValue(true);
    await adattaFinestra("principale");

    expect(finestra.setDecorations).toHaveBeenCalledWith(true);
    expect(finestra.setResizable).toHaveBeenCalledWith(true);
    const ordineResizable = finestra.setResizable.mock.invocationCallOrder[0];
    const ordineMaximize = finestra.maximize.mock.invocationCallOrder[0];
    expect(ordineResizable).toBeLessThan(ordineMaximize);
    expect(finestra.setSize).not.toHaveBeenCalled();
  });

  it("se il sistema ignora la massimizzazione occupa l'area utile dello schermo", async () => {
    finestra.isMaximized.mockResolvedValue(false);
    await adattaFinestra("principale");

    expect(finestra.setSize).toHaveBeenCalledWith(schermo.workArea.size);
    expect(finestra.setPosition).toHaveBeenCalledWith(schermo.workArea.position);
  });

  it("allo sblocco usa una finestra compatta, senza bordi né ombra e centrata", async () => {
    await adattaFinestra("sblocco");
    expect(finestra.setFullscreen).toHaveBeenCalledWith(false);
    expect(finestra.unmaximize).toHaveBeenCalled();
    expect(finestra.setDecorations).toHaveBeenCalledWith(false);
    expect(finestra.setShadow).toHaveBeenCalledWith(false);
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
