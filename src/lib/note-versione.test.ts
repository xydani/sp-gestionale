import { describe, expect, it } from "vitest";
import { estraiNoteVersione } from "../../scripts/note-versione-lib.mjs";

const CHANGELOG = `# Changelog

## [0.2.0] - 2027-01-10

- Novità due

## [0.1.0] - 2026-11-10

- Prima versione
- Seconda riga
`;

describe("estraiNoteVersione", () => {
  it("estrae la sezione della versione fino alla successiva", () => {
    expect(estraiNoteVersione(CHANGELOG, "0.2.0")).toBe("- Novità due");
  });

  it("estrae l'ultima sezione fino alla fine", () => {
    expect(estraiNoteVersione(CHANGELOG, "0.1.0")).toBe("- Prima versione\n- Seconda riga");
  });

  it("restituisce null se la versione manca", () => {
    expect(estraiNoteVersione(CHANGELOG, "9.9.9")).toBeNull();
  });
});
