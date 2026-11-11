<script lang="ts">
  import Icona from "./Icona.svelte";
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
  <Icona nome={schermoIntero ? "esci-schermo-intero" : "schermo-intero"} dimensione={20} />
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
</style>
