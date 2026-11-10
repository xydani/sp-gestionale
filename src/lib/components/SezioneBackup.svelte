<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";
  import {
    eseguiBackupManuale,
    leggiCartellaBackupAutomatici,
    messaggioErrore,
    ripristinaBackup,
  } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";

  let { onRipristinato }: { onRipristinato: () => void } = $props();

  let cartellaBackupAutomatici = $state("");
  let percorsoBackupDaRipristinare = $state<string | null>(null);
  let passwordBackup = $state("");
  let inCorso = $state(false);

  onMount(async () => {
    cartellaBackupAutomatici = await leggiCartellaBackupAutomatici();
  });

  async function salvaBackup() {
    const cartella = await open({ directory: true, title: "Scegli dove salvare il backup" });
    if (!cartella) return;
    inCorso = true;
    try {
      const percorso = await eseguiBackupManuale(cartella);
      mostraAvviso("successo", `Backup salvato in ${percorso}`);
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    } finally {
      inCorso = false;
    }
  }

  async function scegliBackupDaRipristinare() {
    const file = await open({
      title: "Scegli il backup da ripristinare",
      filters: [{ name: "Backup SP Gestionale", extensions: ["db"] }],
    });
    percorsoBackupDaRipristinare = file;
    passwordBackup = "";
  }

  async function confermaRipristino(evento: SubmitEvent) {
    evento.preventDefault();
    if (!percorsoBackupDaRipristinare) return;
    inCorso = true;
    try {
      await ripristinaBackup(percorsoBackupDaRipristinare, passwordBackup);
      mostraAvviso("successo", "Backup ripristinato. I dati precedenti sono stati salvati.");
      percorsoBackupDaRipristinare = null;
      onRipristinato();
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    } finally {
      passwordBackup = "";
      inCorso = false;
    }
  }
</script>

<section class="scheda">
  <h2>Backup</h2>
  <p>
    Ogni giorno, al primo sblocco, viene salvato un backup automatico criptato in:
    <code>{cartellaBackupAutomatici}</code>
  </p>
  <p>
    Per proteggersi da guasti o furti del PC salvate regolarmente un backup anche su una chiavetta o
    una cartella cloud.
  </p>

  <div class="azioni">
    <button class="pulsante" type="button" onclick={salvaBackup} disabled={inCorso}
      >Salva backup su…</button
    >
    <button
      type="button"
      class="pulsante secondario"
      onclick={scegliBackupDaRipristinare}
      disabled={inCorso}
    >
      Ripristina da backup…
    </button>
  </div>

  {#if percorsoBackupDaRipristinare}
    <form class="ripristino" onsubmit={confermaRipristino}>
      <p class="avviso">
        I dati attuali verranno sostituiti da quelli di <code>{percorsoBackupDaRipristinare}</code>.
        Prima viene salvata automaticamente una copia dei dati attuali.
      </p>
      <label>
        Password del backup
        <input type="password" bind:value={passwordBackup} required />
      </label>
      <div class="azioni">
        <button class="pulsante" type="submit" disabled={inCorso}>Ripristina</button>
        <button
          type="button"
          class="pulsante secondario"
          onclick={() => (percorsoBackupDaRipristinare = null)}
        >
          Annulla
        </button>
      </div>
    </form>
  {/if}
</section>

<style>
  code {
    word-break: break-all;
  }

  .azioni {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .ripristino {
    display: grid;
    gap: 0.75rem;
    border-top: 1px solid var(--colore-bordo);
    padding-top: 0.75rem;
  }

  .avviso {
    color: var(--colore-avviso);
  }

  label {
    display: grid;
    gap: 0.35rem;
    max-width: 320px;
  }

  input {
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
  }
</style>
