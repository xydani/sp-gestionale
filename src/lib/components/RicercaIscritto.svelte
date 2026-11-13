<script lang="ts">
  import { elencaIscritti } from "$lib/api";
  import { sospesoSottoCampo } from "$lib/riquadro-sospeso";
  import { ritarda } from "$lib/ritarda";
  import type { Iscritto } from "$lib/types";
  import BadgeStato from "./BadgeStato.svelte";

  const PAUSA_RICERCA_MS = 200;
  const RISULTATI_MASSIMI = 8;

  let { onScelto }: { onScelto: (iscritto: Iscritto) => void } = $props();

  let testo = $state("");
  let contenitore = $state<HTMLDivElement>();
  let risultati = $state<Iscritto[]>([]);

  // Conta le ricerche: una risposta lenta non deve sovrascrivere quella di una ricerca successiva
  let ultimaRicerca = 0;

  const cerca = ritarda(async () => {
    const questaRicerca = ++ultimaRicerca;
    if (!testo.trim()) {
      risultati = [];
      return;
    }
    const trovati = await elencaIscritti({
      testo,
      includiArchiviati: false,
      statoAbbonamento: null,
      statoIscrizione: null,
    });
    if (questaRicerca !== ultimaRicerca) return;
    risultati = trovati.elementi.slice(0, RISULTATI_MASSIMI);
  }, PAUSA_RICERCA_MS);

  function scegli(iscritto: Iscritto) {
    ultimaRicerca++;
    testo = "";
    risultati = [];
    onScelto(iscritto);
  }
</script>

<div class="ricerca" bind:this={contenitore}>
  <!-- svelte-ignore a11y_autofocus -->
  <input
    type="search"
    placeholder="Cerca l'iscritto per nome o cognome…"
    bind:value={testo}
    oninput={cerca}
    autofocus
  />
  {#if risultati.length > 0}
    <ul use:sospesoSottoCampo={{ ancora: contenitore, larghezza: "uguale" }}>
      {#each risultati as iscritto (iscritto.id)}
        <li>
          <button type="button" onclick={() => scegli(iscritto)}>
            <span>{iscritto.cognome} {iscritto.nome}</span>
            <BadgeStato stato={iscritto.statoAbbonamento} />
          </button>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .ricerca {
    position: relative;
  }

  input {
    width: 100%;
    padding: 0.55rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
  }

  /* Posizione e larghezza calcolate da sospesoSottoCampo */
  ul {
    z-index: 10;
    margin: 0;
    padding: 0.25rem;
    list-style: none;
    background: var(--colore-superficie);
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio-grande);
    box-shadow: var(--ombra-sospesa);
  }

  button {
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem 0.75rem;
    border: none;
    border-radius: var(--raggio);
    background: transparent;
    text-align: left;
  }

  button:hover {
    background: var(--colore-sfondo-tenue);
  }
</style>
