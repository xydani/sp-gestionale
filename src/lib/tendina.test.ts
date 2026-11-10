import { describe, expect, it } from "vitest";
import { indiceEstremo, indicePerLettera, indiceSuccessivo, type OpzioneTendina } from "./tendina";

const opzioni: OpzioneTendina<string>[] = [
  { valore: "", etichetta: "Scegli…", disabilitata: true },
  { valore: "m", etichetta: "Mensile" },
  { valore: "t", etichetta: "Trimestrale" },
  { valore: "x", etichetta: "Vecchia", disabilitata: true },
  { valore: "a", etichetta: "Annuale" },
  { valore: "M", etichetta: "Mattino" },
];

describe("indiceSuccessivo", () => {
  it("salta le opzioni disabilitate", () => {
    expect(indiceSuccessivo(opzioni, 2, 1)).toBe(4);
    expect(indiceSuccessivo(opzioni, 4, -1)).toBe(2);
  });

  it("resta fermo ai bordi", () => {
    expect(indiceSuccessivo(opzioni, 5, 1)).toBe(5);
    expect(indiceSuccessivo(opzioni, 1, -1)).toBe(1);
  });
});

describe("indiceEstremo", () => {
  it("trova la prima e l'ultima opzione attivabile", () => {
    expect(indiceEstremo(opzioni, false)).toBe(1);
    expect(indiceEstremo(opzioni, true)).toBe(5);
  });
});

describe("indicePerLettera", () => {
  it("passa tra le voci con la stessa iniziale", () => {
    expect(indicePerLettera(opzioni, "m", 0)).toBe(1);
    expect(indicePerLettera(opzioni, "M", 1)).toBe(5);
    expect(indicePerLettera(opzioni, "m", 5)).toBe(1);
  });

  it("ignora le disabilitate e resta fermo se non trova nulla", () => {
    expect(indicePerLettera(opzioni, "v", 1)).toBe(1);
    expect(indicePerLettera(opzioni, "z", 2)).toBe(2);
  });
});
