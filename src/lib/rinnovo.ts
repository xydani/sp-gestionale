import type { MetodoPagamento, Pagamento, TipoScadenza, TipologiaAbbonamento } from "./types";

/** Scadenze che si rinnovano con un pagamento; il certificato si aggiorna dalla scheda. */
export type TipoRinnovabile = Extract<TipoScadenza, "abbonamento" | "iscrizione">;

export const METODO_PREDEFINITO: MetodoPagamento = "contanti";

export interface SceltePrecedenti {
  tipologiaId: number | null;
  metodo: MetodoPagamento;
}

export function eRinnovabile(tipo: TipoScadenza): tipo is TipoRinnovabile {
  return tipo === "abbonamento" || tipo === "iscrizione";
}

/**
 * Tipologia e metodo dell'ultimo pagamento, per proporre un rinnovo uguale.
 * La tipologia vale solo se è ancora nel listino; le voci pregresse non hanno un metodo reale.
 */
export function sceltePrecedenti(
  pagamentiRecentiPrima: Pagamento[],
  tipologieAttive: TipologiaAbbonamento[],
): SceltePrecedenti {
  const ultimo = pagamentiRecentiPrima[0];
  const ultimoIncassato = pagamentiRecentiPrima.find((pagamento) => !pagamento.pregresso);
  const tipologiaAncoraAttiva = tipologieAttive.some(
    (tipologia) => tipologia.id === ultimo?.tipologiaId,
  );
  return {
    tipologiaId: tipologiaAncoraAttiva ? ultimo.tipologiaId : null,
    metodo: ultimoIncassato?.metodo ?? METODO_PREDEFINITO,
  };
}
