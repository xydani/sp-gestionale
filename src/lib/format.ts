const formatoEuro = new Intl.NumberFormat("it-IT", { style: "currency", currency: "EUR" });

/** Converte un importo in centesimi nel formato "1.234,50 €". */
export function formattaEuro(importoCent: number): string {
  return formatoEuro.format(importoCent / 100);
}

/**
 * Converte un importo scritto dall'utente ("45", "45,5", "1.234,50") in centesimi.
 * Restituisce null se il testo non è un importo valido.
 */
export function euroInCentesimi(testo: string): number | null {
  const pulito = testo.replace(/\s|€/g, "").replace(/\./g, "").replace(",", ".");
  if (!/^\d+(\.\d{1,2})?$/.test(pulito)) return null;
  return Math.round(Number(pulito) * 100);
}

/** Importo in centesimi nel formato da modificare in un campo: "45,50". */
export function centesimiInTestoEuro(importoCent: number): string {
  return (importoCent / 100).toFixed(2).replace(".", ",");
}

/** "1 mese", "3 mesi", "1 giorno", "10 giorni". */
export function descriviDurata(valore: number, unita: "giorni" | "mesi"): string {
  if (unita === "mesi") return `${valore} ${valore === 1 ? "mese" : "mesi"}`;
  return `${valore} ${valore === 1 ? "giorno" : "giorni"}`;
}

/** Data ISO "2026-11-10" nel formato italiano "10/11/2026". */
export function formattaData(dataIso: string): string {
  const [anno, mese, giorno] = dataIso.split("-");
  return `${giorno}/${mese}/${anno}`;
}

/** Data di oggi in formato ISO, secondo il fuso orario locale. */
export function oggiIso(adesso: Date = new Date()): string {
  const mese = String(adesso.getMonth() + 1).padStart(2, "0");
  const giorno = String(adesso.getDate()).padStart(2, "0");
  return `${adesso.getFullYear()}-${mese}-${giorno}`;
}
