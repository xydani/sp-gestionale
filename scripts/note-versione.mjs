// Stampa la sezione del CHANGELOG di una versione: node scripts/note-versione.mjs 0.1.0
import { readFileSync } from "node:fs";
import { estraiNoteVersione } from "./note-versione-lib.mjs";

const versione = process.argv[2];
const note = estraiNoteVersione(readFileSync("CHANGELOG.md", "utf8"), versione);
if (!note) {
  console.error(`Nessuna sezione "## [${versione}]" nel CHANGELOG`);
  process.exit(1);
}
console.log(note);
