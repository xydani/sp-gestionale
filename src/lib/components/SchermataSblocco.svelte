<script lang="ts">
  import Icona from "./Icona.svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import { onMount, tick } from "svelte";
  import { messaggioErrore, sbloccaDatabase } from "$lib/api";
  import { maiuscoleAttive } from "$lib/sblocco";

  let {
    motivoBlocco = null,
    onSbloccato,
  }: {
    /** Perché il programma si è bloccato da solo (es. inattività). */
    motivoBlocco?: string | null;
    onSbloccato: (avvisoBackup: string | null) => void;
  } = $props();

  let password = $state("");
  let passwordVisibile = $state(false);
  let maiuscole = $state(false);
  let errore = $state<string | null>(null);
  let inCorso = $state(false);
  let versione = $state("");
  let campoPassword = $state<HTMLInputElement>();

  onMount(() => {
    getVersion()
      .then((letta) => (versione = letta))
      .catch(() => {});
  });

  async function sblocca(evento: SubmitEvent) {
    evento.preventDefault();
    inCorso = true;
    errore = null;
    try {
      const esito = await sbloccaDatabase(password);
      password = "";
      passwordVisibile = false;
      onSbloccato(esito.avvisoBackup);
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
      await selezionaPassword();
    } finally {
      inCorso = false;
    }
  }

  /** Dopo un errore il testo resta selezionato: si riscrive da capo o si corregge. */
  async function selezionaPassword() {
    await tick();
    campoPassword?.focus();
    campoPassword?.select();
  }

  function controllaMaiuscole(evento: KeyboardEvent) {
    maiuscole = maiuscoleAttive(evento);
  }
</script>

<form class="riquadro" onsubmit={sblocca}>
  <h1>Bentornato</h1>
  {#if motivoBlocco}
    <p class="motivo" role="status">{motivoBlocco}</p>
  {:else}
    <p>Inserisci la password per aprire l'archivio.</p>
  {/if}

  <label>
    Password
    <span class="campo-password">
      <!-- svelte-ignore a11y_autofocus -->
      <input
        bind:this={campoPassword}
        type={passwordVisibile ? "text" : "password"}
        bind:value={password}
        autocomplete="current-password"
        autofocus
        required
        onkeydown={controllaMaiuscole}
        onkeyup={controllaMaiuscole}
      />
      <button
        type="button"
        class="occhio"
        onclick={() => (passwordVisibile = !passwordVisibile)}
        aria-label={passwordVisibile ? "Nascondi la password" : "Mostra la password"}
        title={passwordVisibile ? "Nascondi la password" : "Mostra la password"}
      >
        <Icona nome={passwordVisibile ? "occhio-barrato" : "occhio"} />
      </button>
    </span>
  </label>

  {#if maiuscole}<p class="maiuscole" role="status">Maiuscole attive</p>{/if}
  {#if errore}<p class="errore" role="alert">{errore}</p>{/if}

  <button type="submit" disabled={inCorso}>{inCorso ? "Verifica…" : "Sblocca"}</button>
</form>

{#if versione}<span class="versione">Versione {versione}</span>{/if}

<style>
  .campo-password {
    position: relative;
    display: grid;
  }

  .campo-password input {
    padding-right: 2.75rem;
  }

  .occhio {
    position: absolute;
    top: 50%;
    right: 0.4rem;
    display: grid;
    place-items: center;
    width: 2rem;
    height: 2rem;
    padding: 0;
    border: none;
    border-radius: 8px;
    background: transparent;
    color: var(--colore-testo-secondario);
    transform: translateY(-50%);
    cursor: pointer;
  }

  .occhio:hover {
    color: var(--colore-testo);
    background: var(--colore-sfondo-tenue);
  }

  .motivo,
  .maiuscole {
    padding: 0.5rem 0.75rem;
    border-radius: 10px;
    background: var(--colore-avviso-tenue);
    color: var(--colore-avviso);
    text-align: left;
  }

  .maiuscole {
    padding: 0.35rem 0.75rem;
    font-size: 0.85rem;
    font-weight: 500;
  }

  .versione {
    margin-top: auto;
    padding-top: 1rem;
    color: var(--colore-testo-secondario);
    font-size: 0.75rem;
  }
</style>
