<script lang="ts">
  import {
    numeroIscritti,
    numeroNellaScheda,
    schedeColonna,
    type ColonnaScadenze,
    type StatoScadenza,
  } from "$lib/scadenze";
  import type { RiepilogoDashboard, TipoScadenza } from "$lib/types";
  import ElencoScadenze from "./ElencoScadenze.svelte";

  /** Una colonna della pagina Scadenze: schede per tipo e l'elenco della scheda scelta. */
  let {
    colonna,
    titolo,
    nota,
    riepilogo,
    onCambiato,
  }: {
    colonna: ColonnaScadenze;
    titolo: string;
    nota: string;
    /** Conteggi per i numeri sulle schede; null finché non sono letti. */
    riepilogo: RiepilogoDashboard | null;
    /** Dopo un rinnovo, per aggiornare i conteggi. */
    onCambiato: () => void;
  } = $props();

  const schede = $derived(schedeColonna(colonna));
  let tipoScelto = $state<TipoScadenza>("abbonamento");
  const schedaScelta = $derived(schede.find((scheda) => scheda.tipo === tipoScelto) ?? schede[0]);

  /** Scelta interna quando la scheda ha più stati (certificati scaduti e mai presentati). */
  let statoScelto = $state<StatoScadenza | null>(null);
  const statoMostrato = $derived(
    statoScelto && schedaScelta.stati.includes(statoScelto) ? statoScelto : schedaScelta.stati[0],
  );
  const ETICHETTE_STATO: Record<StatoScadenza, string> = {
    in_scadenza: "In scadenza",
    scaduto: "Scaduti",
    nessuno: "Non presentati",
  };
</script>

<section class="colonna">
  <h2 class={colonna}>{titolo} <span class="nota">{nota}</span></h2>
  <div class="scelte">
    <div class="segmenti" role="group" aria-label="Tipo di scadenza">
      {#each schede as scheda (scheda.tipo)}
        <button
          type="button"
          aria-pressed={scheda.tipo === schedaScelta.tipo}
          onclick={() => (tipoScelto = scheda.tipo)}
        >
          {scheda.etichetta}
          {#if riepilogo}({numeroNellaScheda(scheda, riepilogo)}){/if}
        </button>
      {/each}
    </div>
    {#if schedaScelta.stati.length > 1}
      <div class="segmenti" role="group" aria-label="Stato">
        {#each schedaScelta.stati as stato (stato)}
          <button
            type="button"
            aria-pressed={stato === statoMostrato}
            onclick={() => (statoScelto = stato)}
          >
            {ETICHETTE_STATO[stato]}
            {#if riepilogo}({numeroIscritti(schedaScelta.tipo, stato, riepilogo)}){/if}
          </button>
        {/each}
      </div>
    {/if}
  </div>
  {#key `${schedaScelta.tipo}-${statoMostrato}`}
    <ElencoScadenze tipo={schedaScelta.tipo} stato={statoMostrato} compatto {onCambiato} />
  {/key}
</section>

<style>
  .colonna {
    display: grid;
    gap: 0.6rem;
    align-content: start;
    min-width: 0;
  }

  h2 {
    margin: 0;
    font-size: 1.15rem;
  }

  h2.in_scadenza {
    color: #8a5300;
  }

  h2.scaduto {
    color: #b42318;
  }

  .nota {
    color: var(--colore-testo-secondario);
    font-weight: 400;
    font-size: 0.95rem;
  }

  .scelte {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
</style>
