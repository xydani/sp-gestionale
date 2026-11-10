import { describe, expect, it } from "vitest";
import { PERCORSI_LOGO_PREDEFINITI, prossimoPercorsoLogo } from "./logo";

describe("prossimoPercorsoLogo", () => {
  it("passa dall'SVG al PNG", () => {
    expect(prossimoPercorsoLogo(`http://localhost${PERCORSI_LOGO_PREDEFINITI[0]}`)).toBe(
      PERCORSI_LOGO_PREDEFINITI[1],
    );
  });

  it("senza file locali ripiega sullo stemma", () => {
    expect(prossimoPercorsoLogo(PERCORSI_LOGO_PREDEFINITI[1])).toBe("/stemma.svg");
  });

  it("si ferma dopo lo stemma", () => {
    expect(prossimoPercorsoLogo("/stemma.svg")).toBeNull();
  });
});
