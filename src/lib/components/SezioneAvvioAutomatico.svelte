<script lang="ts">
  import { onMount } from "svelte";
  import { messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { avvioAutomaticoAttivo, impostaAvvioAutomatico } from "$lib/avvio-automatico";

  let attivo = $state(false);
  let caricato = $state(false);

  onMount(async () => {
    try {
      attivo = await avvioAutomaticoAttivo();
      caricato = true;
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  });

  async function cambia() {
    try {
      await impostaAvvioAutomatico(attivo);
    } catch (errore) {
      attivo = !attivo;
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }
</script>

<section class="scheda">
  <h2>Avvio</h2>
  <label>
    <input type="checkbox" bind:checked={attivo} onchange={cambia} disabled={!caricato} />
    Apri SP Gestionale all'accensione del computer
  </label>
</section>

<style>
  label {
    display: flex;
    gap: 0.6rem;
    align-items: center;
  }

  input {
    width: 1.1rem;
    height: 1.1rem;
  }
</style>
