import { beforeEach, describe, expect, it, vi } from "vitest";

function finestraFinta(label: string) {
  return {
    label,
    show: vi.fn(),
    hide: vi.fn(),
    maximize: vi.fn(),
    setFocus: vi.fn(),
    setFullscreen: vi.fn(),
    setSize: vi.fn(),
    center: vi.fn(),
  };
}

const accesso = finestraFinta("accesso");
const principale = finestraFinta("principale");
let finestraCorrente = accesso;
const emit = vi.fn();

vi.mock("@tauri-apps/api/event", () => ({ emit, listen: vi.fn() }));
vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => finestraCorrente,
  Window: {
    getByLabel: async (etichetta: string) =>
      ({ accesso, principale })[etichetta as "accesso" | "principale"] ?? null,
  },
  LogicalSize: class {
    constructor(
      public width: number,
      public height: number,
    ) {}
  },
}));

const finestra = await import("./finestra");

function ordine(funzione: ReturnType<typeof vi.fn>): number {
  return funzione.mock.invocationCallOrder[0];
}

describe("finestre di accesso e principale", () => {
  beforeEach(() => {
    [accesso, principale].forEach((f) =>
      Object.values(f).forEach((v) => typeof v === "function" && v.mockReset()),
    );
    emit.mockReset();
    finestraCorrente = accesso;
  });

  it("riconosce la finestra di accesso dall'etichetta", () => {
    expect(finestra.eFinestraDiAccesso()).toBe(true);
    finestraCorrente = principale;
    expect(finestra.eFinestraDiAccesso()).toBe(false);
  });

  it("dopo lo sblocco mostra la principale massimizzata prima di nascondere l'accesso", async () => {
    await finestra.passaAllaFinestraPrincipale("Backup automatico non riuscito");

    expect(emit).toHaveBeenCalledWith("stato-database-cambiato", {
      avviso: "Backup automatico non riuscito",
    });
    expect(principale.show).toHaveBeenCalled();
    expect(principale.maximize).toHaveBeenCalled();
    expect(accesso.hide).toHaveBeenCalled();
    expect(ordine(principale.show)).toBeLessThan(ordine(accesso.hide));
  });

  it("al blocco esce dallo schermo intero e torna alla finestra di accesso", async () => {
    finestraCorrente = principale;
    await finestra.passaAllaFinestraDiAccesso();

    expect(principale.setFullscreen).toHaveBeenCalledWith(false);
    expect(accesso.show).toHaveBeenCalled();
    expect(principale.hide).toHaveBeenCalled();
    expect(ordine(accesso.show)).toBeLessThan(ordine(principale.hide));
  });

  it("la finestra di accesso è più alta al primo avvio", async () => {
    await finestra.adattaFinestraDiAccesso(true);
    expect(accesso.setSize).toHaveBeenCalledWith({
      width: finestra.DIMENSIONE_PRIMO_AVVIO.larghezza,
      height: finestra.DIMENSIONE_PRIMO_AVVIO.altezza,
    });

    await finestra.adattaFinestraDiAccesso(false);
    expect(accesso.setSize).toHaveBeenLastCalledWith({
      width: finestra.DIMENSIONE_SBLOCCO.larghezza,
      height: finestra.DIMENSIONE_SBLOCCO.altezza,
    });
  });
});
