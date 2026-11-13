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
  const numero = inFormatoNumerico(testo.replace(/\s|€/g, ""));
  if (!/^\d+(\.\d{1,2})?$/.test(numero)) return null;
  return Math.round(Number(numero) * 100);
}

/**
 * Con la virgola, i punti separano le migliaia ("1.234,56").
 * Senza virgola, un punto seguito da una o due cifre finali è decimale ("30.50", dal tastierino);
 * altrimenti i punti separano le migliaia ("1.000").
 */
function inFormatoNumerico(testo: string): string {
  if (testo.includes(",")) return testo.replace(/\./g, "").replace(",", ".");
  if (/^\d+\.\d{1,2}$/.test(testo)) return testo;
  return testo.replace(/\./g, "");
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

/** "10/11" se la data è dell'anno di `oggi`, altrimenti completa: per le colonne strette. */
export function formattaDataBreve(dataIso: string, oggi: string = oggiIso()): string {
  if (dataIso.slice(0, 4) !== oggi.slice(0, 4)) return formattaData(dataIso);
  const [, mese, giorno] = dataIso.split("-");
  return `${giorno}/${mese}`;
}

/** "05/04 – 04/10/2026": l'anno dell'inizio solo se diverso da quello della fine. */
export function formattaIntervalloBreve(inizioIso: string, fineIso: string): string {
  return `${formattaDataBreve(inizioIso, fineIso)} – ${formattaData(fineIso)}`;
}

/** Data di oggi in formato ISO, secondo il fuso orario locale. */
export function oggiIso(adesso: Date = new Date()): string {
  const mese = String(adesso.getMonth() + 1).padStart(2, "0");
  const giorno = String(adesso.getDate()).padStart(2, "0");
  return `${adesso.getFullYear()}-${mese}-${giorno}`;
}

/** Giorni dalla data `daIso` alla data `aIso` (negativo se `aIso` è prima). */
export function giorniTra(daIso: string, aIso: string): number {
  const millisecondiAlGiorno = 24 * 60 * 60 * 1000;
  return Math.round((Date.parse(aIso) - Date.parse(daIso)) / millisecondiAlGiorno);
}

/** "oggi", "domani", "tra 5 giorni", "ieri", "da 12 giorni". */
export function descriviScadenza(scadenzaIso: string, oggi: string = oggiIso()): string {
  const giorni = giorniTra(oggi, scadenzaIso);
  if (giorni === 0) return "oggi";
  if (giorni === 1) return "domani";
  if (giorni > 1) return `tra ${giorni} giorni`;
  if (giorni === -1) return "ieri";
  return `da ${-giorni} giorni`;
}

/** Nome del mese in italiano di una data ISO: "novembre". */
export function nomeMese(dataIso: string): string {
  const [anno, mese] = dataIso.split("-").map(Number);
  return new Date(anno, mese - 1, 1).toLocaleDateString("it-IT", { month: "long" });
}

/** Variazione percentuale rispetto al valore precedente: "+12%", "−5%", null se non calcolabile. */
export function variazionePercentuale(attuale: number, precedente: number): string | null {
  if (precedente === 0) return null;
  const percentuale = Math.round(((attuale - precedente) / precedente) * 100);
  if (percentuale === 0) return "invariato";
  return percentuale > 0 ? `+${percentuale}%` : `−${Math.abs(percentuale)}%`;
}

/** Chiave di periodo del riepilogo in italiano: "2026", "novembre 2026", "10/11/2026". */
export function formattaPeriodo(chiave: string): string {
  const parti = chiave.split("-");
  if (parti.length === 3) return formattaData(chiave);
  if (parti.length === 2) return `${nomeMese(`${chiave}-01`)} ${parti[0]}`;
  return chiave;
}

/** Quota percentuale arrotondata, "0%" se il totale è zero. */
export function formattaPercentuale(parte: number, totale: number): string {
  if (totale === 0) return "0%";
  return `${Math.round((parte / totale) * 100)}%`;
}

/** Nome del mese dal numero (1 = gennaio). */
export function nomeMeseDaNumero(mese: number): string {
  return nomeMese(`2000-${String(mese).padStart(2, "0")}-01`);
}
