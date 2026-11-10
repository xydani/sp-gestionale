<script lang="ts">
  import { onMount } from "svelte";
  import { alternaSchermoIntero, leggiSchermoIntero } from "$lib/schermo-intero";

  let schermoIntero = $state(false);

  onMount(async () => {
    schermoIntero = await leggiSchermoIntero();
  });

  /** Chiamato anche dalla tastiera (F11 / Esc) per tenere l'icona allineata. */
  export async function aggiornaStato() {
    schermoIntero = await leggiSchermoIntero();
  }

  async function alterna() {
    schermoIntero = await alternaSchermoIntero();
  }

  const etichetta = $derived(
    schermoIntero ? "Esci da schermo intero (F11)" : "Schermo intero (F11)",
  );
</script>

<button type="button" onclick={alterna} title={etichetta} aria-label={etichetta}>
  <svg viewBox="0 0 24 24" width="20" height="20" aria-hidden="true">
    {#if schermoIntero}
      <path d="M9 4v5H4M15 4v5h5M9 20v-5H4M15 20v-5h5" />
    {:else}
      <path d="M4 9V4h5M20 9V4h-5M4 15v5h5M20 15v5h-5" />
    {/if}
  </svg>
</button>

<style>
  button {
    display: grid;
    place-items: center;
    width: 36px;
    height: 36px;
    padding: 0;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    background: var(--colore-superficie);
    color: var(--colore-testo-secondario);
  }

  button:hover {
    color: var(--colore-testo);
    border-color: var(--colore-testo-secondario);
  }

  path {
    fill: none;
    stroke: currentColor;
    stroke-width: 2;
    stroke-linecap: round;
    stroke-linejoin: round;
  }
</style>
