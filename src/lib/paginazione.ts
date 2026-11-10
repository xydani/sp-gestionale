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
