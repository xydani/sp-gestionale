<script lang="ts">
  import { formattaPeriodo } from "$lib/format";
  import { meseBreve } from "$lib/statistiche";
  import type { AttiviMese } from "$lib/types";
  import GraficoColonne from "../GraficoColonne.svelte";

  let { attivi }: { attivi: AttiviMese[] } = $props();

  const massimo = $derived(Math.max(1, ...attivi.map((mese) => mese.attivi)));
  const colonne = $derived(
    attivi.map((mese) => ({
      etichetta: meseBreve(mese.mese),
      valore: mese.attivi,
      descrizione: `${formattaPeriodo(mese.mese)}: ${mese.attivi} iscritti attivi`,
    })),
  );
</script>

<section class="scheda">
  <h2>Iscritti attivi nel tempo</h2>
  <p class="spiegazione">
    Iscritti con un abbonamento valido l'ultimo giorno di ogni mese (oggi per il mese in corso).
  </p>
  {#if attivi.length === 0}
    <p class="pochi-dati">Nessun mese da mostrare nel periodo.</p>
  {:else}
    <GraficoColonne
      voci={colonne}
      {massimo}
      etichettaMassimo={String(massimo)}
      titoloAccessibile="Iscritti attivi a fine mese"
    />
  {/if}
</section>
