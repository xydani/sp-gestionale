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
  { etichetta: "Report", percorso: "/report" },
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
