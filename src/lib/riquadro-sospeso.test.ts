import { describe, expect, it } from "vitest";
import { calcolaPosizioneSospesa } from "./riquadro-sospeso";

const FINESTRA = { larghezza: 1366, altezza: 738 };
const CALENDARIO = { larghezza: 280, altezza: 300 };

describe("calcolaPosizioneSospesa", () => {
  it("si apre sotto il campo se c'è spazio", () => {
    expect(
      calcolaPosizioneSospesa({ top: 100, bottom: 140, left: 300 }, CALENDARIO, FINESTRA),
    ).toEqual({ top: 146, left: 300 });
  });

  it("si apre sopra il campo se sotto non ci sta e sopra c'è più spazio", () => {
    expect(
      calcolaPosizioneSospesa({ top: 600, bottom: 640, left: 300 }, CALENDARIO, FINESTRA),
    ).toEqual({ top: 294, left: 300 });
  });

  it("resta sotto se neanche sopra c'è più spazio", () => {
    const finestraBassa = { larghezza: 1366, altezza: 400 };
    expect(
      calcolaPosizioneSospesa({ top: 150, bottom: 190, left: 300 }, CALENDARIO, finestraBassa).top,
    ).toBe(196);
  });

  it("non esce dal bordo destro né da quello sinistro", () => {
    expect(
      calcolaPosizioneSospesa({ top: 100, bottom: 140, left: 1200 }, CALENDARIO, FINESTRA).left,
    ).toBe(1366 - 280 - 8);
    expect(
      calcolaPosizioneSospesa({ top: 100, bottom: 140, left: 2 }, CALENDARIO, FINESTRA).left,
    ).toBe(8);
  });
});
