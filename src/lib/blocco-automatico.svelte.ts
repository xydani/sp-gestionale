/** Blocco dell'archivio dopo un periodo senza usare il programma. */

/** Minuti proposti nelle impostazioni; 0 = mai. */
export const SCELTE_MINUTI_BLOCCO = [0, 5, 10, 15, 30, 60] as const;

/** Azioni che contano come "sto usando il programma". */
export const EVENTI_ATTIVITA = ["pointerdown", "pointermove", "keydown", "wheel"] as const;

/** Il mouse genera molti eventi: il conto alla rovescia si riavvia al massimo una volta al secondo. */
export const INTERVALLO_MINIMO_RIAVVIO_MS = 1_000;

const MILLISECONDI_IN_UN_MINUTO = 60_000;

/** Minuti scelti nelle impostazioni dell'archivio aperto. */
export const bloccoAutomatico = $state({ minuti: 0 });

export function etichettaMinutiBlocco(minuti: number): string {
  if (minuti === 0) return "Mai";
  return minuti === 60 ? "Dopo 1 ora" : `Dopo ${minuti} minuti`;
}

export interface TimerInattivita {
  registraAttivita(): void;
  ferma(): void;
}

/** Chiama `onScaduto` se per `minuti` non si registra attività; con 0 minuti non fa nulla. */
export function avviaTimerInattivita(minuti: number, onScaduto: () => void): TimerInattivita {
  let timer: ReturnType<typeof setTimeout> | undefined;
  let ultimoRiavvio = 0;

  function riavvia() {
    clearTimeout(timer);
    ultimoRiavvio = Date.now();
    timer = setTimeout(onScaduto, minuti * MILLISECONDI_IN_UN_MINUTO);
  }

  if (minuti <= 0) return { registraAttivita: () => {}, ferma: () => {} };
  riavvia();
  return {
    registraAttivita() {
      if (Date.now() - ultimoRiavvio >= INTERVALLO_MINIMO_RIAVVIO_MS) riavvia();
    },
    ferma() {
      clearTimeout(timer);
    },
  };
}
