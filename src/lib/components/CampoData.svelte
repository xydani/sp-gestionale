<script lang="ts">
  import {
    erroreData,
    formattaMentreSiScrive,
    isoInTestoItaliano,
    testoItalianoInIso,
  } from "$lib/data-italiana";
  import CalendarioMese from "./CalendarioMese.svelte";

  /**
   * Campo data nel formato italiano GG/MM/AAAA, indipendente dalla lingua del sistema.
   * `valore` è la data ISO (AAAA-MM-GG), oppure "" se il campo è vuoto o non valido.
   */
  let {
    valore = $bindable(""),
    obbligatoria = false,
    minimaIso = null,
    massimaIso = null,
    onCambia,
  }: {
    valore?: string;
    obbligatoria?: boolean;
    minimaIso?: string | null;
    massimaIso?: string | null;
    /** Chiamata quando l'utente completa una data valida, con la data ISO. */
    onCambia?: (dataIso: string) => void;
  } = $props();

  let testo = $state(isoInTestoItaliano(valore));
  let campo = $state<HTMLInputElement>();

  // Se il valore cambia dall'esterno (es. proposta del programma) si aggiorna il testo
  $effect(() => {
    if (valore !== (testoItalianoInIso(testo) ?? "")) {
      testo = isoInTestoItaliano(valore);
    }
  });

  const errore = $derived(erroreData(testo, { obbligatoria, minimaIso, massimaIso }));

  // Il messaggio blocca l'invio del modulo finché la data non è corretta
  $effect(() => {
    campo?.setCustomValidity(errore ?? "");
  });

  let calendarioAperto = $state(false);
  let contenitore = $state<HTMLDivElement>();

  function scegliDalCalendario(dataIso: string) {
    calendarioAperto = false;
    testo = isoInTestoItaliano(dataIso);
    const precedente = valore;
    valore = dataIso;
    if (dataIso !== precedente) onCambia?.(dataIso);
    campo?.focus();
  }

  /** Il calendario si chiude cliccando fuori o con Esc. */
  function chiudiSeFuori(evento: PointerEvent) {
    if (calendarioAperto && !contenitore?.contains(evento.target as Node)) {
      calendarioAperto = false;
    }
  }

  function gestisciTasti(evento: KeyboardEvent) {
    if (evento.key === "Escape" && calendarioAperto) {
      evento.stopPropagation();
      calendarioAperto = false;
      campo?.focus();
    }
  }

  function scrivi(evento: Event) {
    const input = evento.currentTarget as HTMLInputElement;
    testo = formattaMentreSiScrive(input.value);
    input.value = testo;
    const dataIso = testoItalianoInIso(testo);
    const precedente = valore;
    valore = dataIso ?? "";
    if (dataIso && dataIso !== precedente) onCambia?.(dataIso);
  }
</script>

<svelte:window onpointerdown={chiudiSeFuori} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="campo" bind:this={contenitore} onkeydown={gestisciTasti}>
  <input
    bind:this={campo}
    type="text"
    inputmode="numeric"
    placeholder="GG/MM/AAAA"
    maxlength="10"
    value={testo}
    oninput={scrivi}
    required={obbligatoria}
    title={errore ?? ""}
    class:non-valida={testo.length === 10 && errore !== null}
  />
  <button
    type="button"
    class="apri-calendario"
    aria-label="Apri il calendario"
    title="Scegli dal calendario"
    aria-expanded={calendarioAperto}
    onclick={() => (calendarioAperto = !calendarioAperto)}
  >
    <svg viewBox="0 0 24 24" width="18" height="18" aria-hidden="true">
      <rect x="3.5" y="5" width="17" height="15.5" rx="2.5" />
      <path d="M3.5 10h17M8 3v4M16 3v4" />
    </svg>
  </button>
  {#if calendarioAperto}
    <div class="riquadro-calendario">
      <CalendarioMese
        selezionata={valore}
        {minimaIso}
        {massimaIso}
        onScegli={scegliDalCalendario}
      />
    </div>
  {/if}
</div>

<style>
  .campo {
    position: relative;
    width: 100%;
  }

  .apri-calendario {
    position: absolute;
    top: 50%;
    right: 0.3rem;
    transform: translateY(-50%);
    display: grid;
    place-items: center;
    width: 2rem;
    height: 2rem;
    border: none;
    border-radius: 6px;
    background: transparent;
    color: var(--colore-testo-secondario);
  }

  .apri-calendario:hover,
  .apri-calendario[aria-expanded="true"] {
    background: var(--colore-primario-tenue);
    color: var(--colore-primario);
  }

  .apri-calendario svg {
    fill: none;
    stroke: currentColor;
    stroke-width: 1.8;
    stroke-linecap: round;
  }

  .riquadro-calendario {
    position: absolute;
    top: calc(100% + 6px);
    left: 0;
    z-index: 20;
  }

  input {
    width: 100%;
    padding: 0.55rem 2.5rem 0.55rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
    font-weight: 400;
    background: var(--colore-superficie);
    font-variant-numeric: tabular-nums;
  }

  .non-valida {
    border-color: var(--colore-errore);
  }
</style>
