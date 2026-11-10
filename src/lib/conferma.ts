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
