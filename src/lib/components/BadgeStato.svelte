<script lang="ts">
  import { ETICHETTE_STATO } from "$lib/stato-validita";
  import type { StatoValidita } from "$lib/types";

  let { stato }: { stato: StatoValidita } = $props();
</script>

<span class="badge {stato}">{ETICHETTE_STATO[stato]}</span>

<style>
  .badge {
    display: inline-block;
    padding: 0.15rem 0.6rem;
    border-radius: 999px;
    font-size: 0.85rem;
    font-weight: 500;
    white-space: nowrap;
  }

  .attivo {
    background: #e3f5e9;
    color: #1b7a3d;
  }

  .in_scadenza {
    background: #fff3d6;
    color: #8a5300;
  }

  .scaduto {
    background: #fde4e4;
    color: #b42318;
  }

  .nessuno {
    background: #eceef2;
    color: var(--colore-testo-secondario);
  }
</style>
