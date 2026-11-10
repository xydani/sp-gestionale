<script lang="ts">
  import {
    aggiornamento,
    installaAggiornamento,
    percentualeScaricata,
    rimandaAggiornamento,
  } from "$lib/aggiornamento.svelte";

  const inCorso = $derived(["backup", "download", "installazione"].includes(aggiornamento.fase));
  const visibile = $derived(
    inCorso || (aggiornamento.fase === "disponibile" && !aggiornamento.rimandato),
  );
  const percentuale = $derived(percentualeScaricata(aggiornamento));
</script>

{#if visibile}
  <div class="banner" role="status">
    {#if aggiornamento.fase === "backup"}
      <span>Backup dei dati prima dell'aggiornamento…</span>
    {:else if aggiornamento.fase === "download"}
      <span
        >Download della versione {aggiornamento.versione}{percentuale !== null
          ? ` (${percentuale}%)`
          : "…"}</span
      >
      <progress max="100" value={percentuale ?? undefined}></progress>
    {:else if aggiornamento.fase === "installazione"}
      <span>Installazione in corso: il programma si riavvierà da solo.</span>
    {:else}
      <span>È disponibile la versione <strong>{aggiornamento.versione}</strong>.</span>
      <div class="azioni">
        <button class="pulsante" type="button" onclick={installaAggiornamento}>Aggiorna ora</button>
        <button type="button" class="pulsante secondario" onclick={rimandaAggiornamento}
          >Più tardi</button
        >
      </div>
    {/if}
  </div>
{/if}

<style>
  .banner {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem 1rem;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1rem;
    margin-bottom: 1rem;
    border-radius: var(--raggio);
    border: 1px solid var(--colore-primario);
    background: var(--colore-superficie);
  }

  .azioni {
    display: flex;
    gap: 0.5rem;
  }

  progress {
    flex: 1;
    min-width: 160px;
    accent-color: var(--colore-primario);
  }
</style>
