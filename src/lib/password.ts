/** Deve coincidere con LUNGHEZZA_MINIMA_PASSWORD in src-tauri/src/db/crittografia.rs */
export const LUNGHEZZA_MINIMA_PASSWORD = 8;

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
