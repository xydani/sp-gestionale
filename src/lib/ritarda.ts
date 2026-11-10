/**
 * Restituisce una funzione che esegue `azione` solo dopo `millisecondi` di pausa:
 * usata per non cercare a ogni tasto premuto.
 */
export function ritarda<Argomenti extends unknown[]>(
  azione: (...argomenti: Argomenti) => void,
  millisecondi: number,
): (...argomenti: Argomenti) => void {
  let timer: ReturnType<typeof setTimeout> | undefined;
  return (...argomenti) => {
    clearTimeout(timer);
    timer = setTimeout(() => azione(...argomenti), millisecondi);
  };
}
