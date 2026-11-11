/**
 * Icone a tratto su una griglia 24×24, come tracciati SVG. Ogni icona è un solo tracciato
 * (più segmenti separati da M), così si disegna con un unico elemento.
 */
export const ICONE = {
  cerca: "M18 11a7 7 0 1 1-14 0 7 7 0 0 1 14 0zM20 20l-3.5-3.5",
  lucchetto:
    "M7 11h10a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2v-6a2 2 0 0 1 2-2zM8 11V7a4 4 0 0 1 8 0v4",
  calendario:
    "M6 5h12a2.5 2.5 0 0 1 2.5 2.5V18a2.5 2.5 0 0 1-2.5 2.5H6A2.5 2.5 0 0 1 3.5 18V7.5A2.5 2.5 0 0 1 6 5zM3.5 10h17M8 3v4M16 3v4",
  chiudi: "M6 6l12 12M18 6L6 18",
  "freccia-giu": "M6 9l6 6 6-6",
  spunta: "M5 12.5l4.5 4.5L19 7",
  "schermo-intero": "M4 9V4h5M20 9V4h-5M4 15v5h5M20 15v5h-5",
  "esci-schermo-intero": "M9 4v5H4M15 4v5h5M9 20v-5H4M15 20v-5h5",
  occhio: "M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7S2 12 2 12zM15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0z",
  "occhio-barrato":
    "M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7S2 12 2 12zM15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0zM4 4l16 16",
  nota: "M14 3H7a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8zM14 3v5h5M9 13h6M9 17h4",
} as const;

export type NomeIcona = keyof typeof ICONE;
