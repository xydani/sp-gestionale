import { describe, expect, it } from "vitest";
import { eTastoSchermoIntero } from "./schermo-intero";

describe("eTastoSchermoIntero", () => {
  it("riconosce F11", () => {
    expect(eTastoSchermoIntero({ key: "F11" })).toBe(true);
  });

  it("ignora gli altri tasti", () => {
    expect(eTastoSchermoIntero({ key: "F5" })).toBe(false);
    expect(eTastoSchermoIntero({ key: "Escape" })).toBe(false);
  });
});
