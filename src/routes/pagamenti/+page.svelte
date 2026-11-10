<script lang="ts">
  import { onMount } from "svelte";
  import { elencaPagamenti, eliminaPagamento, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import FormPagamento from "$lib/components/FormPagamento.svelte";
  import TabellaPagamenti from "$lib/components/TabellaPagamenti.svelte";
  import { confermaEliminazionePagamento } from "$lib/conferma";
  import { ETICHETTE_METODO, METODI_PAGAMENTO } from "$lib/metodi-pagamento";
  import { meseCorrente } from "$lib/periodo";
  import type { MetodoPagamento, Pagamento } from "$lib/types";

  const periodoIniziale = meseCorrente();

  let pagamenti = $state<Pagamento[]>([]);
  let dal = $state(periodoIniziale.dal);
  let al = $state(periodoIniziale.al);
  let metodo = $state<MetodoPagamento | "">("");
  let nuovoPagamentoAperto = $state(false);

  async function carica() {
    try {
      pagamenti = await elencaPagamenti({
        dal: dal || null,
        al: al || null,
        metodo: metodo || null,
        iscrittoId: null,
      });
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
      await carica();
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }

  function tuttoLoStorico() {
    dal = "";
    al = "";
    carica();
  }

  onMount(carica);
</script>

<div class="intestazione">
  <h1>Pagamenti</h1>
  {#if !nuovoPagamentoAperto}
    <button type="button" class="primario" onclick={() => (nuovoPagamentoAperto = true)}>
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
  <label>Dal <input type="date" bind:value={dal} onchange={carica} /></label>
  <label>Al <input type="date" bind:value={al} min={dal} onchange={carica} /></label>
  <select bind:value={metodo} onchange={carica}>
    <option value="">Tutti i metodi</option>
    {#each METODI_PAGAMENTO as valore (valore)}
      <option value={valore}>{ETICHETTE_METODO[valore]}</option>
    {/each}
  </select>
  <button type="button" class="collegamento" onclick={tuttoLoStorico}>Tutto lo storico</button>
</div>

<TabellaPagamenti {pagamenti} onElimina={elimina} />

<style>
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
    border-radius: var(--raggio);
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

  input,
  select {
    padding: 0.5rem 0.7rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
    background: var(--colore-superficie);
  }

  .collegamento {
    border: none;
    background: none;
    color: var(--colore-primario);
  }

  .primario {
    padding: 0.55rem 1.1rem;
    border-radius: var(--raggio);
    border: none;
    background: var(--colore-primario);
    color: var(--colore-primario-testo);
    font-weight: 500;
  }
</style>
