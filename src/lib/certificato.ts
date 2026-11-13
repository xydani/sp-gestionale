import { formattaData } from "./format";
import type { DatiIscritto, Iscritto } from "./types";

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

export interface DatiCertificato {
  certificatoPresentato: boolean;
  /** Vuota se non indicata. */
  dataScadenzaCertificato: string;
}

/** I dati da salvare cambiando solo il certificato: l'anagrafica resta com'è. */
export function datiConCertificato(iscritto: Iscritto, certificato: DatiCertificato): DatiIscritto {
  return {
    nome: iscritto.nome,
    cognome: iscritto.cognome,
    dataNascita: iscritto.dataNascita,
    luogoNascita: iscritto.luogoNascita,
    telefono: iscritto.telefono,
    email: iscritto.email,
    note: iscritto.note,
    certificatoPresentato: certificato.certificatoPresentato,
    dataScadenzaCertificato: certificato.certificatoPresentato
      ? certificato.dataScadenzaCertificato || null
      : null,
  };
}
