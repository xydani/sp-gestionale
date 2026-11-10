import { ask } from "@tauri-apps/plugin-dialog";
import { formattaData, formattaEuro } from "./format";
import type { Pagamento } from "./types";

export function testoConfermaEliminazione(pagamento: Pagamento): string {
  return (
    `Eliminare il pagamento di ${formattaEuro(pagamento.importoCent)} ` +
    `del ${formattaData(pagamento.dataPagamento)} ` +
    `(${pagamento.iscrittoCognome} ${pagamento.iscrittoNome}, ${pagamento.tipologiaNome})? ` +
    "L'operazione non si può annullare."
  );
}

/** Chiede conferma con una finestra di sistema prima di eliminare un pagamento. */
export function confermaEliminazionePagamento(pagamento: Pagamento): Promise<boolean> {
  return ask(testoConfermaEliminazione(pagamento), {
    title: "Elimina pagamento",
    kind: "warning",
    okLabel: "Elimina",
    cancelLabel: "Annulla",
  });
}

export const TESTO_AVVISO_ESPORTAZIONE =
  "Il file CSV non è criptato: chi lo apre vede nomi degli iscritti e importi. " +
  "Salvalo in una cartella protetta, non condividerlo e cancellalo quando non serve più. " +
  "Continuare?";

/** L'archivio è criptato, il CSV no: chi esporta deve saperlo prima. */
export function confermaEsportazioneCsv(): Promise<boolean> {
  return ask(TESTO_AVVISO_ESPORTAZIONE, {
    title: "Esporta dati non criptati",
    kind: "warning",
    okLabel: "Esporta",
    cancelLabel: "Annulla",
  });
}
