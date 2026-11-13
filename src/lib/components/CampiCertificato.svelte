<script lang="ts">
  import { limitiScadenzaCertificato } from "$lib/validazione";
  import CampoData from "./CampoData.svelte";

  let {
    presentato = $bindable(),
    scadenza = $bindable(),
  }: {
    presentato: boolean;
    /** Vuota se non indicata. */
    scadenza: string;
  } = $props();

  const limiti = limitiScadenzaCertificato();
</script>

<div class="campi">
  <label class="spunta">
    <input type="checkbox" bind:checked={presentato} />
    Certificato presentato
  </label>
  {#if presentato}
    <label>
      Scade il (facoltativo)
      <CampoData bind:valore={scadenza} {...limiti} />
    </label>
  {:else}
    <p class="avviso">Da portare: finché manca, l'iscritto compare tra le scadenze.</p>
  {/if}
</div>

<style>
  .campi {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    align-items: end;
    gap: 1rem;
  }

  label {
    display: grid;
    min-width: 0;
    gap: 0.35rem;
    font-weight: 500;
    font-size: 0.9rem;
  }

  label.spunta {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-height: 2.5rem;
    white-space: nowrap;
  }

  label.spunta input {
    width: 1.1rem;
    height: 1.1rem;
    padding: 0;
  }

  .avviso {
    margin: 0;
    align-self: center;
    color: var(--colore-avviso);
    font-size: 0.9rem;
  }
</style>
