<script lang="ts">
  import type { Snippet } from "svelte";
  import { chiudiApp } from "$lib/finestra";

  let { children }: { children: Snippet } = $props();
</script>

<!-- La finestra non ha barra del titolo: la fascia in alto serve per trascinarla -->
<div class="cornice">
  <div class="barra-trascinamento" data-tauri-drag-region>
    <button type="button" onclick={chiudiApp} aria-label="Chiudi" title="Chiudi">×</button>
  </div>
  <div class="contenuto">{@render children()}</div>
</div>

<style>
  .cornice {
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: var(--colore-superficie);
    overflow: hidden;
  }

  .barra-trascinamento {
    height: 36px;
    flex-shrink: 0;
    display: flex;
    justify-content: flex-end;
    align-items: center;
    padding: 0 0.5rem;
    user-select: none;
  }

  button {
    width: 28px;
    height: 28px;
    border: none;
    border-radius: var(--raggio);
    background: transparent;
    color: var(--colore-testo-secondario);
    font-size: 1.3rem;
    line-height: 1;
  }

  button:hover {
    background: var(--colore-bordo);
    color: var(--colore-testo);
  }

  .contenuto {
    flex: 1;
    padding: 0 2rem 2rem;
    overflow-y: auto;
  }
</style>
