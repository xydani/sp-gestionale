import { describe, expect, it } from "vitest";
import { VOCI_MENU, eVoceAttiva } from "./menu";

const dashboard = VOCI_MENU[0];
const iscritti = VOCI_MENU[1];

describe("eVoceAttiva", () => {
  it("la dashboard è attiva solo sulla pagina iniziale", () => {
    expect(eVoceAttiva(dashboard, "/")).toBe(true);
    expect(eVoceAttiva(dashboard, "/iscritti")).toBe(false);
  });

  it("una voce è attiva anche nelle sue sottopagine", () => {
    expect(eVoceAttiva(iscritti, "/iscritti")).toBe(true);
    expect(eVoceAttiva(iscritti, "/iscritti/view")).toBe(true);
  });

  it("non confonde percorsi con lo stesso prefisso", () => {
    expect(eVoceAttiva(iscritti, "/iscrittiarchivio")).toBe(false);
  });
});
