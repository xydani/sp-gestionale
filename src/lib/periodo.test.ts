import { describe, expect, it } from "vitest";
import { meseCorrente } from "./periodo";

describe("meseCorrente", () => {
  it("va dal primo all'ultimo giorno del mese", () => {
    expect(meseCorrente(new Date(2026, 10, 10))).toEqual({ dal: "2026-11-01", al: "2026-11-30" });
  });

  it("gestisce febbraio negli anni bisestili", () => {
    expect(meseCorrente(new Date(2028, 1, 15))).toEqual({ dal: "2028-02-01", al: "2028-02-29" });
  });

  it("gestisce dicembre", () => {
    expect(meseCorrente(new Date(2026, 11, 31)).al).toBe("2026-12-31");
  });
});
