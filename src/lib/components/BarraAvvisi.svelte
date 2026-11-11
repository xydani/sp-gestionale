<script lang="ts">
  import { avvisi, chiudiAvviso, type TipoAvviso } from "$lib/avvisi.svelte";

  const SIMBOLI: Record<TipoAvviso, string> = { successo: "✓", avviso: "!", errore: "×" };
</script>

<!-- Fissi in basso a destra: si vedono anche a pagina scorsa e non spostano il contenuto -->
<div class="barra" aria-live="polite">
  {#each avvisi as avviso (avviso.id)}
    <div class="avviso {avviso.tipo}" role={avviso.tipo === "errore" ? "alert" : "status"}>
      <span class="simbolo" aria-hidden="true">{SIMBOLI[avviso.tipo]}</span>
      <span class="testo">{avviso.testo}</span>
      {#if avviso.azione}
        <button type="button" class="azione" onclick={avviso.azione.esegui}>
          {avviso.azione.etichetta}
        </button>
      {/if}
      <button type="button" onclick={() => chiudiAvviso(avviso.id)} aria-label="Chiudi">×</button>
    </div>
  {/each}
</div>

<style>
  .barra {
    position: fixed;
    right: 1.5rem;
    bottom: 1.5rem;
    z-index: 50;
    display: grid;
    gap: 0.5rem;
    width: min(26rem, calc(100vw - 3rem));
  }

  .avviso {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.7rem 0.9rem;
    border-radius: var(--raggio-grande);
    border: 1px solid var(--colore-tipo);
    color: var(--colore-tipo);
    box-shadow: var(--ombra-sospesa);
    animation: entra 0.18s ease-out;
  }

  .successo {
    --colore-tipo: var(--colore-successo);
    background: var(--colore-successo-tenue);
  }

  .avviso.avviso {
    --colore-tipo: var(--colore-avviso);
    background: var(--colore-avviso-tenue);
  }

  .errore {
    --colore-tipo: var(--colore-errore);
    background: var(--colore-errore-tenue);
  }

  .simbolo {
    display: grid;
    place-items: center;
    flex: none;
    width: 1.4rem;
    height: 1.4rem;
    border-radius: 50%;
    background: var(--colore-tipo);
    color: #fff;
    font-weight: 700;
    font-size: 0.85rem;
  }

  .testo {
    flex: 1;
    color: var(--colore-testo);
  }

  button {
    background: none;
    border: none;
    color: inherit;
    font-size: 1.25rem;
    line-height: 1;
  }

  .azione {
    padding: 0.25rem 0.6rem;
    border: 1px solid currentColor;
    border-radius: var(--raggio);
    font-size: 0.9rem;
    font-weight: 600;
  }

  @keyframes entra {
    from {
      opacity: 0;
      transform: translateY(0.5rem);
    }
  }
</style>
