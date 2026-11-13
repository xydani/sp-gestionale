/** Le Impostazioni sono divise in schede, così ognuna sta nella finestra senza scorrere. */
export const SCHEDE_IMPOSTAZIONI = [
  { chiave: "palestra", etichetta: "Palestra" },
  { chiave: "aspetto", etichetta: "Aspetto" },
  { chiave: "password", etichetta: "Password" },
  { chiave: "backup", etichetta: "Backup" },
  { chiave: "programma", etichetta: "Programma" },
] as const;

export type SchedaImpostazioni = (typeof SCHEDE_IMPOSTAZIONI)[number]["chiave"];

/** La scheda chiesta nell'indirizzo (es. `?scheda=backup`), oppure la prima. */
export function schedaDaIndirizzo(valore: string | null): SchedaImpostazioni {
  const trovata = SCHEDE_IMPOSTAZIONI.find((scheda) => scheda.chiave === valore);
  return trovata?.chiave ?? "palestra";
}
