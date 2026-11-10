const formatoEuro = new Intl.NumberFormat("it-IT", { style: "currency", currency: "EUR" });

/** Converte un importo in centesimi nel formato "1.234,50 €". */
export function formattaEuro(importoCent: number): string {
  return formatoEuro.format(importoCent / 100);
}
