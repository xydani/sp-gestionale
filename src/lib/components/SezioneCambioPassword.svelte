<script lang="ts">
  import { cambiaPassword, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { LUNGHEZZA_MINIMA_PASSWORD, validaNuovaPassword } from "$lib/password";

  let passwordAttuale = $state("");
  let nuovaPassword = $state("");
  let confermaPassword = $state("");
  let errore = $state<string | null>(null);
  let inCorso = $state(false);

  async function conferma(evento: SubmitEvent) {
    evento.preventDefault();
    errore = validaNuovaPassword(nuovaPassword, confermaPassword);
    if (errore) return;
    inCorso = true;
    try {
      await cambiaPassword(passwordAttuale, nuovaPassword, confermaPassword);
      passwordAttuale = "";
      nuovaPassword = "";
      confermaPassword = "";
      mostraAvviso("successo", "Password cambiata. Dal prossimo avvio usa quella nuova.");
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
    } finally {
      inCorso = false;
    }
  }
</script>

<section class="scheda">
  <h2>Password</h2>
  <p class="nota">
    I backup già salvati restano protetti dalla password con cui sono stati creati: per
    ripristinarli servirà quella.
  </p>
  <form onsubmit={conferma}>
    <label>
      Password attuale
      <input
        type="password"
        bind:value={passwordAttuale}
        autocomplete="current-password"
        required
      />
    </label>
    <label>
      Nuova password (almeno {LUNGHEZZA_MINIMA_PASSWORD} caratteri)
      <input type="password" bind:value={nuovaPassword} autocomplete="new-password" required />
    </label>
    <label>
      Ripeti la nuova password
      <input type="password" bind:value={confermaPassword} autocomplete="new-password" required />
    </label>
    {#if errore}<p class="errore" role="alert">{errore}</p>{/if}
    <div>
      <button type="submit" disabled={inCorso}>{inCorso ? "Cambio…" : "Cambia password"}</button>
    </div>
  </form>
</section>

<style>
  form {
    display: grid;
    gap: 0.75rem;
    max-width: 360px;
  }

  label {
    display: grid;
    gap: 0.35rem;
  }

  input {
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
  }

  button {
    padding: 0.6rem 1.2rem;
    border-radius: var(--raggio);
    border: none;
    background: var(--colore-primario);
    color: var(--colore-primario-testo);
    font: inherit;
  }

  .nota {
    color: var(--colore-testo-secondario);
  }

  .errore {
    margin: 0;
    color: var(--colore-errore);
  }
</style>
