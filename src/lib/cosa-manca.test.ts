import { describe, expect, it } from "vitest";
import { cosaManca } from "./cosa-manca";
import type { Iscritto } from "./types";

function iscritto(dati: Partial<Iscritto>): Iscritto {
  return {
    archiviato: false,
    statoIscrizione: "attivo",
    statoAbbonamento: "attivo",
    statoCertificato: "attivo",
    dataScadenzaCertificato: "2027-06-30",
    ultimaDataFineIscrizione: "2027-08-31",
    ultimaDataScadenzaAbbonamento: "2026-12-09",
    daCompletare: false,
    ...dati,
  } as Iscritto;
}

describe("cosaManca", () => {
  it("chi è in regola non ha nulla da sistemare", () => {
    expect(cosaManca(iscritto({}))).toEqual([]);
  });

  it("elenca quota, abbonamento, certificato e anagrafica con l'azione giusta", () => {
    const mancanze = cosaManca(
      iscritto({
        statoIscrizione: "scaduto",
        statoAbbonamento: "in_scadenza",
        statoCertificato: "nessuno",
        dataScadenzaCertificato: null,
        daCompletare: true,
      }),
    );
    expect(mancanze.map((mancanza) => mancanza.azione)).toEqual([
      "iscrizione",
      "abbonamento",
      "certificato",
      "anagrafica",
    ]);
    expect(mancanze[0]).toMatchObject({ testo: "Quota annuale scaduta", urgente: true });
    expect(mancanze[1]).toMatchObject({
      testo: "Abbonamento in scadenza il 09/12/2026",
      urgente: false,
    });
  });

  it("per un archiviato non segnala nulla", () => {
    expect(cosaManca(iscritto({ archiviato: true, statoAbbonamento: "scaduto" }))).toEqual([]);
  });
});
