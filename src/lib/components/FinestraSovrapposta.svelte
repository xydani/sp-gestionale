<script lang="ts">
  import { onMount, type Snippet } from "svelte";

  /** Modulo in primo piano sopra la pagina: si compila senza allungarla né scorrerla. */
  let {
    titolo,
    sottotitolo = null,
    larghezza = "normale",
    onChiusa,
    children,
  }: {
    titolo: string;
    /** Di solito il nome dell'iscritto, accanto al titolo. */
    sottotitolo?: string | null;
    /** «larga» per i moduli su due colonne, «stretta» per quelli di pochi campi. */
    larghezza?: "stretta" | "normale" | "larga";
    /** Chiamata con Esc; i pulsanti del modulo chiudono da sé. */
    onChiusa: () => void;
    children: Snippet;
  } = $props();

  let finestra = $state<HTMLDialogElement>();

  onMount(() => finestra?.showModal());

  /** Esc chiude la finestra come «Annulla». */
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
</script>

<dialog
  bind:this={finestra}
  class={larghezza}
  oncancel={annullaConEsc}
  onkeydown={chiudiConEsc}
  aria-label={titolo}
>
  <h2>
    {titolo}
    {#if sottotitolo}<span class="sottotitolo">· {sottotitolo}</span>{/if}
  </h2>
  {@render children()}
</dialog>

<style>
  dialog {
    width: min(48rem, calc(100vw - 3rem));
    padding: 1.5rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio-grande);
    background: var(--colore-superficie);
    color: var(--colore-testo);
    box-shadow: var(--ombra-sospesa);
  }

  dialog.stretta {
    width: min(32rem, calc(100vw - 3rem));
  }

  dialog.larga {
    width: min(76rem, calc(100vw - 3rem));
  }

  dialog::backdrop {
    background: rgb(24 24 27 / 0.35);
  }

  h2 {
    margin: 0 0 1rem;
    font-size: 1.15rem;
  }

  .sottotitolo {
    color: var(--colore-testo-secondario);
    font-weight: 500;
  }
</style>
