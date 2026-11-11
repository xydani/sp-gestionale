<script lang="ts">
  import {
    formattaEuro,
    formattaPercentuale,
    formattaPeriodo,
    variazionePercentuale,
  } from "$lib/format";
  import { tabellaVenditePerMese } from "$lib/statistiche";
  import type { Statistiche } from "$lib/types";
  import BarreOrizzontali from "../BarreOrizzontali.svelte";

  let { vendite }: { vendite: Statistiche["venditePerTipologia"] } = $props();

  const totaleVenduti = $derived(
    vendite.totali.reduce((totale, tipologia) => totale + tipologia.numero, 0),
  );
  const barre = $derived(
    vendite.totali.map((tipologia) => ({
      etichetta: tipologia.tipologiaNome,
      valore: tipologia.numero,
      testoValore: `${tipologia.numero} · ${formattaPercentuale(tipologia.numero, totaleVenduti)}`,
    })),
  );
  const tabella = $derived(tabellaVenditePerMese(vendite.perMese));

  function confronto(attuale: number, precedente: number): string {
    if (precedente === 0) return "—";
    return variazionePercentuale(attuale, precedente) ?? "=";
  }
</script>

<section class="scheda">
  <h2>Vendite per tipologia</h2>
  {#if vendite.totali.length === 0}
    <p class="pochi-dati">Nessun abbonamento venduto nel periodo.</p>
  {:else}
    <p class="spiegazione">Abbonamenti venduti nel periodo e quota sul totale.</p>
    <BarreOrizzontali voci={barre} />

    <table class="tabella riquadrata">
      <thead>
        <tr>
          <th>Tipologia</th>
          <th class="numero">Venduti</th>
          <th class="numero">Incassato</th>
          <th class="numero">Rispetto all'anno prima</th>
        </tr>
      </thead>
      <tbody>
        {#each vendite.totali as tipologia (tipologia.tipologiaNome)}
          <tr>
            <td>{tipologia.tipologiaNome}</td>
            <td class="numero">{tipologia.numero}</td>
            <td class="numero">{formattaEuro(tipologia.importoCent)}</td>
            <td class="numero">{confronto(tipologia.numero, tipologia.numeroAnnoPrecedente)}</td>
          </tr>
        {/each}
      </tbody>
    </table>

    <h3>Mese per mese</h3>
    <div class="scorrevole">
      <table class="tabella riquadrata">
        <thead>
          <tr>
            <th>Mese</th>
            {#each tabella.tipologie as tipologia (tipologia)}<th class="numero">{tipologia}</th
              >{/each}
          </tr>
        </thead>
        <tbody>
          {#each tabella.righe as riga (riga.mese)}
            <tr>
              <td>{formattaPeriodo(riga.mese)}</td>
              {#each tabella.tipologie as tipologia (tipologia)}
                <td class="numero">{riga.numeri[tipologia]}</td>
              {/each}
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</section>

<style>
  h3 {
    margin: 0.5rem 0 0;
    font-size: 1rem;
  }

  .numero {
    text-align: right;
    font-variant-numeric: tabular-nums;
  }

  .scorrevole {
    overflow-x: auto;
  }
</style>
