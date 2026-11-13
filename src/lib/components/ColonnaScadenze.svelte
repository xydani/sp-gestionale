<script lang="ts">
  import { numeroNellaScheda, schedeColonna, type ColonnaScadenze } from "$lib/scadenze";
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

  /** Righe per pagina: la colonna sta nello schermo senza scorrere. */
  const RIGHE_PER_PAGINA = 10;
  /** Con due elenchi nella stessa scheda ognuno ha metà delle righe. */
  const righePerElenco = $derived(RIGHE_PER_PAGINA / schedaScelta.stati.length);

  /** Sottotitoli solo quando la scheda ha più elenchi (certificati scaduti e mai presentati). */
  const SOTTOTITOLI = { in_scadenza: "In scadenza", scaduto: "Scaduti", nessuno: "Non presentati" };
</script>

<section class="colonna">
  <h2 class={colonna}>{titolo} <span class="nota">{nota}</span></h2>
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
  {#key schedaScelta.tipo}
    {#each schedaScelta.stati as stato (stato)}
      {#if schedaScelta.stati.length > 1}<h3>{SOTTOTITOLI[stato]}</h3>{/if}
      <ElencoScadenze
        tipo={schedaScelta.tipo}
        {stato}
        compatto
        elementiPerPagina={righePerElenco}
        {onCambiato}
      />
    {/each}
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

  .segmenti {
    justify-self: start;
  }

  h3 {
    margin: 0.4rem 0 0;
    font-size: 1rem;
  }
</style>
