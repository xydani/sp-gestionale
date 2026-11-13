<script lang="ts">
  import { eliminaConAnnulla } from "$lib/eliminazione-differita";
  import { onMount } from "svelte";
  import { elencaIscrizioni, eliminaIscrizione, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { aggiornaConteggioScadenze } from "$lib/promemoria.svelte";
  import { formattaData, formattaEuro } from "$lib/format";
  import { ETICHETTE_METODO } from "$lib/metodi-pagamento";
  import type { Iscritto, Iscrizione } from "$lib/types";
  import BadgeStato from "./BadgeStato.svelte";
  import FormIscrizione from "./FormIscrizione.svelte";

  let {
    iscritto,
    onCambiata,
    formAperto = $bindable(false),
  }: {
    iscritto: Iscritto;
    onCambiata: () => void;
    /** Aperto anche dalla striscia «Da sistemare» della scheda. */
    formAperto?: boolean;
  } = $props();

  let iscrizioni = $state<Iscrizione[]>([]);
  /** Lo storico delle quote serve di rado: resta chiuso per lasciare spazio all'abbonamento. */
  let storicoAperto = $state(false);

  async function carica() {
    iscrizioni = await elencaIscrizioni(iscritto.id);
  }

  async function registrata() {
    formAperto = false;
    mostraAvviso("successo", "Iscrizione registrata");
    await carica();
    onCambiata();
    aggiornaConteggioScadenze();
  }

  /** Sparisce subito; si cancella davvero dopo qualche secondo, se non si preme «Annulla». */
  function elimina(iscrizione: Iscrizione) {
    iscrizioni = iscrizioni.filter((elemento) => elemento.id !== iscrizione.id);
    eliminaConAnnulla({
      testo: "Iscrizione eliminata",
      elimina: () => eliminaIscrizione(iscrizione.id),
      onEliminato: aggiornaDopoEliminazione,
      onRipristinato: carica,
      messaggioErrore,
    });
  }

  async function aggiornaDopoEliminazione() {
    await carica();
    onCambiata();
    aggiornaConteggioScadenze();
  }

  onMount(carica);
</script>

<section class="scheda" id="sezione-iscrizione">
  <div class="intestazione-sezione">
    <h2>Iscrizione annuale</h2>
    <BadgeStato stato={iscritto.statoIscrizione} />
    <span class="spazio"></span>
    {#if !formAperto && !iscritto.archiviato}
      <button class="pulsante" type="button" onclick={() => (formAperto = true)}>
        {iscrizioni.length > 0 ? "Rinnova iscrizione" : "Iscrivi"}
      </button>
    {/if}
  </div>

  <div class="riepilogo">
    <span>
      {#if iscritto.ultimaDataFineIscrizione}
        Valida fino al <strong>{formattaData(iscritto.ultimaDataFineIscrizione)}</strong>
      {:else}
        Nessuna iscrizione registrata
      {/if}
    </span>
    {#if iscrizioni.length > 0}
      <button
        type="button"
        class="pulsante-testo"
        aria-expanded={storicoAperto}
        onclick={() => (storicoAperto = !storicoAperto)}
      >
        {storicoAperto ? "Nascondi lo storico" : `Storico delle quote (${iscrizioni.length})`}
      </button>
    {/if}
  </div>

  {#if formAperto}
    <FormIscrizione
      iscrittoId={iscritto.id}
      onRegistrata={registrata}
      onAnnulla={() => (formAperto = false)}
    />
  {/if}

  {#if storicoAperto && iscrizioni.length > 0}
    <table class="tabella compatta">
      <thead>
        <tr
          ><th>Periodo</th><th>Pagata il</th><th>Metodo</th><th class="importo">Quota</th><th
          ></th></tr
        >
      </thead>
      <tbody>
        {#each iscrizioni as iscrizione (iscrizione.id)}
          <tr>
            <td>{formattaData(iscrizione.dataInizio)} – {formattaData(iscrizione.dataFine)}</td>
            {#if iscrizione.pregresso}
              <td colspan="3" class="pregresso">Situazione iniziale: pagata prima del programma</td>
            {:else}
              <td>{formattaData(iscrizione.dataPagamento)}</td>
              <td>{ETICHETTE_METODO[iscrizione.metodo]}</td>
              <td class="importo">{formattaEuro(iscrizione.importoCent)}</td>
            {/if}
            <td class="azioni">
              <button type="button" class="elimina" onclick={() => elimina(iscrizione)}>
                Elimina
              </button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</section>

<style>
  .pregresso {
    color: var(--colore-testo-secondario);
  }

  /* Più raccolta delle schede normali: la scheda iscritto deve stare nella finestra */
  .scheda {
    display: grid;
    gap: 0.75rem;
    margin-bottom: 1rem;
    padding: 1.1rem 1.25rem;
    background: var(--colore-superficie);
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio-grande);
    box-shadow: var(--ombra-scheda);
  }

  .intestazione-sezione {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .spazio {
    flex: 1;
  }

  h2 {
    margin: 0;
    font-size: 1.15rem;
  }

  .riepilogo {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    gap: 0.75rem;
    color: var(--colore-testo-secondario);
  }

  .importo {
    text-align: right;
  }

  .azioni {
    text-align: right;
  }

  button.elimina {
    padding: 0.2rem 0.4rem;
    border: none;
    background: none;
    color: var(--colore-errore);
    opacity: 0.7;
  }
</style>
