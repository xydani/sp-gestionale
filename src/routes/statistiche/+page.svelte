<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { leggiImpostazioni, leggiStatistiche, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import CampoData from "$lib/components/CampoData.svelte";
  import SezioneAttivi from "$lib/components/statistiche/SezioneAttivi.svelte";
  import SezionePrevisione from "$lib/components/statistiche/SezionePrevisione.svelte";
  import SezioneRinnovi from "$lib/components/statistiche/SezioneRinnovi.svelte";
  import SezioneValoreMedio from "$lib/components/statistiche/SezioneValoreMedio.svelte";
  import SezioneVendite from "$lib/components/statistiche/SezioneVendite.svelte";
  import {
    ETICHETTE_PERIODO_STATISTICHE,
    MESE_INIZIO_ANNO_SPORTIVO_PREDEFINITO,
    calcolaPeriodoStatistiche,
    type PeriodoStatistiche,
  } from "$lib/statistiche";
  import type { Statistiche } from "$lib/types";

  const PERIODI = Object.keys(ETICHETTE_PERIODO_STATISTICHE) as PeriodoStatistiche[];

  /** Una scheda per argomento: ognuna sta nella finestra; il periodo vale per tutte. */
  const SCHEDE = [
    { chiave: "rinnovi", etichetta: "Rinnovi" },
    { chiave: "attivi", etichetta: "Iscritti attivi" },
    { chiave: "incassi", etichetta: "Incassi" },
    { chiave: "vendite", etichetta: "Vendite" },
  ] as const;
  // Si può aprire una scheda dall'indirizzo, es. `?scheda=vendite`
  const schedaIniziale = SCHEDE.find(
    (scheda) => scheda.chiave === page.url.searchParams.get("scheda"),
  );
  let schedaAttiva = $state<(typeof SCHEDE)[number]["chiave"]>(schedaIniziale?.chiave ?? "rinnovi");

  let meseInizioAnnoSportivo = MESE_INIZIO_ANNO_SPORTIVO_PREDEFINITO;
  let sceltaPeriodo = $state<PeriodoStatistiche | null>("ultimi_12_mesi");
  let dal = $state("");
  let al = $state("");
  let statistiche = $state<Statistiche | null>(null);

  // Conta i caricamenti: una risposta lenta non deve sovrascrivere quella di un periodo successivo
  let ultimoCaricamento = 0;

  onMount(async () => {
    try {
      meseInizioAnnoSportivo = (await leggiImpostazioni()).meseInizioAnnoSportivo;
    } catch {
      // Senza impostazioni si usa settembre: le statistiche restano consultabili
    }
    scegliPeriodo("ultimi_12_mesi");
  });

  function scegliPeriodo(scelta: PeriodoStatistiche) {
    ({ dal, al } = calcolaPeriodoStatistiche(scelta, meseInizioAnnoSportivo));
    sceltaPeriodo = scelta;
    carica();
  }

  function cambiaDate() {
    sceltaPeriodo = null;
    carica();
  }

  async function carica() {
    if (!dal || !al || dal > al) return;
    const questoCaricamento = ++ultimoCaricamento;
    try {
      const lette = await leggiStatistiche(dal, al);
      if (questoCaricamento === ultimoCaricamento) statistiche = lette;
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }
</script>

<div class="intestazione">
  <h1>Statistiche</h1>
  <div class="segmenti" role="group" aria-label="Argomento">
    {#each SCHEDE as scheda (scheda.chiave)}
      <button
        type="button"
        aria-pressed={schedaAttiva === scheda.chiave}
        onclick={() => (schedaAttiva = scheda.chiave)}
      >
        {scheda.etichetta}
      </button>
    {/each}
  </div>
</div>

<div class="periodo">
  <div class="segmenti" role="group" aria-label="Periodo">
    {#each PERIODI as scelta (scelta)}
      <button
        type="button"
        aria-pressed={sceltaPeriodo === scelta}
        onclick={() => scegliPeriodo(scelta)}
      >
        {ETICHETTE_PERIODO_STATISTICHE[scelta]}
      </button>
    {/each}
  </div>
  <span class="date">
    Dal <span class="campo-data"><CampoData bind:valore={dal} onCambia={cambiaDate} /></span>
    al
    <span class="campo-data"
      ><CampoData bind:valore={al} minimaIso={dal} onCambia={cambiaDate} /></span
    >
  </span>
</div>

{#if statistiche}
  <div class="sezioni">
    {#if schedaAttiva === "rinnovi"}
      <SezioneRinnovi
        abbonamenti={statistiche.rinnoviAbbonamenti}
        iscrizioni={statistiche.rinnoviIscrizioni}
        persi={statistiche.persi}
        giorniTolleranza={statistiche.giorniTolleranzaRientro}
      />
    {:else if schedaAttiva === "attivi"}
      <SezioneAttivi attivi={statistiche.attiviPerMese} />
    {:else if schedaAttiva === "incassi"}
      <SezioneValoreMedio valore={statistiche.valoreMedio} />
      <SezionePrevisione previsione={statistiche.previsione} />
    {:else}
      <SezioneVendite vendite={statistiche.venditePerTipologia} />
    {/if}
  </div>
{/if}

<style>
  .intestazione {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem 1.5rem;
    margin-bottom: 0.75rem;
  }

  .intestazione h1 {
    margin: 0;
  }

  .periodo {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.75rem 1.25rem;
    margin-bottom: 1rem;
  }

  .date {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .campo-data {
    width: 10rem;
  }

  .sezioni {
    display: grid;
    gap: 1.25rem;
  }

  .sezioni :global(.scheda) {
    display: grid;
    gap: 1rem;
  }

  .sezioni :global(h2) {
    margin: 0;
  }

  .sezioni :global(.spiegazione) {
    margin: 0;
    color: var(--colore-testo-secondario);
    font-size: 0.9rem;
  }

  .sezioni :global(.pochi-dati) {
    margin: 0;
    padding: 0.6rem 0.8rem;
    border-radius: var(--raggio);
    background: var(--colore-sfondo-tenue);
    color: var(--colore-testo-secondario);
  }
</style>
