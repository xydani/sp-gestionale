export type PercorsoMenu =
  "/" | "/iscritti" | "/pagamenti" | "/scadenze" | "/report" | "/listino" | "/impostazioni";

export interface VoceMenu {
  etichetta: string;
  percorso: PercorsoMenu;
}

export const VOCI_MENU: VoceMenu[] = [
  { etichetta: "Dashboard", percorso: "/" },
  { etichetta: "Iscritti", percorso: "/iscritti" },
  { etichetta: "Pagamenti", percorso: "/pagamenti" },
  { etichetta: "Scadenze", percorso: "/scadenze" },
  { etichetta: "Incassi", percorso: "/report" },
  { etichetta: "Listino", percorso: "/listino" },
  { etichetta: "Impostazioni", percorso: "/impostazioni" },
];

/** Vero se la voce di menu corrisponde alla pagina aperta (anche alle sottopagine). */
export function eVoceAttiva(voce: VoceMenu, percorsoCorrente: string): boolean {
  if (voce.percorso === "/") {
    return percorsoCorrente === "/";
  }
  return percorsoCorrente === voce.percorso || percorsoCorrente.startsWith(`${voce.percorso}/`);
}

/** Scorciatoia per bloccare al volo il programma quando ci si allontana. */
export const SCORCIATOIA_BLOCCA = "Ctrl+L";

/** Vero se il tasto premuto è Ctrl+L (⌘L su macOS). */
export function eTastoBlocca(evento: Pick<KeyboardEvent, "key" | "ctrlKey" | "metaKey">): boolean {
  const conModificatore = evento.ctrlKey || evento.metaKey;
  return conModificatore && evento.key.toLowerCase() === "l";
}
