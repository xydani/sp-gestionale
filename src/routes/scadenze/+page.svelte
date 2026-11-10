<script lang="ts">
  import { onMount } from "svelte";
  import ElencoScadenze from "$lib/components/ElencoScadenze.svelte";
  import { aggiornaConteggioScadenze, scadenzeInArrivo } from "$lib/promemoria.svelte";
  import { REGOLE_SCADENZA, TIPI_SCADENZA, statiDaMostrare } from "$lib/scadenze";
  import type { TipoScadenza } from "$lib/types";

  const TITOLI_SEZIONE = {
    in_scadenza: "In scadenza",
    scaduto: "Scaduti",
    nessuno: "Non presentati",
  } as const;

  let schedaAttiva = $state<TipoScadenza>("abbonamento");

  onMount(aggiornaConteggioScadenze);
</script>

<h1>Scadenze</h1>

<div class="schede" role="tablist">
  {#each TIPI_SCADENZA as tipo (tipo)}
    {@const conteggio = scadenzeInArrivo[REGOLE_SCADENZA[tipo].chiaveConteggio]}
    <button
      type="button"
      role="tab"
      aria-selected={schedaAttiva === tipo}
      class:attiva={schedaAttiva === tipo}
      onclick={() => (schedaAttiva = tipo)}
    >
      {REGOLE_SCADENZA[tipo].etichetta}
      {#if conteggio > 0}<span class="conteggio">{conteggio}</span>{/if}
    </button>
  {/each}
</div>

{#key schedaAttiva}
  {#each statiDaMostrare(schedaAttiva) as stato (stato)}
    <section>
      <h2 class={stato}>{TITOLI_SEZIONE[stato]}</h2>
      <ElencoScadenze tipo={schedaAttiva} {stato} />
    </section>
  {/each}
{/key}

<style>
  .schede {
    display: flex;
    gap: 0.25rem;
    border-bottom: 1px solid var(--colore-bordo);
    margin-bottom: 1.5rem;
  }

  .schede button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.6rem 1rem;
    border: none;
    border-bottom: 2px solid transparent;
    margin-bottom: -1px;
    background: none;
    color: var(--colore-testo-secondario);
    font-weight: 500;
  }

  .schede button.attiva {
    color: var(--colore-primario);
    border-bottom-color: var(--colore-primario);
  }

  .conteggio {
    min-width: 1.4rem;
    padding: 0 0.4rem;
    border-radius: 999px;
    background: #fff3d6;
    color: #8a5300;
    font-size: 0.8rem;
    line-height: 1.4rem;
    text-align: center;
  }

  section {
    margin-bottom: 2rem;
  }

  h2 {
    font-size: 1.1rem;
    margin: 0 0 0.75rem;
  }

  h2.in_scadenza {
    color: #8a5300;
  }

  h2.scaduto,
  h2.nessuno {
    color: #b42318;
  }
</style>
