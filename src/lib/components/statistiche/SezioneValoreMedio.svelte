<script lang="ts">
  import { formattaEuro } from "$lib/format";
  import type { ValoreMedio } from "$lib/types";
  import RiquadroNumero from "../RiquadroNumero.svelte";

  let { valore }: { valore: ValoreMedio } = $props();

  function euroOppureTrattino(cent: number | null): string {
    return cent === null ? "—" : formattaEuro(cent);
  }
</script>

<section class="scheda">
  <h2>Valore medio per iscritto</h2>
  <p class="spiegazione">
    Solo incassi registrati con il programma: abbonamenti e quote pagati prima non contano.
  </p>
  <div class="riquadri">
    <RiquadroNumero
      etichetta="Spesa media nel periodo"
      valore={euroOppureTrattino(valore.valoreMedioCent)}
    >
      {#snippet dettaglio()}
        {formattaEuro(valore.incassiCent)} da {valore.iscrittiPaganti} iscritti che hanno pagato
      {/snippet}
    </RiquadroNumero>
    <RiquadroNumero
      etichetta="Spesa media al mese"
      valore={euroOppureTrattino(valore.spesaMediaMensileCent)}
    >
      {#snippet dettaglio()}
        Per ogni iscritto con abbonamento valido nel mese
      {/snippet}
    </RiquadroNumero>
  </div>
</section>

<style>
  .riquadri {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 1rem;
  }
</style>
