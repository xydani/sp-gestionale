<script lang="ts">
  import { onMount } from "svelte";
  import { leggiImpostazioni, leggiRiepilogoDashboard, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import ColonnaScadenze from "$lib/components/ColonnaScadenze.svelte";
  import { aggiornaConteggioScadenze } from "$lib/promemoria.svelte";
  import type { RiepilogoDashboard } from "$lib/types";

  let riepilogo = $state<RiepilogoDashboard | null>(null);
  let giorniPreavviso = $state<number | null>(null);

  const notaInScadenza = $derived(
    giorniPreavviso === null ? "" : `(prossimi ${giorniPreavviso} giorni)`,
  );

  async function carica() {
    try {
      riepilogo = await leggiRiepilogoDashboard();
      await aggiornaConteggioScadenze();
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }

  onMount(async () => {
    await carica();
    try {
      giorniPreavviso = (await leggiImpostazioni()).giorniPreavviso;
    } catch {
      // Solo la nota del titolo: senza si vede comunque l'elenco
    }
  });
</script>

<h1 class="titolo">Scadenze</h1>

<!-- Scaduti e in scadenza affiancati, ognuno con le sue schede per tipo -->
<div class="colonne">
  <ColonnaScadenze
    colonna="scaduto"
    titolo="Già scaduti"
    nota="(i più recenti prima)"
    {riepilogo}
    onCambiato={carica}
  />
  <ColonnaScadenze
    colonna="in_scadenza"
    titolo="In scadenza"
    nota={notaInScadenza}
    {riepilogo}
    onCambiato={carica}
  />
</div>

<style>
  .titolo {
    margin: 0 0 0.9rem;
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
</style>
