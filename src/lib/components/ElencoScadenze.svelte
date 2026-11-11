<script lang="ts">
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";
  import { elencaIscritti, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { descriviScadenza, formattaData } from "$lib/format";
  import { REGOLE_SCADENZA, filtroScadenze, type StatoScadenza } from "$lib/scadenze";
  import type { Iscritto, Pagina, TipoScadenza } from "$lib/types";
  import { eRinnovabile } from "$lib/rinnovo";
  import FinestraRinnovo from "./FinestraRinnovo.svelte";
  import Paginazione from "./Paginazione.svelte";

  /** Elenco paginato degli iscritti con una scadenza in arrivo, passata o mancante. */
  let { tipo, stato }: { tipo: TipoScadenza; stato: StatoScadenza } = $props();

  let pagina = $state<Pagina<Iscritto>>({
    elementi: [],
    numero: 1,
    elementiPerPagina: 50,
    totaleElementi: 0,
  });

  /** Iscritto di cui è aperta la finestra di rinnovo. */
  let daRinnovare = $state<number | null>(null);

  async function chiudiRinnovo(registrato: boolean) {
    daRinnovare = null;
    if (registrato) await carica(pagina.numero);
  }

  function dataScadenza(iscritto: Iscritto): string {
    return REGOLE_SCADENZA[tipo].dataScadenza(iscritto) ?? "";
  }

  async function carica(numeroPagina = 1) {
    try {
      pagina = await elencaIscritti(filtroScadenze(tipo, stato), numeroPagina);
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }

  onMount(() => carica());
</script>

<table class="tabella riquadrata {stato}">
  <thead>
    <tr><th>Cognome e nome</th><th>Telefono</th><th>Scadenza</th><th></th><th></th></tr>
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
        {#if dataScadenza(iscritto)}
          <td>{formattaData(dataScadenza(iscritto))}</td>
          <td class="distanza">{descriviScadenza(dataScadenza(iscritto))}</td>
        {:else}
          <td colspan="2" class="distanza">Non presentato</td>
        {/if}
        <td class="azione">
          {#if eRinnovabile(tipo)}
            <button
              type="button"
              class="pulsante piccolo"
              onclick={() => (daRinnovare = iscritto.id)}
            >
              Rinnova
            </button>
          {:else}
            <a
              class="pulsante piccolo secondario"
              href={resolve("/iscritti/[id]", { id: String(iscritto.id) })}
            >
              Aggiorna
            </a>
          {/if}
        </td>
      </tr>
    {:else}
      <tr><td colspan="5" class="vuoto">Nessuno</td></tr>
    {/each}
  </tbody>
</table>
{#if pagina.totaleElementi > pagina.elementiPerPagina}
  <Paginazione {pagina} onCambia={carica} />
{/if}

{#if daRinnovare !== null && eRinnovabile(tipo)}
  <FinestraRinnovo iscrittoId={daRinnovare} {tipo} onChiusa={chiudiRinnovo} />
{/if}

<style>
  /* Il colore del bordo sinistro indica lo stato */
  table.tabella {
    border-left-width: 4px;
  }

  table.in_scadenza {
    border-left-color: #e0a100;
  }

  table.nessuno,
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

  .azione {
    width: 1%;
    text-align: right;
    white-space: nowrap;
  }

  .azione a {
    color: var(--colore-testo);
  }

  .vuoto {
    text-align: center;
    color: var(--colore-testo-secondario);
    padding: 1.25rem;
  }
</style>
