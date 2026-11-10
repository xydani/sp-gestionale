<script lang="ts">
  import { untrack } from "svelte";
  import {
    GIORNI_SETTIMANA,
    fuoriIntervallo,
    grigliaMese,
    meseIniziale,
    spostaMese,
  } from "$lib/calendario";
  import { nomeMeseDaNumero, oggiIso } from "$lib/format";

  /** Calendario di un mese per scegliere una data con il mouse. */
  let {
    selezionata,
    minimaIso = null,
    massimaIso = null,
    onScegli,
  }: {
    selezionata: string;
    minimaIso?: string | null;
    massimaIso?: string | null;
    onScegli: (dataIso: string) => void;
  } = $props();

  const oggi = oggiIso();
  let mostrato = $state(
    meseIniziale(
      untrack(() => selezionata),
      oggi,
    ),
  );
  const giorni = $derived(grigliaMese(mostrato));
</script>

<div class="calendario" role="dialog" aria-label="Scegli una data">
  <div class="intestazione">
    <button
      type="button"
      class="freccia"
      aria-label="Mese precedente"
      onclick={() => (mostrato = spostaMese(mostrato, -1))}>‹</button
    >
    <span class="mese">{nomeMeseDaNumero(mostrato.mese)} {mostrato.anno}</span>
    <button
      type="button"
      class="freccia"
      aria-label="Mese successivo"
      onclick={() => (mostrato = spostaMese(mostrato, 1))}>›</button
    >
  </div>

  <div class="griglia">
    {#each GIORNI_SETTIMANA as giorno, indice (indice)}
      <span class="giorno-settimana">{giorno}</span>
    {/each}
    {#each giorni as giorno (giorno.iso)}
      <button
        type="button"
        class="giorno"
        class:altro-mese={!giorno.delMese}
        class:oggi={giorno.iso === oggi}
        class:scelto={giorno.iso === selezionata}
        disabled={fuoriIntervallo(giorno.iso, minimaIso, massimaIso)}
        aria-pressed={giorno.iso === selezionata}
        onclick={() => onScegli(giorno.iso)}>{giorno.numero}</button
      >
    {/each}
  </div>

  {#if !fuoriIntervallo(oggi, minimaIso, massimaIso)}
    <button type="button" class="pulsante-oggi" onclick={() => onScegli(oggi)}>Oggi</button>
  {/if}
</div>

<style>
  .calendario {
    display: grid;
    gap: 0.5rem;
    width: 17rem;
    padding: 0.75rem;
    background: var(--colore-superficie);
    border: 1px solid var(--colore-bordo);
    border-radius: 12px;
    box-shadow: 0 12px 32px rgb(29 35 48 / 0.16);
  }

  .intestazione {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .mese {
    font-weight: 600;
    text-transform: capitalize;
  }

  .freccia {
    width: 2rem;
    height: 2rem;
    border: none;
    border-radius: 50%;
    background: transparent;
    font-size: 1.3rem;
    line-height: 1;
    color: var(--colore-testo-secondario);
  }

  .freccia:hover {
    background: var(--colore-primario-tenue);
    color: var(--colore-primario);
  }

  .griglia {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 2px;
    text-align: center;
  }

  .giorno-settimana {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--colore-testo-secondario);
    padding: 0.25rem 0;
  }

  .giorno {
    aspect-ratio: 1;
    border: none;
    border-radius: 50%;
    background: transparent;
    font-size: 0.9rem;
    font-variant-numeric: tabular-nums;
    color: var(--colore-testo);
  }

  .giorno:hover:not(:disabled) {
    background: var(--colore-primario-tenue);
    color: var(--colore-primario);
  }

  .altro-mese {
    color: var(--colore-testo-secondario);
    opacity: 0.55;
  }

  .oggi {
    box-shadow: inset 0 0 0 1px var(--colore-primario);
    font-weight: 600;
  }

  .giorno.scelto,
  .giorno.scelto:hover {
    background: var(--colore-primario);
    color: var(--colore-primario-testo);
  }

  .giorno:disabled {
    opacity: 0.25;
    cursor: default;
  }

  .pulsante-oggi {
    justify-self: end;
    border: none;
    background: transparent;
    color: var(--colore-primario);
    font-weight: 500;
    padding: 0.25rem 0.5rem;
    border-radius: var(--raggio);
  }

  .pulsante-oggi:hover {
    background: var(--colore-primario-tenue);
  }
</style>
