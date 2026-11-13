<script lang="ts">
  import { onMount } from "svelte";
  import { elencaPagamenti, eliminaPagamento, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { aggiornaConteggioScadenze } from "$lib/promemoria.svelte";
  import { eliminaConAnnulla } from "$lib/eliminazione-differita";
  import { formattaData } from "$lib/format";
  import { seguiFinestra } from "$lib/righe-visibili";
  import type { Iscritto, Pagamento, PaginaPagamenti } from "$lib/types";
  import BadgeStato from "./BadgeStato.svelte";
  import FinestraSovrapposta from "./FinestraSovrapposta.svelte";
  import FormPagamento from "./FormPagamento.svelte";
  import Paginazione from "./Paginazione.svelte";
  import TabellaPagamenti from "./TabellaPagamenti.svelte";

  let {
    iscritto,
    onCambiato,
    formAperto = $bindable(false),
  }: {
    iscritto: Iscritto;
    onCambiato: () => void;
    /** Aperto anche dalla striscia «Da sistemare» della scheda. */
    formAperto?: boolean;
  } = $props();

  let storico = $state<PaginaPagamenti>({
    elementi: [],
    numero: 1,
    elementiPerPagina: 50,
    totaleElementi: 0,
    totaleImportoCent: 0,
  });

  /** Sotto lo storico: le pagine, il bordo della scheda e il margine della pagina. */
  const SPAZIO_SOTTO_PX = 85;
  let tabella = $state<HTMLTableElement>();
  /** Righe che stanno nella finestra; null finché non sono misurate. */
  let righePerPagina = $state<number | null>(null);
  let caricato = $state(false);

  async function carica(numeroPagina = 1) {
    storico = await elencaPagamenti(
      { dal: null, al: null, metodo: null, iscrittoId: iscritto.id },
      numeroPagina,
      righePerPagina ?? undefined,
    );
    caricato = true;
  }

  async function registrato() {
    formAperto = false;
    mostraAvviso("successo", "Pagamento registrato");
    await carica();
    onCambiato();
    aggiornaConteggioScadenze();
  }

  /** Sparisce subito; si cancella davvero dopo qualche secondo, se non si preme «Annulla». */
  function elimina(pagamento: Pagamento) {
    storico.elementi = storico.elementi.filter((elemento) => elemento.id !== pagamento.id);
    eliminaConAnnulla({
      testo: "Pagamento eliminato",
      elimina: () => eliminaPagamento(pagamento.id),
      onEliminato: aggiornaDopoEliminazione,
      onRipristinato: () => carica(storico.numero),
      messaggioErrore,
    });
  }

  async function aggiornaDopoEliminazione() {
    await carica(storico.numero);
    onCambiato();
    aggiornaConteggioScadenze();
  }

  // Lo storico si carica appena si sa quante righe stanno nella finestra
  onMount(() => {
    if (!tabella) return;
    return seguiFinestra(tabella, SPAZIO_SOTTO_PX, {
      numeroPagina: () => storico.numero,
      carica: (numero, righe) => {
        righePerPagina = righe;
        carica(numero);
      },
    });
  });
</script>

<section class="scheda" id="sezione-abbonamento">
  <div class="intestazione-sezione">
    <h2>Abbonamento</h2>
    <BadgeStato stato={iscritto.statoAbbonamento} />
    <span class="spazio"></span>
    {#if !iscritto.archiviato}
      <button class="pulsante" type="button" onclick={() => (formAperto = true)}>
        {storico.totaleElementi > 0 ? "Rinnova abbonamento" : "Primo abbonamento"}
      </button>
    {/if}
  </div>

  <p class="riepilogo">
    {#if iscritto.ultimaDataScadenzaAbbonamento}
      Scade il <strong>{formattaData(iscritto.ultimaDataScadenzaAbbonamento)}</strong>
    {:else}
      Nessun abbonamento registrato
    {/if}
  </p>

  {#if formAperto}
    <FinestraSovrapposta
      titolo={storico.totaleElementi > 0 ? "Rinnova abbonamento" : "Primo abbonamento"}
      sottotitolo="{iscritto.cognome} {iscritto.nome}"
      onChiusa={() => (formAperto = false)}
    >
      <FormPagamento
        iscrittoIniziale={iscritto}
        onRegistrato={registrato}
        onAnnulla={() => (formAperto = false)}
      />
    </FinestraSovrapposta>
  {/if}

  <TabellaPagamenti
    bind:tabella
    compatta
    mostraDataPagamento={false}
    pagamenti={storico.elementi}
    totaleElementi={storico.totaleElementi}
    totaleImportoCent={storico.totaleImportoCent}
    mostraIscritto={false}
    onElimina={elimina}
  />
  {#if caricato && storico.totaleElementi > storico.elementiPerPagina}
    <Paginazione pagina={storico} onCambia={carica} />
  {/if}
</section>

<style>
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
    margin: 0;
    color: var(--colore-testo-secondario);
  }
</style>
