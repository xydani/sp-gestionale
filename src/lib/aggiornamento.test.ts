import { beforeEach, describe, expect, it, vi } from "vitest";

const mock = vi.hoisted(() => ({
  check: vi.fn(),
  relaunch: vi.fn(),
  backup: vi.fn(),
  ordine: [] as string[],
}));
vi.mock("@tauri-apps/plugin-updater", () => ({ check: mock.check }));
vi.mock("@tauri-apps/plugin-process", () => ({ relaunch: mock.relaunch }));
vi.mock("./api", () => ({
  eseguiBackupPreAggiornamento: mock.backup,
  messaggioErrore: (errore: unknown) => String(errore),
}));

import {
  aggiornamento,
  applicaEventoDownload,
  installaAggiornamento,
  percentualeScaricata,
  rimandaAggiornamento,
  verificaAggiornamenti,
  type StatoAggiornamento,
} from "./aggiornamento.svelte";

function nuovoAggiornamento(installa = vi.fn()) {
  return { version: "0.2.0", body: "Novità", downloadAndInstall: installa, close: vi.fn() };
}

function statoVuoto(): StatoAggiornamento {
  return {
    fase: "download",
    versione: null,
    note: null,
    scaricatiByte: 0,
    totaleByte: null,
    errore: null,
    rimandato: false,
  };
}

describe("eventi di download", () => {
  it("somma i byte e calcola la percentuale", () => {
    const stato = statoVuoto();
    applicaEventoDownload(stato, { event: "Started", data: { contentLength: 200 } });
    applicaEventoDownload(stato, { event: "Progress", data: { chunkLength: 50 } });
    expect(percentualeScaricata(stato)).toBe(25);
    applicaEventoDownload(stato, { event: "Finished" });
    expect(stato.fase).toBe("installazione");
  });

  it("senza dimensione nota la percentuale è assente", () => {
    const stato = statoVuoto();
    applicaEventoDownload(stato, { event: "Started", data: {} });
    expect(percentualeScaricata(stato)).toBeNull();
  });
});

describe("verificaAggiornamenti", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    aggiornamento.fase = "inattivo";
  });

  it("segnala che non ci sono aggiornamenti", async () => {
    mock.check.mockResolvedValue(null);
    await verificaAggiornamenti();
    expect(aggiornamento.fase).toBe("nessuno");
  });

  it("mostra versione e note di una nuova versione", async () => {
    mock.check.mockResolvedValue(nuovoAggiornamento());
    await verificaAggiornamenti();
    expect(aggiornamento).toMatchObject({ fase: "disponibile", versione: "0.2.0", note: "Novità" });
  });

  it("senza internet resta in errore con un messaggio", async () => {
    mock.check.mockRejectedValue("rete non raggiungibile");
    await verificaAggiornamenti();
    expect(aggiornamento.fase).toBe("errore");
    expect(aggiornamento.errore).toContain("rete non raggiungibile");
  });

  it("il controllo automatico ripropone un aggiornamento rimandato", async () => {
    mock.check.mockResolvedValue(nuovoAggiornamento());
    await verificaAggiornamenti();
    rimandaAggiornamento();
    await verificaAggiornamenti();
    expect(aggiornamento.rimandato).toBe(true);
    await verificaAggiornamenti(true);
    expect(aggiornamento.rimandato).toBe(false);
  });
});

describe("installaAggiornamento", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mock.ordine.length = 0;
    aggiornamento.fase = "inattivo";
    mock.backup.mockImplementation(async () => mock.ordine.push("backup"));
    mock.relaunch.mockImplementation(async () => mock.ordine.push("riavvio"));
  });

  it("fa il backup prima di scaricare e poi riavvia", async () => {
    const installa = vi.fn(async () => {
      mock.ordine.push("installazione");
    });
    mock.check.mockResolvedValue(nuovoAggiornamento(installa));
    await verificaAggiornamenti();
    await installaAggiornamento();
    expect(mock.ordine).toEqual(["backup", "installazione", "riavvio"]);
  });

  it("se il backup fallisce non installa nulla", async () => {
    const installa = vi.fn();
    mock.check.mockResolvedValue(nuovoAggiornamento(installa));
    mock.backup.mockRejectedValue("disco pieno");
    await verificaAggiornamenti();
    await installaAggiornamento();
    expect(installa).not.toHaveBeenCalled();
    expect(aggiornamento.fase).toBe("errore");
    expect(aggiornamento.errore).toContain("resta in uso");
  });

  it("una firma non valida lascia il programma attuale", async () => {
    const installa = vi.fn().mockRejectedValue("firma non valida");
    mock.check.mockResolvedValue(nuovoAggiornamento(installa));
    await verificaAggiornamenti();
    await installaAggiornamento();
    expect(mock.relaunch).not.toHaveBeenCalled();
    expect(aggiornamento.errore).toContain("firma non valida");
  });
});
