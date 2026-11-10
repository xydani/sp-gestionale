<script lang="ts">
  import { getVersion } from "@tauri-apps/api/app";
  import { onMount } from "svelte";
  import {
    aggiornamento,
    installaAggiornamento,
    percentualeScaricata,
    verificaAggiornamenti,
  } from "$lib/aggiornamento.svelte";

  let versioneInstallata = $state("");

  const inCorso = $derived(
    ["controllo", "backup", "download", "installazione"].includes(aggiornamento.fase),
  );
  const percentuale = $derived(percentualeScaricata(aggiornamento));

  onMount(async () => {
    versioneInstallata = await getVersion();
  });
</script>

<section class="scheda">
  <h2>Aggiornamenti</h2>
  <p>Versione installata: <strong>{versioneInstallata}</strong></p>

  {#if aggiornamento.fase === "nessuno"}
    <p>Stai usando la versione più recente.</p>
  {:else if aggiornamento.fase === "errore"}
    <p class="errore" role="alert">{aggiornamento.errore}</p>
  {:else if aggiornamento.fase === "disponibile"}
    <p>È disponibile la versione <strong>{aggiornamento.versione}</strong>.</p>
  {:else if aggiornamento.fase === "backup"}
    <p>Backup dei dati in corso…</p>
  {:else if aggiornamento.fase === "download"}
    <p>Download in corso{percentuale !== null ? `: ${percentuale}%` : "…"}</p>
    <progress max="100" value={percentuale ?? undefined}></progress>
  {:else if aggiornamento.fase === "installazione"}
    <p>Installazione in corso: il programma si riavvierà da solo.</p>
  {/if}

  {#if aggiornamento.note && aggiornamento.fase === "disponibile"}
    <div class="note">
      <h3>Novità</h3>
      <p>{aggiornamento.note}</p>
    </div>
  {/if}

  <p class="nota">Prima di installare viene salvato un backup dei dati tra i backup automatici.</p>

  <div class="azioni">
    <button
      type="button"
      class="secondario"
      onclick={() => verificaAggiornamenti()}
      disabled={inCorso}
    >
      {aggiornamento.fase === "controllo" ? "Verifica in corso…" : "Verifica aggiornamenti"}
    </button>
    {#if aggiornamento.fase === "disponibile"}
      <button type="button" onclick={installaAggiornamento}>Aggiorna ora</button>
    {/if}
  </div>
</section>

<style>
  .azioni {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  button {
    padding: 0.6rem 1rem;
    border-radius: var(--raggio);
    border: 1px solid var(--colore-primario);
    background: var(--colore-primario);
    color: var(--colore-primario-testo);
    font: inherit;
  }

  button.secondario {
    background: transparent;
    color: var(--colore-primario);
  }

  .note {
    padding: 0.75rem 1rem;
    border-radius: var(--raggio);
    background: var(--colore-sfondo);
  }

  .note h3 {
    margin: 0 0 0.35rem;
    font-size: 1rem;
  }

  .note p {
    white-space: pre-line;
  }

  .nota {
    color: var(--colore-testo-secondario);
  }

  .errore {
    color: var(--colore-errore);
  }

  progress {
    width: 100%;
    accent-color: var(--colore-primario);
  }
</style>
