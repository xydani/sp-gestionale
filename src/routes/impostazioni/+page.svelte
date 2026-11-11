<script lang="ts">
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
  import type { Impostazioni } from "$lib/types";

  let impostazioni = $state<Impostazioni | null>(null);

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
  <h1>Impostazioni</h1>
  <div class="sezioni">
    {#if impostazioni}
      <section class="scheda">
        <h2>Palestra</h2>
        {#key impostazioni}
          <FormImpostazioni iniziale={impostazioni} etichettaConferma="Salva" onSalva={salva} />
        {/key}
      </section>
    {/if}
    <SezioneLogo />
    <SezioneDimensioneTesto />
    <SezioneCambioPassword />
    <SezioneAvvioAutomatico />
    <SezioneBackup onRipristinato={ricaricaDopoRipristino} />
    <SezioneAggiornamenti />
  </div>
</div>

<style>
  .sezioni {
    display: grid;
    gap: 1.25rem;
  }
</style>
