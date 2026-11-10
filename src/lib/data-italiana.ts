/**
 * Date scritte dall'utente nel formato italiano GG/MM/AAAA.
 * Internamente (backend, database) le date restano ISO AAAA-MM-GG.
 */

const FORMATO_ITALIANO = /^(\d{2})\/(\d{2})\/(\d{4})$/;

/** "10/11/2026" → "2026-11-10"; null se il testo non è una data esistente. */
export function testoItalianoInIso(testo: string): string | null {
  const parti = FORMATO_ITALIANO.exec(testo.trim());
  if (!parti) return null;
  const [, giorno, mese, anno] = parti;
  const data = new Date(Number(anno), Number(mese) - 1, Number(giorno));
  const esiste =
    data.getFullYear() === Number(anno) &&
    data.getMonth() === Number(mese) - 1 &&
    data.getDate() === Number(giorno);
  return esiste ? `${anno}-${mese}-${giorno}` : null;
}

/** "2026-11-10" → "10/11/2026"; stringa vuota se manca. */
export function isoInTestoItaliano(dataIso: string | null | undefined): string {
  if (!dataIso) return "";
  const [anno, mese, giorno] = dataIso.split("-");
  return `${giorno}/${mese}/${anno}`;
}

/**
 * Formatta mentre si scrive: tiene solo le cifre e inserisce le barre
 * ("10112026" → "10/11/2026", "101" → "10/1").
 */
export function formattaMentreSiScrive(testo: string): string {
  const cifre = testo.replace(/\D/g, "").slice(0, 8);
  const giorno = cifre.slice(0, 2);
  const mese = cifre.slice(2, 4);
  const anno = cifre.slice(4, 8);
  return [giorno, mese, anno].filter((parte) => parte.length > 0).join("/");
}

/** Messaggio d'errore per una data scritta dall'utente, oppure null se va bene. */
export function erroreData(
  testo: string,
  vincoli: { obbligatoria?: boolean; minimaIso?: string | null; massimaIso?: string | null } = {},
): string | null {
  if (testo.trim() === "") {
    return vincoli.obbligatoria ? "Inserisci la data (GG/MM/AAAA)" : null;
  }
  const dataIso = testoItalianoInIso(testo);
  if (!dataIso) return "Data non valida: usa il formato GG/MM/AAAA";
  if (vincoli.minimaIso && dataIso < vincoli.minimaIso) {
    return `La data non può essere prima del ${isoInTestoItaliano(vincoli.minimaIso)}`;
  }
  if (vincoli.massimaIso && dataIso > vincoli.massimaIso) {
    return `La data non può essere dopo il ${isoInTestoItaliano(vincoli.massimaIso)}`;
  }
  return null;
}
