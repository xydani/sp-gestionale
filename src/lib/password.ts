/** Deve coincidere con LUNGHEZZA_MINIMA_PASSWORD in src-tauri/src/db/crittografia.rs */
export const LUNGHEZZA_MINIMA_PASSWORD = 12;

/** Controlla la nuova password prima di inviarla: restituisce il messaggio d'errore o null. */
export function validaNuovaPassword(password: string, confermaPassword: string): string | null {
  if ([...password].length < LUNGHEZZA_MINIMA_PASSWORD) {
    return `La password deve avere almeno ${LUNGHEZZA_MINIMA_PASSWORD} caratteri`;
  }
  if (password !== confermaPassword) {
    return "Le due password non coincidono";
  }
  return null;
}

export type LivelloRobustezza = 0 | 1 | 2 | 3 | 4;

export interface Robustezza {
  livello: LivelloRobustezza;
  etichetta: string;
  suggerimento: string | null;
}

/** Parti facili da indovinare: valgono poco anche se allungano la password. */
const PAROLE_COMUNI = [
  "password",
  "qwerty",
  "asdf",
  "123456",
  "abc123",
  "admin",
  "palestra",
  "gestionale",
  "ciao",
  "amore",
  "juventus",
  "napoli",
  "forzainter",
];

/** Bit stimati oltre i quali si passa al livello successivo (debole, discreta, buona, ottima). */
const SOGLIE_BIT = [50, 70, 90] as const;
const PESO_CARATTERE_PREVEDIBILE = 0.25;

const ETICHETTE_ROBUSTEZZA: Record<LivelloRobustezza, string> = {
  0: "Troppo corta",
  1: "Debole",
  2: "Discreta",
  3: "Buona",
  4: "Ottima",
};

/**
 * Stima quanto è difficile indovinare la password: varietà di caratteri per lunghezza,
 * scontando ripetizioni, sequenze (abc, 123) e parole comuni. È un'indicazione, non un blocco.
 */
export function valutaRobustezzaPassword(password: string): Robustezza {
  const caratteri = [...password];
  if (caratteri.length < LUNGHEZZA_MINIMA_PASSWORD) {
    return { livello: 0, etichetta: ETICHETTE_ROBUSTEZZA[0], suggerimento: null };
  }
  const bit = lunghezzaEfficace(caratteri, password) * Math.log2(ampiezzaAlfabeto(password));
  const livello = (1 + SOGLIE_BIT.filter((soglia) => bit >= soglia).length) as LivelloRobustezza;
  return {
    livello,
    etichetta: ETICHETTE_ROBUSTEZZA[livello],
    suggerimento: livello >= 3 ? null : suggerisci(password),
  };
}

function ampiezzaAlfabeto(password: string): number {
  const gruppi: [RegExp, number][] = [
    [/[a-z]/, 26],
    [/[A-Z]/, 26],
    [/[0-9]/, 10],
    [/[^a-zA-Z0-9]/, 33],
  ];
  return gruppi.reduce(
    (totale, [gruppo, ampiezza]) => totale + (gruppo.test(password) ? ampiezza : 0),
    0,
  );
}

function lunghezzaEfficace(caratteri: string[], password: string): number {
  let lunghezza = 0;
  caratteri.forEach((carattere, indice) => {
    const distanza =
      indice > 0 ? carattere.codePointAt(0)! - caratteri[indice - 1].codePointAt(0)! : 2;
    lunghezza += Math.abs(distanza) <= 1 ? PESO_CARATTERE_PREVEDIBILE : 1;
  });
  const minuscola = password.toLowerCase();
  for (const parola of PAROLE_COMUNI) {
    if (minuscola.includes(parola)) lunghezza -= parola.length * (1 - PESO_CARATTERE_PREVEDIBILE);
  }
  return Math.max(lunghezza, 1);
}

function suggerisci(password: string): string {
  const minuscola = password.toLowerCase();
  if (PAROLE_COMUNI.some((parola) => minuscola.includes(parola))) {
    return "Evita parole comuni come «password» o il nome della palestra";
  }
  if (!/[^a-zA-Z0-9]/.test(password) || !/[A-Z]/.test(password)) {
    return "Aggiungi maiuscole, numeri o simboli, oppure usa una frase più lunga";
  }
  return "Allunga la password: una frase di più parole è facile da ricordare e difficile da indovinare";
}
