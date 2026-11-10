import { formattaData } from "./format";
import type { Iscritto } from "./types";

/** Avviso da mostrare sul certificato medico, oppure null se è valido. */
export function avvisoCertificato(
  iscritto: Pick<Iscritto, "statoCertificato" | "dataScadenzaCertificato">,
): string | null {
  const scadenza = iscritto.dataScadenzaCertificato
    ? formattaData(iscritto.dataScadenzaCertificato)
    : "";
  switch (iscritto.statoCertificato) {
    case "nessuno":
      return "Certificato medico non presentato: va portato.";
    case "scaduto":
      return `Certificato medico scaduto il ${scadenza}: va rinnovato.`;
    case "in_scadenza":
      return `Certificato medico in scadenza il ${scadenza}: ricordarlo all'iscritto.`;
    case "attivo":
      return null;
  }
}
