/**
 * Testo della sezione "## [versione]" del changelog, senza titolo, o null se manca.
 * @param {string} changelog
 * @param {string} versione
 * @returns {string | null}
 */
export function estraiNoteVersione(changelog, versione) {
  const righe = changelog.split(/\r?\n/);
  const inizio = righe.findIndex((riga) => riga.startsWith(`## [${versione}]`));
  if (inizio === -1) return null;
  const fine = righe.findIndex((riga, indice) => indice > inizio && riga.startsWith("## "));
  const sezione = righe
    .slice(inizio + 1, fine === -1 ? undefined : fine)
    .join("\n")
    .trim();
  return sezione || null;
}
