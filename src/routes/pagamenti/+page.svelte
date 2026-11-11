<script lang="ts">
  import Tendina from "$lib/components/Tendina.svelte";
  import CampoData from "$lib/components/CampoData.svelte";
  import { onMount } from "svelte";
  import { elencaPagamenti, eliminaPagamento, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import FormPagamento from "$lib/components/FormPagamento.svelte";
  import Paginazione from "$lib/components/Paginazione.svelte";
  import TabellaPagamenti from "$lib/components/TabellaPagamenti.svelte";
  import { confermaEliminazionePagamento } from "$lib/conferma";
  import { OPZIONI_METODO } from "$lib/metodi-pagamento";
  import { filtriPagamenti } from "$lib/filtri.svelte";
  import type { Pagamento, PaginaPagamenti } from "$lib/types";

  let pagina = $state<PaginaPagamenti>({
    elementi: [],
    numero: 1,
    elementiPerPagina: 50,
    totaleElementi: 0,
    totaleImportoCent: 0,
  });
  let nuovoPagamentoAperto = $state(false);

  /** Carica una pagina; cambiando i filtri si riparte dalla prima. */
  async function carica(numeroPagina = 1) {
    try {
      pagina = await elencaPagamenti(
        {
          dal: filtriPagamenti.dal || null,
          al: filtriPagamenti.al || null,
          metodo: filtriPagamenti.metodo || null,
          iscrittoId: null,
        },
        numeroPagina,
      );
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }

  async function pagamentoRegistrato(pagamento: Pagamento) {
    nuovoPagamentoAperto = false;
    mostraAvviso(
      "successo",
      `Pagamento di ${pagamento.iscrittoNome} ${pagamento.iscrittoCognome} registrato`,
    );
    await carica();
  }

  async function elimina(pagamento: Pagamento) {
    if (!(await confermaEliminazionePagamento(pagamento))) return;
    try {
      await eliminaPagamento(pagamento.id);
      mostraAvviso("successo", "Pagamento eliminato");
      await carica(pagina.numero);
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }

  function tuttoLoStorico() {
    filtriPagamenti.dal = "";
    filtriPagamenti.al = "";
    carica();
  }

  onMount(() => carica());
</script>

<div class="intestazione">
  <h1>Pagamenti</h1>
  {#if !nuovoPagamentoAperto}
    <button type="button" class="pulsante" onclick={() => (nuovoPagamentoAperto = true)}>
      + Registra pagamento
    </button>
  {/if}
</div>

{#if nuovoPagamentoAperto}
  <section class="pannello">
    <h2>Nuovo pagamento</h2>
    <FormPagamento
      onRegistrato={pagamentoRegistrato}
      onAnnulla={() => (nuovoPagamentoAperto = false)}
    />
  </section>
{/if}

<div class="filtri">
  <label
    >Dal <span class="campo-data"
      ><CampoData bind:valore={filtriPagamenti.dal} onCambia={() => carica()} /></span
    ></label
  >
  <label
    >Al <span class="campo-data"
      ><CampoData
        bind:valore={filtriPagamenti.al}
        minimaIso={filtriPagamenti.dal}
        onCambia={() => carica()}
      /></span
    ></label
  >
  <div class="filtro-metodo">
    <Tendina
      bind:valore={filtriPagamenti.metodo}
      opzioni={[{ valore: "", etichetta: "Tutti i metodi" }, ...OPZIONI_METODO]}
      etichettaAccessibile="Metodo di pagamento"
      onCambia={() => carica()}
    />
  </div>
  <button type="button" class="pulsante-testo" onclick={tuttoLoStorico}>Tutto lo storico</button>
</div>

<Paginazione {pagina} onCambia={carica} />
<TabellaPagamenti
  pagamenti={pagina.elementi}
  totaleElementi={pagina.totaleElementi}
  totaleImportoCent={pagina.totaleImportoCent}
  onElimina={elimina}
/>
{#if pagina.totaleElementi > pagina.elementiPerPagina}
  <Paginazione {pagina} onCambia={carica} />
{/if}

<style>
  .filtro-metodo {
    width: 12rem;
  }

  .intestazione {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .pannello {
    display: grid;
    gap: 1rem;
    margin-bottom: 1.5rem;
    padding: 1.5rem;
    background: var(--colore-superficie);
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio-grande);
    box-shadow: var(--ombra-scheda);
  }

  h2 {
    margin: 0;
    font-size: 1.15rem;
  }

  .filtri {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
    margin-bottom: 1rem;
  }

  .filtri label {
    display: flex;
    gap: 0.4rem;
    align-items: center;
  }

  .campo-data {
    width: 10rem;
  }
</style>
