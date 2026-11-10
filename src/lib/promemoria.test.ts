import { describe, expect, it, vi } from "vitest";

vi.mock("@tauri-apps/plugin-notification", () => ({}));
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const { testoNotifica, totaleInScadenza } = await import("./promemoria.svelte");

describe("testoNotifica", () => {
  it("non notifica se non ci sono nuove scadenze", () => {
    expect(testoNotifica({ abbonamenti: 0, iscrizioni: 0, certificati: 0 })).toBeNull();
  });

  it("riassume abbonamenti e iscrizioni con singolare e plurale", () => {
    expect(testoNotifica({ abbonamenti: 3, iscrizioni: 1, certificati: 0 })).toBe(
      'In scadenza: 3 abbonamenti e 1 iscrizione. Apri "Scadenze" per i dettagli.',
    );
    expect(testoNotifica({ abbonamenti: 1, iscrizioni: 0, certificati: 0 })).toBe(
      'In scadenza: 1 abbonamento. Apri "Scadenze" per i dettagli.',
    );
    expect(testoNotifica({ abbonamenti: 0, iscrizioni: 2, certificati: 0 }, 7)).toBe(
      'In scadenza nei prossimi 7 giorni: 2 iscrizioni. Apri "Scadenze" per i dettagli.',
    );
  });
});

describe("certificati medici", () => {
  it("compaiono nella notifica con gli altri", () => {
    expect(testoNotifica({ abbonamenti: 2, iscrizioni: 1, certificati: 1 })).toBe(
      'In scadenza: 2 abbonamenti, 1 iscrizione e 1 certificato medico. Apri "Scadenze" per i dettagli.',
    );
    expect(testoNotifica({ abbonamenti: 0, iscrizioni: 0, certificati: 3 })).toBe(
      'In scadenza: 3 certificati medici. Apri "Scadenze" per i dettagli.',
    );
  });

  it("contano nel totale del badge", () => {
    expect(totaleInScadenza({ abbonamenti: 2, iscrizioni: 1, certificati: 4 })).toBe(7);
  });
});
