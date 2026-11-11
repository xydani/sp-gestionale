<script lang="ts">
  import { onMount } from "svelte";
  import { messaggioErrore, trovaIscritto } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { aggiornaConteggioScadenze } from "$lib/promemoria.svelte";
  import type { TipoRinnovabile } from "$lib/rinnovo";
  import type { Iscritto } from "$lib/types";
  import FormIscrizione from "./FormIscrizione.svelte";
  import FormPagamento from "./FormPagamento.svelte";

  /** Rinnovo al volo da un elenco, senza passare dalla scheda dell'iscritto. */
  let {
    iscrittoId,
    tipo,
    onChiusa,
  }: {
    iscrittoId: number;
    tipo: TipoRinnovabile;
    /** `registrato` è vero se il rinnovo è stato salvato: l'elenco va ricaricato. */
    onChiusa: (registrato: boolean) => void;
  } = $props();

  const TITOLI: Record<TipoRinnovabile, string> = {
    abbonamento: "Rinnova abbonamento",
    iscrizione: "Rinnova iscrizione annuale",
  };

  let finestra = $state<HTMLDialogElement>();
  let iscritto = $state<Iscritto | null>(null);

  onMount(async () => {
    finestra?.showModal();
    try {
      iscritto = await trovaIscritto(iscrittoId);
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
      onChiusa(false);
    }
  });

  async function registrato() {
    mostraAvviso("successo", `${TITOLI[tipo]}: fatto per ${iscritto?.nome ?? "l'iscritto"}`);
    await aggiornaConteggioScadenze();
    onChiusa(true);
  }

  /** Esc chiude la finestra come «Annulla». */
  function annullaConEsc(evento: Event) {
    evento.preventDefault();
    onChiusa(false);
  }
</script>

<dialog bind:this={finestra} oncancel={annullaConEsc} aria-label={TITOLI[tipo]}>
  <h2>
    {TITOLI[tipo]}
    {#if iscritto}<span class="nome">· {iscritto.cognome} {iscritto.nome}</span>{/if}
  </h2>
  {#if iscritto && tipo === "abbonamento"}
    <FormPagamento
      iscrittoIniziale={iscritto}
      onRegistrato={registrato}
      onAnnulla={() => onChiusa(false)}
    />
  {:else if iscritto}
    <FormIscrizione
      iscrittoId={iscritto.id}
      onRegistrata={registrato}
      onAnnulla={() => onChiusa(false)}
    />
  {/if}
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

  dialog::backdrop {
    background: rgb(24 24 27 / 0.35);
  }

  h2 {
    margin: 0 0 1rem;
    font-size: 1.15rem;
  }

  .nome {
    color: var(--colore-testo-secondario);
    font-weight: 500;
  }
</style>
