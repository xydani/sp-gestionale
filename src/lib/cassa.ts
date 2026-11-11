import { METODI_PAGAMENTO } from "./metodi-pagamento";
import type { MetodoPagamento, Movimento } from "./types";

export interface TotaleMetodo {
  metodo: MetodoPagamento;
  totaleCent: number;
  numeroMovimenti: number;
}

/** Totali della cassa per metodo, nell'ordine fisso dei metodi; solo quelli usati. */
export function totaliPerMetodo(movimenti: Movimento[]): TotaleMetodo[] {
  const totali: TotaleMetodo[] = [];
  for (const metodo of METODI_PAGAMENTO) {
    const delMetodo = movimenti.filter((movimento) => movimento.metodo === metodo);
    if (delMetodo.length === 0) continue;
    totali.push({
      metodo,
      totaleCent: sommaImporti(delMetodo),
      numeroMovimenti: delMetodo.length,
    });
  }
  return totali;
}

export function sommaImporti(movimenti: Movimento[]): number {
  return movimenti.reduce((totale, movimento) => totale + movimento.importoCent, 0);
}
