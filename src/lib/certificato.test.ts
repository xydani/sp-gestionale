import { describe, expect, it } from "vitest";
import { avvisoCertificato, datiConCertificato } from "./certificato";
import type { Iscritto } from "./types";

describe("avvisoCertificato", () => {
  it("avvisa se manca, è scaduto o sta per scadere", () => {
    expect(avvisoCertificato({ statoCertificato: "nessuno", dataScadenzaCertificato: null })).toBe(
      "Certificato medico non presentato: va portato.",
    );
    expect(
      avvisoCertificato({ statoCertificato: "scaduto", dataScadenzaCertificato: "2026-10-31" }),
    ).toBe("Certificato medico scaduto il 31/10/2026: va rinnovato.");
    expect(
      avvisoCertificato({ statoCertificato: "in_scadenza", dataScadenzaCertificato: "2026-11-14" }),
    ).toContain("in scadenza il 14/11/2026");
  });

  it("non avvisa se è valido", () => {
    expect(
      avvisoCertificato({ statoCertificato: "attivo", dataScadenzaCertificato: "2027-06-01" }),
    ).toBeNull();
  });
});

describe("datiConCertificato", () => {
  const iscritto = {
    id: 7,
    nome: "Lucia",
    cognome: "Ferraresi",
    dataNascita: "1990-04-12",
    luogoNascita: "Lodi",
    telefono: "333 0000000",
    email: null,
    note: "Preferisce la sera",
    certificatoPresentato: false,
    dataScadenzaCertificato: null,
  } as Iscritto;

  it("cambia solo il certificato e lascia l'anagrafica com'è", () => {
    expect(
      datiConCertificato(iscritto, {
        certificatoPresentato: true,
        dataScadenzaCertificato: "2027-11-30",
      }),
    ).toEqual({
      nome: "Lucia",
      cognome: "Ferraresi",
      dataNascita: "1990-04-12",
      luogoNascita: "Lodi",
      telefono: "333 0000000",
      email: null,
      note: "Preferisce la sera",
      certificatoPresentato: true,
      dataScadenzaCertificato: "2027-11-30",
    });
  });

  it("senza scadenza o senza certificato non manda alcuna data", () => {
    expect(
      datiConCertificato(iscritto, { certificatoPresentato: true, dataScadenzaCertificato: "" })
        .dataScadenzaCertificato,
    ).toBeNull();
    expect(
      datiConCertificato(iscritto, {
        certificatoPresentato: false,
        dataScadenzaCertificato: "2027-11-30",
      }).dataScadenzaCertificato,
    ).toBeNull();
  });
});
