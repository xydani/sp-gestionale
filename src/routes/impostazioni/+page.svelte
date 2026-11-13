<script lang="ts">
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { leggiImpostazioni, messaggioErrore, salvaImpostazioni } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import SezioneAggiornamenti from "$lib/components/SezioneAggiornamenti.svelte";
  import FormImpostazioni from "$lib/components/FormImpostazioni.svelte";
  import SezioneAvvioAutomatico from "$lib/components/SezioneAvvioAutomatico.svelte";
  import SezioneBackup from "$lib/components/SezioneBackup.svelte";
  import SezioneDimensioneTesto from "$lib/components/SezioneDimensioneTesto.svelte";
  import SezioneCambioPassword from "$lib/components/SezioneCambioPassword.svelte";
  import SezioneLogo from "$lib/components/SezioneLogo.svelte";
  import { bloccoAutomatico } from "$lib/blocco-automatico.svelte";
  import { palestra } from "$lib/palestra.svelte";
  import { aggiornaConteggioScadenze } from "$lib/promemoria.svelte";
  import {
    SCHEDE_IMPOSTAZIONI,
    schedaDaIndirizzo,
    type SchedaImpostazioni,
  } from "$lib/schede-impostazioni";
  import type { Impostazioni } from "$lib/types";

  let impostazioni = $state<Impostazioni | null>(null);
  let schedaAttiva = $state<SchedaImpostazioni>(
    schedaDaIndirizzo(page.url.searchParams.get("scheda")),
  );

  onMount(async () => {
    try {
      impostazioni = await leggiImpostazioni();
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  });

  /** Le impostazioni valgono subito anche fuori da questa pagina (menu, blocco, scadenze). */
  async function applica(nuove: Impostazioni) {
    impostazioni = nuove;
    palestra.nome = nuove.nomePalestra;
    bloccoAutomatico.minuti = nuove.minutiBloccoAutomatico;
    // I giorni di preavviso cambiano cosa è "in scadenza"
    await aggiornaConteggioScadenze();
  }

  async function salva(nuove: Impostazioni) {
    try {
      await applica(await salvaImpostazioni(nuove));
      mostraAvviso("successo", "Impostazioni salvate");
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }

  /** Dopo un ripristino le impostazioni sono quelle del backup: il modulo va ricaricato. */
  async function ricaricaDopoRipristino() {
    try {
      await applica(await leggiImpostazioni());
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }
</script>

<div class="pagina-stretta">
  <div class="intestazione">
    <h1>Impostazioni</h1>
    <div class="segmenti" role="group" aria-label="Sezione delle impostazioni">
      {#each SCHEDE_IMPOSTAZIONI as scheda (scheda.chiave)}
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
  <div class="sezioni">
    {#if schedaAttiva === "palestra" && impostazioni}
      <section class="scheda">
        <h2>Palestra</h2>
        <p class="nota">
          Quota annuale e inizio dell'anno sportivo si cambiano dal
          <a href={resolve("/listino")}>Listino</a>, insieme agli altri prezzi.
        </p>
        {#key impostazioni}
          <FormImpostazioni iniziale={impostazioni} etichettaConferma="Salva" onSalva={salva} />
        {/key}
      </section>
    {:else if schedaAttiva === "aspetto"}
      <SezioneLogo />
      <SezioneDimensioneTesto />
    {:else if schedaAttiva === "password"}
      <SezioneCambioPassword />
    {:else if schedaAttiva === "backup"}
      <SezioneBackup onRipristinato={ricaricaDopoRipristino} />
    {:else if schedaAttiva === "programma"}
      <SezioneAvvioAutomatico />
      <SezioneAggiornamenti />
    {/if}
  </div>
</div>

<style>
  .intestazione {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem 1.5rem;
    margin-bottom: 1rem;
  }

  .intestazione h1 {
    margin: 0;
  }

  .sezioni {
    display: grid;
    gap: 1.25rem;
  }

  .nota {
    color: var(--colore-testo-secondario);
  }
</style>
