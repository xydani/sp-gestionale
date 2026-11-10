import { describe, expect, it } from "vitest";
import { formattaEuro } from "./format";

// Intl separa il simbolo € con uno spazio non separabile
const SPAZIO_NON_SEPARABILE = " ";

describe("formattaEuro", () => {
  it("formatta i centesimi in euro con punto delle migliaia e virgola decimale", () => {
    expect(formattaEuro(12345678)).toBe(`123.456,78${SPAZIO_NON_SEPARABILE}€`);
  });

  it("formatta lo zero", () => {
    expect(formattaEuro(0)).toBe(`0,00${SPAZIO_NON_SEPARABILE}€`);
  });
});
