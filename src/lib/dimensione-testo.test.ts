import { describe, expect, it } from "vitest";
import {
  leggiDimensioneTesto,
  pixelDimensioneTesto,
  salvaDimensioneTesto,
} from "./dimensione-testo";

function archivioInMemoria(): Storage {
  const valori = new Map<string, string>();
  return {
    getItem: (chiave: string) => valori.get(chiave) ?? null,
    setItem: (chiave: string, valore: string) => void valori.set(chiave, valore),
  } as Storage;
}

describe("dimensione del testo", () => {
  it("parte normale e ricorda la scelta", () => {
    const archivio = archivioInMemoria();
    expect(leggiDimensioneTesto(archivio)).toBe("normale");
    salvaDimensioneTesto("grande", archivio);
    expect(leggiDimensioneTesto(archivio)).toBe("grande");
  });

  it("ignora valori sconosciuti e archivi non disponibili", () => {
    const archivio = archivioInMemoria();
    archivio.setItem("dimensione-testo", "enorme");
    expect(leggiDimensioneTesto(archivio)).toBe("normale");
    const guasto = {
      getItem: () => {
        throw new Error("bloccato");
      },
    } as unknown as Storage;
    expect(leggiDimensioneTesto(guasto)).toBe("normale");
  });

  it("ogni scelta ingrandisce la base del testo", () => {
    expect(pixelDimensioneTesto("normale")).toBe(16);
    expect(pixelDimensioneTesto("molto_grande")).toBe(20);
  });
});
