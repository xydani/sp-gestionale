<script lang="ts">
  import { valutaRobustezzaPassword } from "$lib/password";

  let { password }: { password: string } = $props();

  const LIVELLI_BARRA = [1, 2, 3, 4] as const;

  const robustezza = $derived(valutaRobustezzaPassword(password));
</script>

{#if password}
  <div class="indicatore livello-{robustezza.livello}" aria-live="polite">
    <div class="barra" aria-hidden="true">
      {#each LIVELLI_BARRA as livello (livello)}
        <span class:pieno={robustezza.livello >= livello}></span>
      {/each}
    </div>
    <p>
      Robustezza: <strong>{robustezza.etichetta}</strong>
      {#if robustezza.suggerimento}<span class="suggerimento">· {robustezza.suggerimento}</span
        >{/if}
    </p>
  </div>
{/if}

<style>
  .indicatore {
    display: grid;
    gap: 0.3rem;
    --colore-livello: var(--colore-bordo-hover);
  }

  .livello-0,
  .livello-1 {
    --colore-livello: var(--colore-errore);
  }

  .livello-2 {
    --colore-livello: var(--colore-avviso);
  }

  .livello-3,
  .livello-4 {
    --colore-livello: var(--colore-successo);
  }

  .barra {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 4px;
  }

  .barra span {
    height: 4px;
    border-radius: 2px;
    background: var(--colore-bordo);
  }

  .barra span.pieno {
    background: var(--colore-livello);
  }

  p {
    margin: 0;
    font-size: 0.85rem;
    color: var(--colore-testo-secondario);
  }

  strong {
    color: var(--colore-livello);
  }
</style>
