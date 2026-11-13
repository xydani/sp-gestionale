<script lang="ts">
  import {
    formattaEuro,
    formattaPercentuale,
    formattaPeriodo,
    variazionePercentuale,
  } from "$lib/format";
  import { paginaDiElenco } from "$lib/paginazione";
  import { RIGHE_MINIME, seguiFinestra } from "$lib/righe-visibili";
  import { tabellaVenditePerMese } from "$lib/statistiche";
  import type { Statistiche } from "$lib/types";
  import BarreOrizzontali from "../BarreOrizzontali.svelte";
  import Paginazione from "../Paginazione.svelte";

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

  /** Sotto l'elenco dei mesi: le pagine, il bordo della scheda e il margine della pagina. */
  const SPAZIO_SOTTO_PX = 100;
  let tabellaMesi = $state<HTMLTableElement>();
  let numeroPaginaMesi = $state(1);
  let righeMesi = $state(RIGHE_MINIME);
  const paginaMesi = $derived(paginaDiElenco(tabella.righe, numeroPaginaMesi, righeMesi));

  $effect(() => {
    if (!tabellaMesi) return;
    return seguiFinestra(tabellaMesi, SPAZIO_SOTTO_PX, {
      numeroPagina: () => numeroPaginaMesi,
      carica: (numero, righe) => {
        righeMesi = righe;
        numeroPaginaMesi = numero;
      },
    });
  });

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
    <!-- Totali a sinistra, mese per mese a destra -->
    <div class="colonne">
      <div class="colonna">
        <p class="spiegazione">Abbonamenti venduti nel periodo e quota sul totale.</p>
        <BarreOrizzontali voci={barre} />

        <table class="tabella riquadrata compatta">
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
                <td class="numero">{confronto(tipologia.numero, tipologia.numeroAnnoPrecedente)}</td
                >
              </tr>
            {/each}
          </tbody>
        </table>
      </div>

      <div class="colonna">
        <h3>Mese per mese</h3>
        <div class="scorrevole">
          <table bind:this={tabellaMesi} class="tabella riquadrata compatta">
            <thead>
              <tr>
                <th>Mese</th>
                {#each tabella.tipologie as tipologia (tipologia)}<th class="numero">{tipologia}</th
                  >{/each}
              </tr>
            </thead>
            <tbody>
              {#each paginaMesi.elementi as riga (riga.mese)}
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
        {#if paginaMesi.totaleElementi > paginaMesi.elementiPerPagina}
          <Paginazione pagina={paginaMesi} onCambia={(numero) => (numeroPaginaMesi = numero)} />
        {/if}
      </div>
    </div>
  {/if}
</section>

<style>
  h3 {
    margin: 0;
    font-size: 1rem;
  }

  .colonne {
    display: grid;
    gap: 1.25rem;
    align-items: start;
  }

  @media (min-width: 1100px) {
    .colonne {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  .colonna {
    display: grid;
    gap: 0.75rem;
    min-width: 0;
  }

  .numero {
    text-align: right;
    font-variant-numeric: tabular-nums;
  }

  .scorrevole {
    overflow-x: auto;
  }
</style>
