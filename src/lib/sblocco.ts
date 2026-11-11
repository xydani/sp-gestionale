/** Testi e controlli della schermata di sblocco. */

export function motivoBloccoInattivita(minuti: number): string {
  const durata = minuti === 1 ? "1 minuto" : `${minuti} minuti`;
  return `Bloccato dopo ${durata} senza attività: inserisci di nuovo la password.`;
}

/** Vero se il blocco maiuscole è inserito mentre si scrive. */
export function maiuscoleAttive(evento: Pick<KeyboardEvent, "getModifierState">): boolean {
  return evento.getModifierState("CapsLock");
}
