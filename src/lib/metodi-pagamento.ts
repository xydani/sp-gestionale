import type { MetodoPagamento } from "./types";

export const ETICHETTE_METODO: Record<MetodoPagamento, string> = {
  contanti: "Contanti",
  carta: "Carta",
  bonifico: "Bonifico",
  altro: "Altro",
};

export const METODI_PAGAMENTO = Object.keys(ETICHETTE_METODO) as MetodoPagamento[];
