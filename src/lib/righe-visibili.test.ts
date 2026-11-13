import { describe, expect, it } from "vitest";
import {
  RIGHE_MASSIME,
  RIGHE_MINIME,
  calcolaRigheVisibili,
  paginaDopoCambioRighe,
  type MisureTabella,
} from "./righe-visibili";

/** Un portatile da 15": finestra alta 738, tabella che parte a 230. */
const PORTATILE: MisureTabella = {
  altezzaFinestra: 738,
  inizioTabella: 230,
  altezzaFissa: 33,
  altezzaRiga: 42,
  spazioSotto: 60,
};

describe("calcolaRigheVisibili", () => {
  it("conta solo le righe intere che stanno sopra lo spazio riservato", () => {
    // 738 - 230 - 33 - 60 = 415 → 9 righe da 42
    expect(calcolaRigheVisibili(PORTATILE)).toBe(9);
  });

  it("su uno schermo più alto ci stanno più righe", () => {
    expect(calcolaRigheVisibili({ ...PORTATILE, altezzaFinestra: 1040 })).toBe(17);
  });

  it("con il testo più grande le righe sono più alte e ne stanno meno", () => {
    expect(calcolaRigheVisibili({ ...PORTATILE, altezzaRiga: 52 })).toBe(7);
  });

  it("resta tra il minimo e il massimo", () => {
    expect(calcolaRigheVisibili({ ...PORTATILE, altezzaFinestra: 300 })).toBe(RIGHE_MINIME);
    expect(calcolaRigheVisibili({ ...PORTATILE, altezzaFinestra: 9000 })).toBe(RIGHE_MASSIME);
  });

  it("un'altezza di riga nulla non manda in errore il calcolo", () => {
    expect(calcolaRigheVisibili({ ...PORTATILE, altezzaRiga: 0 })).toBe(RIGHE_MASSIME);
  });
});

describe("paginaDopoCambioRighe", () => {
  it("dalla prima pagina si resta sulla prima", () => {
    expect(paginaDopoCambioRighe(1, 10, 7)).toBe(1);
  });

  it("si resta sulla pagina che contiene la prima riga vista", () => {
    // Pagina 3 da 10: prima riga vista la 21ª → con pagine da 7 è nella 3ª (15-21)
    expect(paginaDopoCambioRighe(3, 10, 7)).toBe(3);
    // Pagina 4 da 10: 31ª riga → con pagine da 15 è nella 3ª (31-45)
    expect(paginaDopoCambioRighe(4, 10, 15)).toBe(3);
  });
});
