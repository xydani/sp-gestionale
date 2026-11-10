import { beforeEach, describe, expect, it } from "vitest";
import { avvisi, chiudiAvviso, mostraAvviso } from "./avvisi.svelte";

describe("avvisi", () => {
  beforeEach(() => avvisi.splice(0));

  it("aggiunge un avviso con un id univoco", () => {
    mostraAvviso("errore", "primo");
    mostraAvviso("successo", "secondo");
    expect(avvisi.map((avviso) => avviso.testo)).toEqual(["primo", "secondo"]);
    expect(avvisi[0].id).not.toBe(avvisi[1].id);
  });

  it("chiude solo l'avviso indicato", () => {
    mostraAvviso("avviso", "da tenere");
    mostraAvviso("avviso", "da chiudere");
    chiudiAvviso(avvisi[1].id);
    expect(avvisi.map((avviso) => avviso.testo)).toEqual(["da tenere"]);
  });
});
