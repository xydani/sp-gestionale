<script lang="ts">
  import {
    SCELTE_DIMENSIONE_TESTO,
    applicaDimensioneTesto,
    leggiDimensioneTesto,
    salvaDimensioneTesto,
    type DimensioneTesto,
  } from "$lib/dimensione-testo";

  let dimensione = $state(leggiDimensioneTesto());

  function scegli(nuova: DimensioneTesto) {
    dimensione = nuova;
    salvaDimensioneTesto(nuova);
    applicaDimensioneTesto(nuova);
  }
</script>

<section class="scheda">
  <h2>Dimensione del testo</h2>
  <p class="spiegazione">
    Utile su schermi piccoli o per leggere meglio. Vale per questo computer.
  </p>
  <div class="segmenti" role="group" aria-label="Dimensione del testo">
    {#each SCELTE_DIMENSIONE_TESTO as scelta (scelta.valore)}
      <button
        type="button"
        aria-pressed={dimensione === scelta.valore}
        onclick={() => scegli(scelta.valore)}
      >
        {scelta.etichetta}
      </button>
    {/each}
  </div>
</section>

<style>
  .spiegazione {
    margin: 0 0 0.75rem;
    color: var(--colore-testo-secondario);
    font-size: 0.9rem;
  }
</style>
