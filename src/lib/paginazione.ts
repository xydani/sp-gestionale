import type { Pagina } from "./types";

export function numeroPagine(
  pagina: Pick<Pagina<unknown>, "totaleElementi" | "elementiPerPagina">,
): number {
  return Math.max(1, Math.ceil(pagina.totaleElementi / pagina.elementiPerPagina));
}

/** "Da 51 a 100 di 250", oppure "Nessun risultato". */
export function descriviIntervallo(
  pagina: Pick<Pagina<unknown>, "numero" | "elementiPerPagina" | "totaleElementi">,
): string {
  if (pagina.totaleElementi === 0) return "Nessun risultato";
  const primo = (pagina.numero - 1) * pagina.elementiPerPagina + 1;
  const ultimo = Math.min(pagina.numero * pagina.elementiPerPagina, pagina.totaleElementi);
  return `Da ${primo} a ${ultimo} di ${pagina.totaleElementi}`;
}

/** Una pagina di un elenco già tutto in memoria; un numero fuori dai limiti va al più vicino. */
export function paginaDiElenco<T>(
  elementi: T[],
  numero: number,
  righePerPagina: number,
): Pagina<T> {
  const ultima = numeroPagine({
    totaleElementi: elementi.length,
    elementiPerPagina: righePerPagina,
  });
  const valido = Math.min(Math.max(numero, 1), ultima);
  const inizio = (valido - 1) * righePerPagina;
  return {
    elementi: elementi.slice(inizio, inizio + righePerPagina),
    numero: valido,
    elementiPerPagina: righePerPagina,
    totaleElementi: elementi.length,
  };
}
