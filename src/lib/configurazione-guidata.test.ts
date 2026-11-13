import { describe, expect, it, vi } from "vitest";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const { descriviPasso, eUltimoPasso, percorsoObbligato } =
  await import("./configurazione-guidata.svelte");

describe("configurazione guidata", () => {
  it("descrive il passo con numero e titolo", () => {
    expect(descriviPasso(0)).toBe("Passo 1 di 2 · Opzioni della palestra");
    expect(descriviPasso(1)).toBe("Passo 2 di 2 · Listino prezzi");
  });

  it("il listino è l'ultimo passo", () => {
    expect(eUltimoPasso(0)).toBe(false);
    expect(eUltimoPasso(1)).toBe(true);
  });
});

describe("percorsoObbligato", () => {
  it("senza configurazione completata porta sempre alla guida", () => {
    expect(percorsoObbligato(false, "/")).toBe("/benvenuto");
    expect(percorsoObbligato(false, "/iscritti")).toBe("/benvenuto");
    expect(percorsoObbligato(false, "/benvenuto")).toBeNull();
  });

  it("a configurazione completata la guida non è più raggiungibile", () => {
    expect(percorsoObbligato(true, "/benvenuto")).toBe("/");
    expect(percorsoObbligato(true, "/pagamenti")).toBeNull();
  });
});
