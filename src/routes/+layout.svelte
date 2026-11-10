<script lang="ts">
  import "../app.css";
  import { onMount, type Snippet } from "svelte";
  import { bloccaDatabase, leggiStatoDatabase } from "$lib/api";
  import { adattaFinestra, modalitaFinestraPerStato } from "$lib/finestra";
  import BarraAvvisi from "$lib/components/BarraAvvisi.svelte";
  import CorniceSenzaBordi from "$lib/components/CorniceSenzaBordi.svelte";
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
    const nuovoStato = await leggiStatoDatabase();
    // Prima si adatta la finestra, poi si cambia schermata: evita di vedere
    // per un istante il layout principale nella finestra compatta
    await adattaFinestra(modalitaFinestraPerStato(nuovoStato));
    statoDatabase = nuovoStato;
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
  <CorniceSenzaBordi><SchermataSblocco onSbloccato={aggiornaStato} /></CorniceSenzaBordi>
{:else if statoDatabase === "da_creare"}
  <CorniceSenzaBordi><PrimoAvvio onCompletato={aggiornaStato} /></CorniceSenzaBordi>
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

  /* Stili condivisi dai riquadri di primo avvio e sblocco */
  :global(.riquadro) {
    display: grid;
    gap: 1rem;
  }

  :global(.riquadro h1) {
    margin: 0;
    font-size: 1.5rem;
  }

  :global(.riquadro p) {
    margin: 0;
    color: var(--colore-testo-secondario);
  }

  :global(.riquadro label) {
    display: grid;
    gap: 0.35rem;
    font-weight: 500;
  }

  :global(.riquadro input) {
    padding: 0.6rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
  }

  :global(.riquadro button) {
    padding: 0.7rem;
    border: none;
    border-radius: var(--raggio);
    background: var(--colore-primario);
    color: var(--colore-primario-testo);
    font-weight: 600;
  }

  :global(.riquadro button:disabled) {
    opacity: 0.6;
  }

  :global(.riquadro .avviso) {
    color: var(--colore-avviso);
  }

  :global(.riquadro .errore) {
    color: var(--colore-errore);
  }
</style>
