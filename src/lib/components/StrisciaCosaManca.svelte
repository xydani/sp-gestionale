<script lang="ts">
  import { cosaManca, type AzioneMancanza } from "$lib/cosa-manca";
  import type { Iscritto } from "$lib/types";

  /** In cima alla scheda: tutto ciò che va sistemato, ognuno porta al punto giusto. */
  let {
    iscritto,
    onAzione,
  }: {
    iscritto: Iscritto;
    onAzione: (azione: AzioneMancanza) => void;
  } = $props();

  const mancanze = $derived(cosaManca(iscritto));
</script>

{#if mancanze.length > 0}
  <div class="striscia" role="status" aria-label="Cosa manca">
    <span class="titolo">Da sistemare</span>
    {#each mancanze as mancanza (mancanza.testo)}
      <button
        type="button"
        class="voce"
        class:urgente={mancanza.urgente}
        onclick={() => onAzione(mancanza.azione)}
      >
        {mancanza.testo} →
      </button>
    {/each}
  </div>
{:else if !iscritto.archiviato}
  <div class="striscia in-regola" role="status">✓ Tutto in regola</div>
{/if}

<style>
  .striscia {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    padding: 0.75rem 1rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio-grande);
    background: var(--colore-superficie);
  }

  .titolo {
    margin-right: 0.25rem;
    color: var(--colore-testo-secondario);
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .voce {
    padding: 0.3rem 0.75rem;
    border: 1px solid transparent;
    border-radius: 999px;
    background: var(--colore-avviso-tenue);
    color: var(--colore-avviso);
    font: inherit;
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
  }

  .voce.urgente {
    background: var(--colore-errore-tenue);
    color: var(--colore-errore);
  }

  .voce:hover {
    border-color: currentColor;
  }

  .in-regola {
    color: var(--colore-successo);
    background: var(--colore-successo-tenue);
    font-weight: 500;
  }
</style>
