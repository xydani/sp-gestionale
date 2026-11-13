/** Deve coincidere con LUNGHEZZA_MINIMA_PASSWORD in src-tauri/src/db/crittografia.rs */
export const LUNGHEZZA_MINIMA_PASSWORD = 8;

/** Va detto quando si sceglie la password: dopo, quando la si è persa, è troppo tardi. */
export const AVVISO_PASSWORD_NON_RECUPERABILE =
  "Se perdi la password, i dati non si recuperano in nessun modo: nemmeno i backup " +
  "si aprono senza. Scrivila e conservala in un posto sicuro, lontano dal computer.";

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
  const bit = lunghezzaEfficace(caratteri) * Math.log2(ampiezzaAlfabeto(password));
  const livello = livelloPerBit(bit);
  let suggerimento: string | null = null;
  if (livello < 3) suggerimento = suggerisci(password);
  return { livello, etichetta: ETICHETTE_ROBUSTEZZA[livello], suggerimento };
}

function livelloPerBit(bit: number): LivelloRobustezza {
  if (bit >= SOGLIE_BIT[2]) return 4;
  if (bit >= SOGLIE_BIT[1]) return 3;
  if (bit >= SOGLIE_BIT[0]) return 2;
  return 1;
}

/** Quanti simboli diversi potrebbero stare in ogni posizione, secondo i tipi di carattere usati. */
function ampiezzaAlfabeto(password: string): number {
  let ampiezza = 0;
  if (/[a-z]/.test(password)) ampiezza += 26;
  if (/[A-Z]/.test(password)) ampiezza += 26;
  if (/[0-9]/.test(password)) ampiezza += 10;
  if (/[^a-zA-Z0-9]/.test(password)) ampiezza += 33;
  return ampiezza;
}

/** Lunghezza che conta davvero: ripetizioni, sequenze (abc, 123) e parole comuni valgono poco. */
function lunghezzaEfficace(caratteri: string[]): number {
  let lunghezza = 0;
  let codicePrecedente: number | null = null;
  for (const carattere of caratteri) {
    const codice = carattere.codePointAt(0) ?? 0;
    const prevedibile = codicePrecedente !== null && Math.abs(codice - codicePrecedente) <= 1;
    lunghezza += prevedibile ? PESO_CARATTERE_PREVEDIBILE : 1;
    codicePrecedente = codice;
  }
  for (const parola of paroleComuniContenute(caratteri.join(""))) {
    lunghezza -= parola.length * (1 - PESO_CARATTERE_PREVEDIBILE);
  }
  return Math.max(lunghezza, 1);
}

function paroleComuniContenute(password: string): string[] {
  const minuscola = password.toLowerCase();
  return PAROLE_COMUNI.filter((parola) => minuscola.includes(parola));
}

function suggerisci(password: string): string {
  if (paroleComuniContenute(password).length > 0) {
    return "Evita parole comuni come «password» o il nome della palestra";
  }
  if (!/[^a-zA-Z0-9]/.test(password) || !/[A-Z]/.test(password)) {
    return "Aggiungi maiuscole, numeri o simboli, oppure usa una frase più lunga";
  }
  return "Allunga la password: una frase di più parole è facile da ricordare e difficile da indovinare";
}
