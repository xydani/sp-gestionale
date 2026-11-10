<script lang="ts">
  import { onMount } from "svelte";
  import {
    elencaTipologie,
    iscriviNuovoIscritto,
    messaggioErrore,
    proponiIscrizione,
    proponiPagamento,
  } from "$lib/api";
  import { centesimiInTestoEuro, euroInCentesimi, formattaEuro, oggiIso } from "$lib/format";
  import { totaleNuovoIscritto } from "$lib/iscrizione";
  import { OPZIONI_METODO } from "$lib/metodi-pagamento";
  import type { DatiIscritto, Iscritto, MetodoPagamento, TipologiaAbbonamento } from "$lib/types";
  import {
    DURATA_ISCRIZIONE_MESI,
    DURATA_MASSIMA_ABBONAMENTO_MESI,
    validita,
    erroreImportoEuro,
    limitiDataPagamento,
    limitiFinePeriodo,
  } from "$lib/validazione";
  import CampoData from "./CampoData.svelte";
  import FormIscritto from "./FormIscritto.svelte";
  import Tendina from "./Tendina.svelte";

  /** Nuovo iscritto: anagrafica, quota annuale e primo abbonamento si salvano insieme. */
  let {
    onIscritto,
    onAnnulla,
  }: {
    onIscritto: (iscritto: Iscritto) => void;
    onAnnulla: () => void;
  } = $props();

  /** L'id viene assegnato dal programma quando l'iscritto è creato. */
  const ID_DA_ASSEGNARE = 0;

  let tipologie = $state<TipologiaAbbonamento[]>([]);
  let quota = $state("");
  let inizioIscrizione = $state("");
  let fineIscrizione = $state("");
  let tipologiaId = $state<number | null>(null);
  let importoAbbonamento = $state("");
  let inizioAbbonamento = $state("");
  let scadenzaAbbonamento = $state("");
  let metodo = $state<MetodoPagamento>("contanti");
  let dataPagamento = $state(oggiIso());
  let errore = $state<string | null>(null);

  const totaleCent = $derived(totaleNuovoIscritto(quota, importoAbbonamento));

  onMount(async () => {
    try {
      tipologie = await elencaTipologie(true);
      const proposta = await proponiIscrizione(null);
      quota = centesimiInTestoEuro(proposta.importoCent);
      inizioIscrizione = proposta.dataInizio;
      fineIscrizione = proposta.dataFine;
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
    }
  });

  /** Prezzo e date proposti per l'abbonamento scelto; con una data di inizio ricalcola la scadenza. */
  async function proponiAbbonamento(dataInizioScelta: string | null = null) {
    if (tipologiaId === null) return;
    try {
      const proposta = await proponiPagamento(null, tipologiaId, dataInizioScelta);
      if (dataInizioScelta === null)
        importoAbbonamento = centesimiInTestoEuro(proposta.importoCent);
      inizioAbbonamento = proposta.dataInizio;
      scadenzaAbbonamento = proposta.dataScadenza;
      errore = null;
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
    }
  }

  async function iscrivi(anagrafica: DatiIscritto): Promise<boolean> {
    const quotaCent = euroInCentesimi(quota);
    const importoCent = euroInCentesimi(importoAbbonamento);
    if (tipologiaId === null) {
      errore = "Scegli l'abbonamento";
      return false;
    }
    if (quotaCent === null || importoCent === null) {
      errore = "Importo non valido (esempio: 45 oppure 45,50)";
      return false;
    }
    try {
      const iscritto = await iscriviNuovoIscritto({
        anagrafica,
        iscrizione: {
          iscrittoId: ID_DA_ASSEGNARE,
          importoCent: quotaCent,
          dataPagamento,
          metodo,
          dataInizio: inizioIscrizione,
          dataFine: fineIscrizione,
          note: null,
        },
        pagamento: {
          iscrittoId: ID_DA_ASSEGNARE,
          tipologiaId,
          importoCent,
          dataPagamento,
          metodo,
          dataInizio: inizioAbbonamento,
          dataScadenza: scadenzaAbbonamento,
          note: null,
          rinnovoDiId: null,
        },
      });
      onIscritto(iscritto);
      return true;
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
      return false;
    }
  }
</script>

<FormIscritto etichettaConferma="Iscrivi e registra il pagamento" onConferma={iscrivi} {onAnnulla}>
  {#snippet sezioniAggiuntive()}
    <fieldset>
      <legend>Iscrizione annuale</legend>
      <div class="griglia">
        <label>
          Quota (€)
          <input
            bind:value={quota}
            inputmode="decimal"
            required
            use:validita={erroreImportoEuro(quota)}
          />
        </label>
        <label>
          Dal
          <CampoData bind:valore={inizioIscrizione} obbligatoria />
        </label>
        <label>
          Al
          <CampoData
            bind:valore={fineIscrizione}
            {...limitiFinePeriodo(inizioIscrizione, DURATA_ISCRIZIONE_MESI)}
            obbligatoria
          />
        </label>
      </div>
    </fieldset>

    <fieldset>
      <legend>Primo abbonamento</legend>
      <div class="griglia">
        <label>
          Abbonamento
          <Tendina
            bind:valore={tipologiaId}
            opzioni={tipologie.map((tipologia) => ({
              valore: tipologia.id,
              etichetta: tipologia.nome,
              dettaglio: formattaEuro(tipologia.prezzoCent),
            }))}
            obbligatoria
            onCambia={() => proponiAbbonamento()}
          />
        </label>
        <label>
          Importo (€)
          <input
            bind:value={importoAbbonamento}
            inputmode="decimal"
            required
            use:validita={erroreImportoEuro(importoAbbonamento)}
          />
        </label>
        <label>
          Inizio
          <CampoData
            bind:valore={inizioAbbonamento}
            obbligatoria
            onCambia={(data) => proponiAbbonamento(data)}
          />
        </label>
        <label>
          Scadenza
          <CampoData
            bind:valore={scadenzaAbbonamento}
            {...limitiFinePeriodo(inizioAbbonamento, DURATA_MASSIMA_ABBONAMENTO_MESI)}
            obbligatoria
          />
        </label>
      </div>
    </fieldset>

    <fieldset>
      <legend>Pagamento</legend>
      <div class="griglia">
        <label>
          Metodo
          <Tendina bind:valore={metodo} opzioni={OPZIONI_METODO} />
        </label>
        <label>
          Data pagamento
          <CampoData bind:valore={dataPagamento} {...limitiDataPagamento()} obbligatoria />
        </label>
        <div class="totale">
          <span>Totale da incassare</span>
          <strong>{totaleCent === null ? "—" : formattaEuro(totaleCent)}</strong>
        </div>
      </div>
    </fieldset>

    {#if errore}<p class="errore" role="alert">{errore}</p>{/if}
  {/snippet}
</FormIscritto>

<style>
  fieldset {
    margin: 0;
    padding: 1rem 0 0;
    border: none;
    border-top: 1px solid var(--colore-bordo);
    display: grid;
    gap: 0.6rem;
  }

  legend {
    padding: 0 0 0.5rem;
    color: var(--colore-testo-secondario);
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .griglia {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  label {
    display: grid;
    gap: 0.35rem;
    font-weight: 500;
    font-size: 0.9rem;
  }

  input {
    padding: 0.55rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
    font-weight: 400;
  }

  .totale {
    display: grid;
    gap: 0.35rem;
    align-content: end;
    font-size: 0.9rem;
  }

  .totale strong {
    font-size: 1.35rem;
    font-variant-numeric: tabular-nums;
  }

  .errore {
    margin: 0;
    color: var(--colore-errore);
  }
</style>
