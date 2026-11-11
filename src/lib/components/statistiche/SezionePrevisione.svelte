<script lang="ts">
  import { formattaEuro } from "$lib/format";
  import type { PrevisioneFinestra } from "$lib/types";
  import RiquadroNumero from "../RiquadroNumero.svelte";

  let { previsione }: { previsione: PrevisioneFinestra[] } = $props();

  const senzaStorico = $derived(previsione.every((finestra) => finestra.stimaCent === null));
</script>

<section class="scheda">
  <h2>Previsione di incasso</h2>
  <p class="spiegazione">
    Da oggi in avanti, qualunque periodo sia scelto. Abbonamenti e quote che scadono vengono contati
    al prezzo attuale del listino, pesati con la percentuale di rinnovo dell'ultimo anno. È una
    stima, non denaro certo.
  </p>
  {#if senzaStorico}
    <p class="pochi-dati">
      Non c'è ancora storico di rinnovi: si vede solo quanto entrerebbe se rinnovassero tutti.
    </p>
  {/if}
  <div class="riquadri">
    {#each previsione as finestra (finestra.giorni)}
      <RiquadroNumero
        etichetta="Prossimi {finestra.giorni} giorni"
        valore={formattaEuro(finestra.stimaCent ?? finestra.massimoCent)}
      >
        {#snippet dettaglio()}
          {#if finestra.stimaCent !== null && finestra.minimoCent !== null}
            Tra {formattaEuro(finestra.minimoCent)} e {formattaEuro(finestra.massimoCent)}
          {:else if finestra.stimaCent !== null}
            Al massimo {formattaEuro(finestra.massimoCent)}
          {:else}
            Se rinnovassero tutti
          {/if}
        {/snippet}
      </RiquadroNumero>
    {/each}
  </div>
</section>

<style>
  .riquadri {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }
</style>
