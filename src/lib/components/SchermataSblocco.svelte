<script lang="ts">
  import { messaggioErrore, sbloccaDatabase } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";

  let { onSbloccato }: { onSbloccato: () => void } = $props();

  let password = $state("");
  let errore = $state<string | null>(null);
  let inCorso = $state(false);

  async function sblocca(evento: SubmitEvent) {
    evento.preventDefault();
    inCorso = true;
    errore = null;
    try {
      const esito = await sbloccaDatabase(password);
      if (esito.avvisoBackup) mostraAvviso("avviso", esito.avvisoBackup);
      onSbloccato();
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
      password = "";
    } finally {
      inCorso = false;
    }
  }
</script>

<form class="riquadro" onsubmit={sblocca}>
  <h1>SP Gestionale</h1>
  <p>Inserisci la password per aprire l'archivio.</p>

  <label>
    Password
    <!-- svelte-ignore a11y_autofocus -->
    <input
      type="password"
      bind:value={password}
      autocomplete="current-password"
      autofocus
      required
    />
  </label>

  {#if errore}<p class="errore" role="alert">{errore}</p>{/if}

  <button type="submit" disabled={inCorso}>{inCorso ? "Verifica…" : "Sblocca"}</button>
</form>
