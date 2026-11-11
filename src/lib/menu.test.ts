import { describe, expect, it } from "vitest";
import { VOCI_MENU, eTastoBlocca, eVoceAttiva } from "./menu";

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

describe("VOCI_MENU", () => {
  it("ha il listino separato dalle impostazioni, che restano in fondo", () => {
    const percorsi = VOCI_MENU.map((voce) => voce.percorso);
    expect(percorsi).toContain("/listino");
    expect(percorsi.at(-1)).toBe("/impostazioni");
  });
});

describe("eTastoBlocca", () => {
  const tasto = (key: string, ctrlKey = false, metaKey = false) => ({ key, ctrlKey, metaKey });

  it("riconosce Ctrl+L e ⌘L, anche con le maiuscole attive", () => {
    expect(eTastoBlocca(tasto("l", true))).toBe(true);
    expect(eTastoBlocca(tasto("L", true))).toBe(true);
    expect(eTastoBlocca(tasto("l", false, true))).toBe(true);
  });

  it("ignora la L scritta senza modificatore e le altre combinazioni", () => {
    expect(eTastoBlocca(tasto("l"))).toBe(false);
    expect(eTastoBlocca(tasto("k", true))).toBe(false);
  });
});
