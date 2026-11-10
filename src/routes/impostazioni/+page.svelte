<script lang="ts">
  import { onMount } from "svelte";
  import { leggiImpostazioni, messaggioErrore, salvaImpostazioni } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import FormImpostazioni from "$lib/components/FormImpostazioni.svelte";
  import SezioneAvvioAutomatico from "$lib/components/SezioneAvvioAutomatico.svelte";
  import SezioneBackup from "$lib/components/SezioneBackup.svelte";
  import SezioneCambioPassword from "$lib/components/SezioneCambioPassword.svelte";
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

  async function salva(nuove: Impostazioni) {
    try {
      impostazioni = await salvaImpostazioni(nuove);
      // I giorni di preavviso cambiano cosa è "in scadenza"
      await aggiornaConteggioScadenze();
      mostraAvviso("successo", "Impostazioni salvate");
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
        <FormImpostazioni iniziale={impostazioni} etichettaConferma="Salva" onSalva={salva} />
      </section>
    {/if}
    <SezioneCambioPassword />
    <SezioneAvvioAutomatico />
    <SezioneBackup />
  </div>
</div>

<style>
  .sezioni {
    display: grid;
    gap: 1.25rem;
  }
</style>
