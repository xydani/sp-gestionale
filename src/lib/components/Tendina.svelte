<script lang="ts" generics="T">
  import Icona from "./Icona.svelte";
  import {
    indiceEstremo,
    indicePerLettera,
    indiceSuccessivo,
    type OpzioneTendina,
  } from "$lib/tendina";
  import { validita } from "$lib/validazione";

  /** Tendina disegnata dall'app: uguale su ogni sistema, usabile con mouse e tastiera. */
  let {
    valore = $bindable(),
    opzioni,
    segnaposto = "Scegli…",
    etichettaAccessibile,
    obbligatoria = false,
    onCambia,
  }: {
    valore: T;
    opzioni: OpzioneTendina<T>[];
    segnaposto?: string;
    etichettaAccessibile?: string;
    obbligatoria?: boolean;
    onCambia?: (valore: T) => void;
  } = $props();

  const idElenco = `tendina-${Math.random().toString(36).slice(2, 9)}`;

  let aperta = $state(false);
  let indiceEvidenziato = $state(-1);
  let contenitore = $state<HTMLDivElement>();
  let pulsante = $state<HTMLButtonElement>();
  let elenco = $state<HTMLUListElement>();

  const indiceScelto = $derived(opzioni.findIndex((opzione) => opzione.valore === valore));
  const scelta = $derived(indiceScelto >= 0 ? opzioni[indiceScelto] : null);
  const mostraSegnaposto = $derived(!scelta || scelta.disabilitata);

  function apri() {
    aperta = true;
    indiceEvidenziato = indiceScelto >= 0 ? indiceScelto : indiceEstremo(opzioni, false);
    queueMicrotask(mostraEvidenziata);
  }

  function chiudi(restituisciFocus = true) {
    aperta = false;
    if (restituisciFocus) pulsante?.focus();
  }

  function scegli(indice: number) {
    const opzione = opzioni[indice];
    if (!opzione || opzione.disabilitata) return;
    const cambiata = opzione.valore !== valore;
    valore = opzione.valore;
    chiudi();
    if (cambiata) onCambia?.(opzione.valore);
  }

  function evidenzia(indice: number) {
    indiceEvidenziato = indice;
    mostraEvidenziata();
  }

  function mostraEvidenziata() {
    elenco?.children[indiceEvidenziato]?.scrollIntoView({ block: "nearest" });
  }

  function gestisciTasti(evento: KeyboardEvent) {
    if (aperta) {
      gestisciTastiDaAperta(evento);
    } else {
      gestisciTastiDaChiusa(evento);
    }
  }

  /** Da chiusa, frecce, Invio e spazio aprono l'elenco. */
  function gestisciTastiDaChiusa(evento: KeyboardEvent) {
    if (["ArrowDown", "ArrowUp", "Enter", " "].includes(evento.key)) {
      evento.preventDefault();
      apri();
    }
  }

  /** Da aperta si scorre con frecce, Inizio e Fine, si sceglie con Invio, si salta con una lettera. */
  function gestisciTastiDaAperta(evento: KeyboardEvent) {
    const tasto = evento.key;
    if (tasto === "Tab") {
      chiudi(false);
      return;
    }
    const azione = azionePerTasto(tasto);
    if (azione) {
      evento.preventDefault();
      evento.stopPropagation();
      azione();
    } else if (tasto.length === 1) {
      evidenzia(indicePerLettera(opzioni, tasto, indiceEvidenziato));
    }
  }

  function azionePerTasto(tasto: string): (() => void) | null {
    switch (tasto) {
      case "ArrowDown":
        return () => evidenzia(indiceSuccessivo(opzioni, indiceEvidenziato, 1));
      case "ArrowUp":
        return () => evidenzia(indiceSuccessivo(opzioni, indiceEvidenziato, -1));
      case "Home":
        return () => evidenzia(indiceEstremo(opzioni, false));
      case "End":
        return () => evidenzia(indiceEstremo(opzioni, true));
      case "Enter":
      case " ":
        return () => scegli(indiceEvidenziato);
      case "Escape":
        return () => chiudi();
      default:
        return null;
    }
  }

  function chiudiSeFuori(evento: PointerEvent) {
    if (aperta && !contenitore?.contains(evento.target as Node)) chiudi(false);
  }
</script>

<svelte:window onpointerdown={chiudiSeFuori} />

<div class="tendina" bind:this={contenitore}>
  <button
    bind:this={pulsante}
    type="button"
    class="campo"
    class:aperta
    role="combobox"
    aria-label={etichettaAccessibile}
    aria-expanded={aperta}
    aria-controls={idElenco}
    aria-haspopup="listbox"
    aria-activedescendant={aperta && indiceEvidenziato >= 0
      ? `${idElenco}-${indiceEvidenziato}`
      : undefined}
    onclick={() => (aperta ? chiudi() : apri())}
    onkeydown={gestisciTasti}
  >
    <span class="testo" class:segnaposto={mostraSegnaposto}>
      {mostraSegnaposto ? segnaposto : scelta?.etichetta}
    </span>
    <span class="freccia"><Icona nome="freccia-giu" dimensione={14} spessore={1.8} /></span>
  </button>

  {#if obbligatoria}
    <!-- Campo nascosto solo per la validazione del modulo: senza scelta l'invio è bloccato -->
    <input
      class="validazione"
      tabindex="-1"
      aria-hidden="true"
      value={mostraSegnaposto ? "" : "scelta"}
      required
      use:validita={mostraSegnaposto ? "Scegli un'opzione" : null}
      onfocus={() => pulsante?.focus()}
    />
  {/if}

  {#if aperta}
    <ul class="elenco" id={idElenco} role="listbox" bind:this={elenco}>
      {#each opzioni as opzione, indice (indice)}
        <!-- La tastiera è gestita dal pulsante della tendina, che indica l'opzione evidenziata -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <li
          id="{idElenco}-{indice}"
          role="option"
          aria-selected={indice === indiceScelto}
          aria-disabled={opzione.disabilitata}
          class:evidenziata={indice === indiceEvidenziato}
          class:scelta={indice === indiceScelto}
          class:disabilitata={opzione.disabilitata}
          onpointerenter={() => !opzione.disabilitata && (indiceEvidenziato = indice)}
          onpointerdown={(evento) => evento.preventDefault()}
          onclick={() => scegli(indice)}
        >
          <span class="spunta"><Icona nome="spunta" dimensione={16} spessore={2} /></span>
          <span class="etichetta">{opzione.etichetta}</span>
          {#if opzione.dettaglio}<span class="dettaglio">{opzione.dettaglio}</span>{/if}
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .tendina {
    position: relative;
    min-width: 0;
  }

  .campo {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    min-height: 2.5rem;
    padding: 0.45rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    background: var(--colore-superficie);
    color: var(--colore-testo);
    box-shadow: var(--ombra-campo);
    text-align: left;
  }

  :global(:root) .campo:hover {
    border-color: var(--colore-bordo-hover);
    filter: none;
  }

  :global(:root) .campo:focus-visible,
  .campo.aperta {
    outline: none;
    border-color: var(--colore-primario);
    box-shadow: 0 0 0 3px var(--colore-focus);
  }

  .testo {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .segnaposto {
    color: var(--colore-testo-secondario);
  }

  .freccia {
    display: flex;
    flex-shrink: 0;
    color: var(--colore-testo-secondario);
    transition: transform 0.15s;
  }

  .aperta .freccia {
    transform: rotate(180deg);
  }

  .validazione {
    position: absolute;
    left: 0;
    bottom: 0;
    width: 100%;
    height: 1px;
    opacity: 0;
    pointer-events: none;
  }

  .elenco {
    position: absolute;
    top: calc(100% + 6px);
    left: 0;
    z-index: 30;
    min-width: 100%;
    max-height: 18rem;
    overflow-y: auto;
    margin: 0;
    padding: 0.3rem;
    list-style: none;
    background: var(--colore-superficie);
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio-grande);
    box-shadow: var(--ombra-sospesa);
  }

  li {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.45rem 0.6rem;
    border-radius: 6px;
    white-space: nowrap;
    cursor: pointer;
  }

  li.evidenziata {
    background: var(--colore-sfondo-tenue);
  }

  li.scelta {
    color: var(--colore-primario);
    font-weight: 500;
  }

  li.disabilitata {
    color: var(--colore-testo-secondario);
    cursor: default;
  }

  .spunta {
    display: flex;
    flex-shrink: 0;
    visibility: hidden;
  }

  li.scelta .spunta {
    visibility: visible;
  }

  .etichetta {
    flex: 1;
  }

  .dettaglio {
    margin-left: 1rem;
    color: var(--colore-testo-secondario);
    font-variant-numeric: tabular-nums;
    font-weight: 400;
  }
</style>
