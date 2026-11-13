<script lang="ts">
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";
  import { elencaIscritti, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { descriviScadenza, formattaData } from "$lib/format";
  import { seguiFinestra } from "$lib/righe-visibili";
  import { REGOLE_SCADENZA, filtroScadenze, type StatoScadenza } from "$lib/scadenze";
  import type { Iscritto, Pagina, TipoScadenza } from "$lib/types";
  import { eRinnovabile } from "$lib/rinnovo";
  import FinestraRinnovo from "./FinestraRinnovo.svelte";
  import Paginazione from "./Paginazione.svelte";

  /**
   * Elenco degli iscritti con una scadenza in arrivo, passata o mancante. Mostra le righe che
   * stanno nella finestra: a pagine, oppure in anteprima con il collegamento all'elenco completo.
   */
  let {
    tipo,
    stato,
    anteprima = false,
    compatto = false,
    onCambiato,
  }: {
    tipo: TipoScadenza;
    stato: StatoScadenza;
    /** Solo la prima pagina e un collegamento alle Scadenze (es. in dashboard). */
    anteprima?: boolean;
    /** Righe più basse, per stare in poco spazio. */
    compatto?: boolean;
    /** Dopo un rinnovo, per aggiornare il resto della pagina. */
    onCambiato?: () => void;
  } = $props();

  /** Sotto la tabella: pagine o collegamento, più il margine della pagina. */
  const SPAZIO_SOTTO_PX = 80;

  let pagina = $state<Pagina<Iscritto>>({
    elementi: [],
    numero: 1,
    elementiPerPagina: 50,
    totaleElementi: 0,
  });

  /** Iscritto di cui è aperta la finestra di rinnovo. */
  let daRinnovare = $state<number | null>(null);
  let tabella = $state<HTMLTableElement>();
  /** Righe che entrano nella finestra; null finché non sono misurate. */
  let righePerPagina = $state<number | null>(null);
  /** Finché i dati non arrivano non si mostra «Nessuno», che falserebbe la misura delle righe. */
  let caricato = $state(false);

  async function chiudiRinnovo(registrato: boolean) {
    daRinnovare = null;
    if (!registrato) return;
    await carica(pagina.numero);
    onCambiato?.();
  }

  const altreRighe = $derived(anteprima && pagina.totaleElementi > pagina.elementi.length);

  function dataScadenza(iscritto: Iscritto): string {
    return REGOLE_SCADENZA[tipo].dataScadenza(iscritto) ?? "";
  }

  async function carica(numeroPagina = 1) {
    try {
      const righe = righePerPagina ?? undefined;
      pagina = await elencaIscritti(filtroScadenze(tipo, stato), numeroPagina, righe);
      caricato = true;
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }

  onMount(() => {
    if (!tabella) return;
    return seguiFinestra(tabella, SPAZIO_SOTTO_PX, {
      numeroPagina: () => pagina.numero,
      carica: (numero, righe) => {
        righePerPagina = righe;
        carica(numero);
      },
    });
  });
</script>

<table bind:this={tabella} class="tabella riquadrata {stato}" class:compatta={compatto}>
  <thead>
    <tr>
      <th>Cognome e nome</th>
      <th>Telefono</th>
      <th colspan={compatto ? 1 : 2}>Scadenza</th>
      <th></th>
    </tr>
  </thead>
  <tbody>
    {#each pagina.elementi as iscritto (iscritto.id)}
      <tr>
        <td class="elastica">
          <a
            href={resolve("/iscritti/[id]", { id: String(iscritto.id) })}
            title="{iscritto.cognome} {iscritto.nome}"
          >
            {iscritto.cognome}
            {iscritto.nome}
          </a>
        </td>
        <td>{iscritto.telefono ?? "—"}</td>
        {#if !dataScadenza(iscritto)}
          <td colspan={compatto ? 1 : 2} class="distanza">Non presentato</td>
        {:else if compatto}
          <td class="distanza" title={formattaData(dataScadenza(iscritto))}>
            {descriviScadenza(dataScadenza(iscritto))}
          </td>
        {:else}
          <td>{formattaData(dataScadenza(iscritto))}</td>
          <td class="distanza">{descriviScadenza(dataScadenza(iscritto))}</td>
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
      {#if caricato}<tr><td colspan={compatto ? 4 : 5} class="vuoto">Nessuno</td></tr>{/if}
    {/each}
  </tbody>
</table>
{#if altreRighe}
  <a class="tutti" href={resolve("/scadenze")}>Tutti ({pagina.totaleElementi}) →</a>
{:else if !anteprima && pagina.totaleElementi > pagina.elementiPerPagina}
  <Paginazione {pagina} onCambia={carica} />
{/if}

{#if daRinnovare !== null && eRinnovabile(tipo)}
  <FinestraRinnovo iscrittoId={daRinnovare} {tipo} onChiusa={chiudiRinnovo} />
{/if}

<style>
  .tutti {
    display: block;
    margin-top: 0.5rem;
    text-align: right;
    color: var(--colore-primario);
    text-decoration: none;
  }

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
