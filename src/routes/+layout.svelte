<script lang="ts">
  import "../app.css";
  import { onMount, type Snippet } from "svelte";
  import { bloccaDatabase, leggiStatoDatabase } from "$lib/api";
  import BarraAvvisi from "$lib/components/BarraAvvisi.svelte";
  import MenuLaterale from "$lib/components/MenuLaterale.svelte";
  import PrimoAvvio from "$lib/components/PrimoAvvio.svelte";
  import PulsanteSchermoIntero from "$lib/components/PulsanteSchermoIntero.svelte";
  import SchermataSblocco from "$lib/components/SchermataSblocco.svelte";
  import {
    alternaSchermoIntero,
    eTastoSchermoIntero,
    esciDaSchermoIntero,
  } from "$lib/schermo-intero";
  import type { StatoDatabase } from "$lib/types";

  let { children }: { children: Snippet } = $props();

  let statoDatabase = $state<StatoDatabase | null>(null);
  let pulsanteSchermoIntero = $state<PulsanteSchermoIntero>();

  async function aggiornaStato() {
    statoDatabase = await leggiStatoDatabase();
  }

  async function blocca() {
    await bloccaDatabase();
    await aggiornaStato();
  }

  async function gestisciTasti(evento: KeyboardEvent) {
    if (eTastoSchermoIntero(evento)) {
      evento.preventDefault();
      await alternaSchermoIntero();
    } else if (evento.key === "Escape") {
      await esciDaSchermoIntero();
    } else {
      return;
    }
    await pulsanteSchermoIntero?.aggiornaStato();
  }

  onMount(aggiornaStato);
</script>

<svelte:window onkeydown={gestisciTasti} />

{#if statoDatabase === "sbloccato"}
  <div class="app">
    <MenuLaterale onBlocca={blocca} />
    <main>
      <div class="barra-superiore">
        <PulsanteSchermoIntero bind:this={pulsanteSchermoIntero} />
      </div>
      <BarraAvvisi />
      {@render children()}
    </main>
  </div>
{:else if statoDatabase === "bloccato"}
  <div class="centrato"><SchermataSblocco onSbloccato={aggiornaStato} /></div>
{:else if statoDatabase === "da_creare"}
  <div class="centrato"><PrimoAvvio onCompletato={aggiornaStato} /></div>
{/if}

<style>
  .app {
    display: flex;
    min-height: 100vh;
  }

  main {
    flex: 1;
    min-width: 0;
    padding: 1rem 2rem 1.5rem;
  }

  .barra-superiore {
    display: flex;
    justify-content: flex-end;
    margin-bottom: 0.5rem;
  }

  .centrato {
    min-height: 100vh;
    display: grid;
    place-items: center;
    padding: 1rem;
  }

  /* Stili condivisi dai riquadri di primo avvio e sblocco */
  .centrato :global(.riquadro) {
    width: min(420px, 100%);
    display: grid;
    gap: 1rem;
    background: var(--colore-superficie);
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    padding: 2rem;
  }

  .centrato :global(h1) {
    margin: 0;
    font-size: 1.5rem;
  }

  .centrato :global(p) {
    margin: 0;
    color: var(--colore-testo-secondario);
  }

  .centrato :global(label) {
    display: grid;
    gap: 0.35rem;
    font-weight: 500;
  }

  .centrato :global(input) {
    padding: 0.6rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
  }

  .centrato :global(button) {
    padding: 0.7rem;
    border: none;
    border-radius: var(--raggio);
    background: var(--colore-primario);
    color: var(--colore-primario-testo);
    font-weight: 600;
  }

  .centrato :global(button:disabled) {
    opacity: 0.6;
  }

  .centrato :global(.avviso) {
    color: var(--colore-avviso);
  }

  .centrato :global(.errore) {
    color: var(--colore-errore);
  }
</style>
