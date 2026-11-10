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

  it("sblocca il database con la password e restituisce l'esito", async () => {
    invoke.mockResolvedValue({ avvisoBackup: null });
    await expect(api.sbloccaDatabase("segreta123")).resolves.toEqual({ avvisoBackup: null });
    expect(invoke).toHaveBeenCalledWith("sblocca_database", { password: "segreta123" });
  });
});

describe("api backup", () => {
  beforeEach(() => invoke.mockReset());

  it("esegue il backup manuale nella cartella scelta", async () => {
    invoke.mockResolvedValue("/media/chiavetta/sp-gestionale-2026-11-10.db");
    await expect(api.eseguiBackupManuale("/media/chiavetta")).resolves.toBe(
      "/media/chiavetta/sp-gestionale-2026-11-10.db",
    );
    expect(invoke).toHaveBeenCalledWith("esegui_backup_manuale", {
      cartellaDestinazione: "/media/chiavetta",
    });
  });

  it("ripristina un backup con la sua password", async () => {
    invoke.mockResolvedValue(undefined);
    await api.ripristinaBackup("/media/chiavetta/backup.db", "segreta123");
    expect(invoke).toHaveBeenCalledWith("ripristina_backup", {
      percorsoBackup: "/media/chiavetta/backup.db",
      passwordBackup: "segreta123",
    });
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
