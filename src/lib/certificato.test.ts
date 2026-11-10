import { describe, expect, it } from "vitest";
import { avvisoCertificato } from "./certificato";

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
