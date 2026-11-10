import { beforeEach, describe, expect, it, vi } from "vitest";

const invoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({ invoke }));

const api = await import("./api");

describe("api", () => {
  beforeEach(() => invoke.mockReset());

  it("legge lo stato del database", async () => {
    invoke.mockResolvedValue("bloccato");
    await expect(api.leggiStatoDatabase()).resolves.toBe("bloccato");
    expect(invoke).toHaveBeenCalledWith("stato_database");
  });

  it("crea il database passando password e conferma in camelCase", async () => {
    invoke.mockResolvedValue(undefined);
    await api.creaDatabase("segreta123", "segreta123");
    expect(invoke).toHaveBeenCalledWith("crea_database", {
      password: "segreta123",
      confermaPassword: "segreta123",
    });
  });

  it("sblocca il database con la password", async () => {
    invoke.mockResolvedValue(undefined);
    await api.sbloccaDatabase("segreta123");
    expect(invoke).toHaveBeenCalledWith("sblocca_database", { password: "segreta123" });
  });
});

describe("messaggioErrore", () => {
  it("usa il messaggio degli errori del backend", () => {
    expect(api.messaggioErrore({ codice: "password_errata", messaggio: "Password errata" })).toBe(
      "Password errata",
    );
  });

  it("usa il testo degli errori stringa", () => {
    expect(api.messaggioErrore("errore di rete")).toBe("errore di rete");
  });

  it("ha un messaggio generico per gli errori sconosciuti", () => {
    expect(api.messaggioErrore(42)).toBe("Si è verificato un errore imprevisto");
  });
});
