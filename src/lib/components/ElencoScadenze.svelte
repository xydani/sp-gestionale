<script lang="ts">
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";
  import { elencaIscritti, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { descriviScadenza, formattaData } from "$lib/format";
  import type { Iscritto, OrdinamentoIscritti, Pagina } from "$lib/types";
  import Paginazione from "./Paginazione.svelte";

  /** Elenco paginato degli iscritti con abbonamento o iscrizione in scadenza oppure scaduti. */
  let {
    tipo,
    stato,
  }: {
    tipo: "abbonamento" | "iscrizione";
    stato: "in_scadenza" | "scaduto";
  } = $props();

  let pagina = $state<Pagina<Iscritto>>({
    elementi: [],
    numero: 1,
    elementiPerPagina: 50,
    totaleElementi: 0,
  });

  // In scadenza: le più vicine prima. Scaduti: i più recenti prima.
  const ordinamento = $derived<OrdinamentoIscritti>(
    tipo === "abbonamento"
      ? stato === "in_scadenza"
        ? "scadenza_abbonamento_crescente"
        : "scadenza_abbonamento_decrescente"
      : stato === "in_scadenza"
        ? "fine_iscrizione_crescente"
        : "fine_iscrizione_decrescente",
  );

  function dataScadenza(iscritto: Iscritto): string {
    const data =
      tipo === "abbonamento"
        ? iscritto.ultimaDataScadenzaAbbonamento
        : iscritto.ultimaDataFineIscrizione;
    return data ?? "";
  }

  async function carica(numeroPagina = 1) {
    try {
      pagina = await elencaIscritti(
        {
          testo: "",
          includiArchiviati: false,
          statoAbbonamento: tipo === "abbonamento" ? stato : null,
          statoIscrizione: tipo === "iscrizione" ? stato : null,
          ordinamento,
        },
        numeroPagina,
      );
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }

  onMount(() => carica());
</script>

<table class="tabella riquadrata {stato}">
  <thead>
    <tr><th>Cognome e nome</th><th>Telefono</th><th>Scadenza</th><th></th></tr>
  </thead>
  <tbody>
    {#each pagina.elementi as iscritto (iscritto.id)}
      <tr>
        <td>
          <a href={resolve("/iscritti/[id]", { id: String(iscritto.id) })}>
            {iscritto.cognome}
            {iscritto.nome}
          </a>
        </td>
        <td>{iscritto.telefono ?? "—"}</td>
        <td>{formattaData(dataScadenza(iscritto))}</td>
        <td class="distanza">{descriviScadenza(dataScadenza(iscritto))}</td>
      </tr>
    {:else}
      <tr><td colspan="4" class="vuoto">Nessuno</td></tr>
    {/each}
  </tbody>
</table>
{#if pagina.totaleElementi > pagina.elementiPerPagina}
  <Paginazione {pagina} onCambia={carica} />
{/if}

<style>
  /* Il colore del bordo sinistro indica lo stato */
  table.tabella {
    border-left-width: 4px;
  }

  table.in_scadenza {
    border-left-color: #e0a100;
  }

  table.scaduto {
    border-left-color: #d92d20;
  }

  a {
    color: inherit;
    text-decoration: none;
    font-weight: 500;
  }

  a:hover {
    color: var(--colore-primario);
  }

  .in_scadenza .distanza {
    color: #8a5300;
    font-weight: 500;
  }

  .scaduto .distanza {
    color: #b42318;
    font-weight: 500;
  }

  .vuoto {
    text-align: center;
    color: var(--colore-testo-secondario);
    padding: 1.25rem;
  }
</style>
