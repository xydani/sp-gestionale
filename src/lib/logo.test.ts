import { describe, expect, it } from "vitest";
import { PERCORSI_LOGO, prossimoPercorsoLogo } from "./logo";

describe("prossimoPercorsoLogo", () => {
  it("passa dall'SVG al PNG", () => {
    expect(prossimoPercorsoLogo(`http://localhost${PERCORSI_LOGO[0]}`)).toBe(PERCORSI_LOGO[1]);
  });

  it("si ferma dopo l'ultimo", () => {
    expect(prossimoPercorsoLogo(PERCORSI_LOGO[1])).toBeNull();
  });
});
