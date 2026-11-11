<script lang="ts">
  /** Confronto tra poche voci (es. tipologie): una barra per voce, il valore scritto accanto. */
  let {
    voci,
  }: {
    voci: { etichetta: string; valore: number; testoValore: string }[];
  } = $props();

  const massimo = $derived(Math.max(1, ...voci.map((voce) => voce.valore)));
</script>

<ul class="barre">
  {#each voci as voce (voce.etichetta)}
    <li>
      <span class="etichetta">{voce.etichetta}</span>
      <span class="traccia">
        <span class="barra" style:width={`${(voce.valore / massimo) * 100}%`}></span>
      </span>
      <span class="valore">{voce.testoValore}</span>
    </li>
  {/each}
</ul>

<style>
  .barre {
    display: grid;
    gap: 0.5rem;
    margin: 0;
    padding: 0;
    list-style: none;
  }

  li {
    display: grid;
    grid-template-columns: minmax(6rem, 10rem) 1fr auto;
    align-items: center;
    gap: 0.75rem;
  }

  .etichetta {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .traccia {
    height: 0.75rem;
  }

  .barra {
    display: block;
    height: 100%;
    min-width: 1px;
    border-radius: 0 4px 4px 0;
    background: var(--colore-primario);
    opacity: 0.85;
  }

  .valore {
    min-width: 6rem;
    font-variant-numeric: tabular-nums;
    text-align: right;
  }
</style>
