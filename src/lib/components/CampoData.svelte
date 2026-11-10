<script lang="ts">
  import {
    erroreData,
    formattaMentreSiScrive,
    isoInTestoItaliano,
    testoItalianoInIso,
  } from "$lib/data-italiana";

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

<style>
  input {
    width: 100%;
    padding: 0.55rem 0.75rem;
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
