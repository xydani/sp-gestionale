<script lang="ts">
  import { creaDatabase, messaggioErrore } from "$lib/api";
  import { LUNGHEZZA_MINIMA_PASSWORD, validaNuovaPassword } from "$lib/password";
  import IndicatoreRobustezza from "./IndicatoreRobustezza.svelte";

  let { onCompletato }: { onCompletato: () => void } = $props();

  let password = $state("");
  let confermaPassword = $state("");
  let errore = $state<string | null>(null);
  let inCorso = $state(false);

  async function conferma(evento: SubmitEvent) {
    evento.preventDefault();
    errore = validaNuovaPassword(password, confermaPassword);
    if (errore) return;

    inCorso = true;
    try {
      await creaDatabase(password, confermaPassword);
      password = "";
      confermaPassword = "";
      onCompletato();
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
    } finally {
      inCorso = false;
    }
  }
</script>

<form class="riquadro" onsubmit={conferma}>
  <h1>Benvenuto in SP Gestionale</h1>
  <p>Scegli la password che protegge i dati della palestra. Sarà richiesta a ogni avvio.</p>

  <p class="avviso">
    <strong>Attenzione:</strong> se perdi la password i dati non sono recuperabili. Conservala in un luogo
    sicuro.
  </p>

  <label>
    Password (almeno {LUNGHEZZA_MINIMA_PASSWORD} caratteri)
    <input type="password" bind:value={password} autocomplete="new-password" required />
  </label>
  <IndicatoreRobustezza {password} />
  <label>
    Ripeti la password
    <input type="password" bind:value={confermaPassword} autocomplete="new-password" required />
  </label>

  {#if errore}<p class="errore" role="alert">{errore}</p>{/if}

  <button type="submit" disabled={inCorso}>{inCorso ? "Creazione…" : "Crea archivio"}</button>
</form>
