<script lang="ts">
  import { avvisi, chiudiAvviso } from "$lib/avvisi.svelte";
</script>

{#if avvisi.length > 0}
  <div class="barra">
    {#each avvisi as avviso (avviso.id)}
      <div class="avviso {avviso.tipo}" role="status">
        <span>{avviso.testo}</span>
        <button type="button" onclick={() => chiudiAvviso(avviso.id)} aria-label="Chiudi">×</button>
      </div>
    {/each}
  </div>
{/if}

<style>
  .barra {
    display: grid;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .avviso {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 0.6rem 1rem;
    border-radius: var(--raggio);
    border: 1px solid currentColor;
    background: var(--colore-superficie);
  }

  .successo {
    color: #1b7a3d;
  }

  .avviso.avviso {
    color: var(--colore-avviso);
  }

  .errore {
    color: var(--colore-errore);
  }

  button {
    background: none;
    border: none;
    color: inherit;
    font-size: 1.25rem;
    line-height: 1;
  }
</style>
