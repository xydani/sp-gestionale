import { describe, expect, it } from "vitest";
import {
  calcolaDataInizioDaScadenza,
  calcolaDataScadenza,
  completaDateAbbonamento,
  dateDaInviare,
  limitiInizioPregresso,
  limitiScadenzaPregressa,
  trovaOmonimi,
} from "./gia-iscritto";
import type { Iscritto, TipologiaAbbonamento } from "./types";

const OGGI = "2026-11-10";

function tipologia(durataValore: number, durataUnita: "mesi" | "giorni"): TipologiaAbbonamento {
  return { id: 1, nome: "Prova", durataValore, durataUnita, prezzoCent: 5000, attiva: true };
}

function iscritto(nome: string, cognome: string): Iscritto {
  return { nome, cognome } as Iscritto;
}

describe("limitiScadenzaPregressa", () => {
  it("non accetta scadenze di più di un anno fa", () => {
    expect(limitiScadenzaPregressa(undefined, OGGI)).toEqual({
      minimaIso: "2025-11-10",
      massimaIso: null,
    });
  });

  it("con un mensile la scadenza è al massimo quella di chi inizia oggi", () => {
    expect(limitiScadenzaPregressa(tipologia(1, "mesi"), OGGI).massimaIso).toBe("2026-12-09");
  });

  it("con una durata in giorni conta anche il giorno di inizio", () => {
    expect(limitiScadenzaPregressa(tipologia(10, "giorni"), OGGI).massimaIso).toBe("2026-11-19");
  });
});

describe("limitiInizioPregresso", () => {
  it("l'abbonamento non può iniziare dopo oggi", () => {
    expect(limitiInizioPregresso(undefined, OGGI)).toEqual({ minimaIso: null, massimaIso: OGGI });
  });

  it("e non prima di chi è scaduto da un anno", () => {
    expect(limitiInizioPregresso(tipologia(1, "mesi"), OGGI).minimaIso).toBe("2025-10-11");
  });
});

// Stessi casi dei test di domain/date.rs: modulo e programma devono dare le stesse date
describe("calcolaDataScadenza", () => {
  it("un mensile finisce il giorno prima dello stesso giorno del mese dopo", () => {
    expect(calcolaDataScadenza("2026-10-26", tipologia(1, "mesi"))).toBe("2026-11-25");
  });

  it("se il mese dopo è più corto si ferma all'ultimo giorno", () => {
    expect(calcolaDataScadenza("2026-01-31", tipologia(1, "mesi"))).toBe("2026-02-28");
    expect(calcolaDataScadenza("2026-10-31", tipologia(1, "mesi"))).toBe("2026-11-30");
    expect(calcolaDataScadenza("2028-01-31", tipologia(1, "mesi"))).toBe("2028-02-29");
  });

  it("una durata in giorni conta anche il giorno di inizio", () => {
    expect(calcolaDataScadenza("2026-12-28", tipologia(10, "giorni"))).toBe("2027-01-06");
  });
});

describe("calcolaDataInizioDaScadenza", () => {
  it("un mensile inizia il giorno dopo lo stesso giorno del mese prima", () => {
    expect(calcolaDataInizioDaScadenza("2026-11-25", tipologia(1, "mesi"))).toBe("2026-10-26");
  });

  it("a fine mese riparte dal primo del mese", () => {
    expect(calcolaDataInizioDaScadenza("2026-02-28", tipologia(1, "mesi"))).toBe("2026-02-01");
    expect(calcolaDataInizioDaScadenza("2026-12-31", tipologia(12, "mesi"))).toBe("2026-01-01");
  });

  it("una durata in giorni torna indietro di durata − 1", () => {
    expect(calcolaDataInizioDaScadenza("2027-01-06", tipologia(10, "giorni"))).toBe("2026-12-28");
  });
});

describe("completaDateAbbonamento", () => {
  const mensile = tipologia(1, "mesi");
  const trimestrale = tipologia(3, "mesi");

  it("dall'inizio calcola la scadenza", () => {
    const date = { inizioIso: "2026-10-26", scadenzaIso: "" };
    expect(completaDateAbbonamento(date, "inizio", mensile).scadenzaIso).toBe("2026-11-25");
  });

  it("dalla scadenza calcola l'inizio", () => {
    const date = { inizioIso: "", scadenzaIso: "2026-11-25" };
    expect(completaDateAbbonamento(date, "scadenza", mensile).inizioIso).toBe("2026-10-26");
  });

  it("cambiando tipologia ricalcola solo la data non scritta a mano", () => {
    const date = { inizioIso: "2026-10-26", scadenzaIso: "2026-11-25" };
    expect(completaDateAbbonamento(date, "inizio", trimestrale)).toEqual({
      inizioIso: "2026-10-26",
      scadenzaIso: "2027-01-25",
    });
    expect(completaDateAbbonamento(date, "scadenza", trimestrale)).toEqual({
      inizioIso: "2026-08-26",
      scadenzaIso: "2026-11-25",
    });
  });

  it("senza una data scritta a mano non cambia nulla", () => {
    const date = { inizioIso: "", scadenzaIso: "" };
    expect(completaDateAbbonamento(date, null, mensile)).toEqual(date);
  });
});

describe("dateDaInviare", () => {
  const date = { inizioIso: "2026-10-26", scadenzaIso: "2026-11-25" };

  it("invia solo la data scritta a mano", () => {
    expect(dateDaInviare(date, "inizio")).toEqual({
      dataInizioAbbonamento: "2026-10-26",
      dataScadenzaAbbonamento: null,
    });
    expect(dateDaInviare(date, "scadenza")).toEqual({
      dataInizioAbbonamento: null,
      dataScadenzaAbbonamento: "2026-11-25",
    });
  });

  it("senza date non invia nulla: il programma chiede di indicarne una", () => {
    expect(dateDaInviare(date, null)).toEqual({
      dataInizioAbbonamento: null,
      dataScadenzaAbbonamento: null,
    });
  });
});

describe("trovaOmonimi", () => {
  const iscritti = [iscritto("Nicolò", "De Rosa"), iscritto("Mario", "Rossi")];

  it("trova lo stesso nome ignorando maiuscole, accenti e spazi", () => {
    expect(trovaOmonimi(iscritti, " nicolo ", "de  rosa")).toEqual([iscritti[0]]);
  });

  it("non considera omonimi nomi solo simili", () => {
    expect(trovaOmonimi(iscritti, "Mario", "Rossini")).toEqual([]);
  });
});
