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

describe("api tipologie", () => {
  beforeEach(() => invoke.mockReset());

  it("elenca solo le tipologie attive se richiesto", async () => {
    invoke.mockResolvedValue([]);
    await api.elencaTipologie(true);
    expect(invoke).toHaveBeenCalledWith("elenca_tipologie", { soloAttive: true });
  });

  it("modifica una tipologia inviando id e dati", async () => {
    const dati = {
      nome: "Mensile",
      durataValore: 1,
      durataUnita: "mesi" as const,
      prezzoCent: 5500,
    };
    invoke.mockResolvedValue({ id: 1, attiva: true, ...dati });
    await api.modificaTipologia(1, dati);
    expect(invoke).toHaveBeenCalledWith("modifica_tipologia", { id: 1, dati });
  });

  it("disattiva una tipologia", async () => {
    invoke.mockResolvedValue({});
    await api.impostaTipologiaAttiva(4, false);
    expect(invoke).toHaveBeenCalledWith("imposta_tipologia_attiva", { id: 4, attiva: false });
  });
});

describe("api iscritti", () => {
  beforeEach(() => invoke.mockReset());

  it("elenca gli iscritti con il filtro", async () => {
    invoke.mockResolvedValue([]);
    const filtro = {
      testo: "ross",
      includiArchiviati: false,
      statoAbbonamento: "scaduto" as const,
      statoIscrizione: null,
    };
    await api.elencaIscritti(filtro, 3);
    expect(invoke).toHaveBeenCalledWith("elenca_iscritti", { filtro, pagina: { numero: 3 } });
  });

  it("archivia un iscritto", async () => {
    invoke.mockResolvedValue({});
    await api.impostaIscrittoArchiviato(7, true);
    expect(invoke).toHaveBeenCalledWith("imposta_iscritto_archiviato", { id: 7, archiviato: true });
  });
});

describe("api pagamenti", () => {
  beforeEach(() => invoke.mockReset());

  it("chiede la proposta senza data di inizio per il rinnovo automatico", async () => {
    invoke.mockResolvedValue({});
    await api.proponiPagamento(3, 1);
    expect(invoke).toHaveBeenCalledWith("proponi_pagamento", {
      iscrittoId: 3,
      tipologiaId: 1,
      dataInizio: null,
    });
  });

  it("chiede la proposta con una data di inizio scelta", async () => {
    invoke.mockResolvedValue({});
    await api.proponiPagamento(3, 1, "2026-12-01");
    expect(invoke).toHaveBeenCalledWith("proponi_pagamento", {
      iscrittoId: 3,
      tipologiaId: 1,
      dataInizio: "2026-12-01",
    });
  });

  it("elenca i pagamenti con il filtro", async () => {
    invoke.mockResolvedValue([]);
    const filtro = { dal: "2026-11-01", al: "2026-11-30", metodo: null, iscrittoId: null };
    await api.elencaPagamenti(filtro);
    expect(invoke).toHaveBeenCalledWith("elenca_pagamenti", { filtro, pagina: { numero: 1 } });
  });
});

describe("api iscrizioni", () => {
  beforeEach(() => invoke.mockReset());

  it("chiede la proposta di iscrizione per l'iscritto", async () => {
    invoke.mockResolvedValue({});
    await api.proponiIscrizione(5);
    expect(invoke).toHaveBeenCalledWith("proponi_iscrizione", { iscrittoId: 5 });
  });

  it("elenca lo storico iscrizioni dell'iscritto", async () => {
    invoke.mockResolvedValue([]);
    await api.elencaIscrizioni(5);
    expect(invoke).toHaveBeenCalledWith("elenca_iscrizioni", { iscrittoId: 5 });
  });
});

describe("api promemoria", () => {
  beforeEach(() => invoke.mockReset());

  it("controlla i promemoria", async () => {
    invoke.mockResolvedValue({});
    await api.controllaPromemoria();
    expect(invoke).toHaveBeenCalledWith("controlla_promemoria");
  });
});

describe("riepilogo economico", () => {
  beforeEach(() => invoke.mockReset());

  it("chiede il riepilogo del periodo con il raggruppamento", async () => {
    await api.leggiRiepilogoIncassi("2026-01-01", "2026-12-31", "mese");
    expect(invoke).toHaveBeenCalledWith("riepilogo_incassi", {
      dal: "2026-01-01",
      al: "2026-12-31",
      raggruppamento: "mese",
    });
  });

  it("esporta il CSV nel percorso scelto", async () => {
    await api.esportaIncassiCsv("2026-11-01", "2026-11-30", "/tmp/incassi.csv");
    expect(invoke).toHaveBeenCalledWith("esporta_incassi_csv", {
      dal: "2026-11-01",
      al: "2026-11-30",
      percorso: "/tmp/incassi.csv",
    });
  });
});

describe("impostazioni", () => {
  beforeEach(() => invoke.mockReset());

  it("salva le impostazioni passandole intere", async () => {
    const impostazioni = {
      nomePalestra: "Palestra Esempio",
      giorniPreavviso: 7,
      quotaIscrizioneCent: 3000,
      meseInizioAnnoSportivo: 9,
      backupConservati: 30,
      minutiBloccoAutomatico: 15,
    };
    await api.salvaImpostazioni(impostazioni);
    expect(invoke).toHaveBeenCalledWith("salva_impostazioni", { impostazioni });
  });

  it("cambia la password con quella attuale e la conferma", async () => {
    await api.cambiaPassword("vecchia-password", "nuova-password", "nuova-password");
    expect(invoke).toHaveBeenCalledWith("cambia_password", {
      passwordAttuale: "vecchia-password",
      nuovaPassword: "nuova-password",
      confermaPassword: "nuova-password",
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
