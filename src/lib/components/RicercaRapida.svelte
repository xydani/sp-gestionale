<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";
  import { elencaIscritti } from "$lib/api";
  import { formattaData } from "$lib/format";
  import { spostaSelezione } from "$lib/menu";
  import { ritarda } from "$lib/ritarda";
  import type { Iscritto } from "$lib/types";
  import BadgeStato from "./BadgeStato.svelte";

  /** Ricerca da qualsiasi pagina: si scrive il nome e con Invio si apre la scheda. */
  let { onChiusa }: { onChiusa: () => void } = $props();

  const PAUSA_RICERCA_MS = 150;
  const RISULTATI_MASSIMI = 8;

  let finestra = $state<HTMLDialogElement>();
  let testo = $state("");
  let risultati = $state<Iscritto[]>([]);
  let evidenziato = $state(0);
  let cercato = $state(false);

  // Conta le ricerche: una risposta lenta non deve sovrascrivere quella di una ricerca successiva
  let ultimaRicerca = 0;

  onMount(() => finestra?.showModal());

  const cerca = ritarda(async () => {
    const questaRicerca = ++ultimaRicerca;
    if (!testo.trim()) {
      risultati = [];
      cercato = false;
      return;
    }
    try {
      const trovati = await elencaIscritti({
        testo,
        includiArchiviati: true,
        statoAbbonamento: null,
        statoIscrizione: null,
      });
      if (questaRicerca !== ultimaRicerca) return;
      risultati = trovati.elementi.slice(0, RISULTATI_MASSIMI);
    } catch {
      risultati = [];
    }
    evidenziato = 0;
    cercato = true;
  }, PAUSA_RICERCA_MS);

  async function apri(iscritto: Iscritto) {
    onChiusa();
    await goto(resolve("/iscritti/[id]", { id: String(iscritto.id) }));
  }

  function gestisciTasti(evento: KeyboardEvent) {
    if (evento.key === "ArrowDown" || evento.key === "ArrowUp") {
      evento.preventDefault();
      const passo = evento.key === "ArrowDown" ? 1 : -1;
      evidenziato = spostaSelezione(evidenziato, passo, risultati.length);
    } else if (evento.key === "Enter" && risultati[evidenziato]) {
      evento.preventDefault();
      apri(risultati[evidenziato]);
    }
  }

  function annullaConEsc(evento: Event) {
    evento.preventDefault();
    onChiusa();
  }

  /**
   * Esc chiude sempre: alcuni campi (come quello di ricerca) usano Esc per svuotarsi
   * e il browser integrato allora non avvisa la finestra.
   */
  function chiudiConEsc(evento: KeyboardEvent) {
    if (evento.key !== "Escape") return;
    evento.preventDefault();
    evento.stopPropagation();
    onChiusa();
  }

  /** Un clic sullo sfondo scuro chiude la ricerca. */
  function chiudiSeSfondo(evento: MouseEvent) {
    if (evento.target === finestra) onChiusa();
  }
</script>

<dialog
  bind:this={finestra}
  oncancel={annullaConEsc}
  onkeydown={chiudiConEsc}
  onclick={chiudiSeSfondo}
  aria-label="Cerca un iscritto"
>
  <input
    type="search"
    placeholder="Cerca un iscritto per nome o cognome…"
    bind:value={testo}
    oninput={cerca}
    onkeydown={gestisciTasti}
    autofocus
  />
  {#if risultati.length > 0}
    <ul>
      {#each risultati as iscritto, indice (iscritto.id)}
        <li>
          <button
            type="button"
            class:evidenziato={indice === evidenziato}
            onclick={() => apri(iscritto)}
            onmouseenter={() => (evidenziato = indice)}
          >
            <span class="nome">
              {iscritto.cognome}
              {iscritto.nome}
              {#if iscritto.archiviato}<span class="nota">archiviato</span>{/if}
            </span>
            {#if iscritto.ultimaDataScadenzaAbbonamento}
              <span class="nota">scade {formattaData(iscritto.ultimaDataScadenzaAbbonamento)}</span>
            {/if}
            <BadgeStato stato={iscritto.statoAbbonamento} />
          </button>
        </li>
      {/each}
    </ul>
  {:else if cercato}
    <p class="vuoto">Nessun iscritto trovato</p>
  {:else}
    <p class="vuoto">↑ ↓ per scegliere, Invio per aprire la scheda, Esc per chiudere</p>
  {/if}
</dialog>

<style>
  dialog {
    width: min(36rem, calc(100vw - 3rem));
    margin-top: 12vh;
    padding: 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio-grande);
    background: var(--colore-superficie);
    color: var(--colore-testo);
    box-shadow: var(--ombra-sospesa);
  }

  dialog::backdrop {
    background: rgb(24 24 27 / 0.35);
  }

  input {
    width: 100%;
    padding: 0.7rem 0.85rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
    font-size: 1.05rem;
  }

  ul {
    list-style: none;
    margin: 0.5rem 0 0;
    padding: 0;
    display: grid;
    gap: 2px;
  }

  li button {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    padding: 0.55rem 0.75rem;
    border: none;
    border-radius: var(--raggio);
    background: transparent;
    font: inherit;
    text-align: left;
    color: inherit;
    cursor: pointer;
  }

  li button.evidenziato {
    background: var(--colore-primario-tenue);
  }

  .nome {
    flex: 1;
    font-weight: 500;
  }

  .nota {
    margin-left: 0.4rem;
    color: var(--colore-testo-secondario);
    font-size: 0.85rem;
    font-weight: 400;
  }

  .vuoto {
    margin: 0.75rem 0.25rem 0.25rem;
    color: var(--colore-testo-secondario);
    font-size: 0.9rem;
  }
</style>
