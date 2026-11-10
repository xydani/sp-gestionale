<script lang="ts">
  import type { Snippet } from "svelte";

  /** Riquadro con un numero principale, un'etichetta e un dettaglio facoltativo. */
  let {
    etichetta,
    valore,
    tono = "neutro",
    href,
    dettaglio,
  }: {
    etichetta: string;
    valore: string;
    tono?: "neutro" | "attivo" | "in_scadenza" | "scaduto";
    href?: string;
    dettaglio?: Snippet;
  } = $props();
</script>

<svelte:element this={href ? "a" : "div"} {href} class="riquadro {tono}">
  <span class="etichetta">
    {#if tono !== "neutro"}<span class="indicatore" aria-hidden="true"></span>{/if}
    {etichetta}
  </span>
  <span class="valore">{valore}</span>
  {#if dettaglio}<span class="dettaglio">{@render dettaglio()}</span>{/if}
</svelte:element>

<style>
  .riquadro {
    display: grid;
    gap: 0.35rem;
    align-content: start;
    padding: 1.1rem 1.25rem;
    background: var(--colore-superficie);
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    color: inherit;
    text-decoration: none;
  }

  a.riquadro:hover {
    border-color: var(--colore-primario);
  }

  .etichetta {
    display: flex;
    align-items: center;
    gap: 0.45rem;
    color: var(--colore-testo-secondario);
    font-size: 0.9rem;
  }

  .indicatore {
    width: 0.6rem;
    height: 0.6rem;
    border-radius: 50%;
  }

  .attivo .indicatore {
    background: #1b7a3d;
  }

  .in_scadenza .indicatore {
    background: #e0a100;
  }

  .scaduto .indicatore {
    background: #d92d20;
  }

  .valore {
    font-size: 2rem;
    font-weight: 650;
    line-height: 1.1;
    font-variant-numeric: tabular-nums;
  }

  .dettaglio {
    color: var(--colore-testo-secondario);
    font-size: 0.85rem;
  }
</style>
