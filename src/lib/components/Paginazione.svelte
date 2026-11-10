<script lang="ts">
  import { descriviIntervallo, numeroPagine } from "$lib/paginazione";
  import type { Pagina } from "$lib/types";

  let {
    pagina,
    onCambia,
  }: {
    pagina: Pick<Pagina<unknown>, "numero" | "elementiPerPagina" | "totaleElementi">;
    onCambia: (numero: number) => void;
  } = $props();

  const totalePagine = $derived(numeroPagine(pagina));
</script>

<nav class="paginazione" aria-label="Pagine">
  <span class="intervallo">{descriviIntervallo(pagina)}</span>
  {#if totalePagine > 1}
    <div class="comandi">
      <button
        type="button"
        disabled={pagina.numero <= 1}
        onclick={() => onCambia(pagina.numero - 1)}
      >
        ‹ Precedente
      </button>
      <span>Pagina {pagina.numero} di {totalePagine}</span>
      <button
        type="button"
        disabled={pagina.numero >= totalePagine}
        onclick={() => onCambia(pagina.numero + 1)}
      >
        Successiva ›
      </button>
    </div>
  {/if}
</nav>

<style>
  .paginazione {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    gap: 0.75rem;
    margin: 0.75rem 0;
    color: var(--colore-testo-secondario);
    font-size: 0.9rem;
  }

  .comandi {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  button {
    padding: 0.4rem 0.8rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    background: var(--colore-superficie);
    color: var(--colore-testo);
  }

  button:disabled {
    opacity: 0.45;
    cursor: default;
  }
</style>
