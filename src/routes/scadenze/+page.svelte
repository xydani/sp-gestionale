<script lang="ts">
  import { onMount } from "svelte";
  import ElencoScadenze from "$lib/components/ElencoScadenze.svelte";
  import { aggiornaConteggioScadenze, scadenzeInArrivo } from "$lib/promemoria.svelte";

  type Scheda = "abbonamento" | "iscrizione";

  let schedaAttiva = $state<Scheda>("abbonamento");

  onMount(aggiornaConteggioScadenze);
</script>

<h1>Scadenze</h1>

<div class="schede" role="tablist">
  <button
    type="button"
    role="tab"
    aria-selected={schedaAttiva === "abbonamento"}
    class:attiva={schedaAttiva === "abbonamento"}
    onclick={() => (schedaAttiva = "abbonamento")}
  >
    Abbonamenti
    {#if scadenzeInArrivo.abbonamenti > 0}<span class="conteggio"
        >{scadenzeInArrivo.abbonamenti}</span
      >{/if}
  </button>
  <button
    type="button"
    role="tab"
    aria-selected={schedaAttiva === "iscrizione"}
    class:attiva={schedaAttiva === "iscrizione"}
    onclick={() => (schedaAttiva = "iscrizione")}
  >
    Iscrizioni annuali
    {#if scadenzeInArrivo.iscrizioni > 0}<span class="conteggio">{scadenzeInArrivo.iscrizioni}</span
      >{/if}
  </button>
</div>

{#key schedaAttiva}
  <section>
    <h2 class="in-scadenza">In scadenza</h2>
    <ElencoScadenze tipo={schedaAttiva} stato="in_scadenza" />
  </section>
  <section>
    <h2 class="scaduti">Scaduti</h2>
    <ElencoScadenze tipo={schedaAttiva} stato="scaduto" />
  </section>
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

  h2.in-scadenza {
    color: #8a5300;
  }

  h2.scaduti {
    color: #b42318;
  }
</style>
