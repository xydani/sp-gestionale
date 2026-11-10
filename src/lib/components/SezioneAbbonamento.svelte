<script lang="ts">
  import { onMount } from "svelte";
  import { elencaPagamenti, eliminaPagamento, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { confermaEliminazionePagamento } from "$lib/conferma";
  import { formattaData } from "$lib/format";
  import type { Iscritto, Pagamento, PaginaPagamenti } from "$lib/types";
  import BadgeStato from "./BadgeStato.svelte";
  import FormPagamento from "./FormPagamento.svelte";
  import Paginazione from "./Paginazione.svelte";
  import TabellaPagamenti from "./TabellaPagamenti.svelte";

  let { iscritto, onCambiato }: { iscritto: Iscritto; onCambiato: () => void } = $props();

  let storico = $state<PaginaPagamenti>({
    elementi: [],
    numero: 1,
    elementiPerPagina: 50,
    totaleElementi: 0,
    totaleImportoCent: 0,
  });
  let formAperto = $state(false);

  async function carica(numeroPagina = 1) {
    storico = await elencaPagamenti(
      { dal: null, al: null, metodo: null, iscrittoId: iscritto.id },
      numeroPagina,
    );
  }

  async function registrato() {
    formAperto = false;
    mostraAvviso("successo", "Pagamento registrato");
    await carica();
    onCambiato();
  }

  async function elimina(pagamento: Pagamento) {
    if (!(await confermaEliminazionePagamento(pagamento))) return;
    try {
      await eliminaPagamento(pagamento.id);
      mostraAvviso("successo", "Pagamento eliminato");
      await carica(storico.numero);
      onCambiato();
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }

  onMount(() => carica());
</script>

<section class="scheda">
  <div class="intestazione-sezione">
    <h2>Abbonamento</h2>
    <BadgeStato stato={iscritto.statoAbbonamento} />
    <span class="spazio"></span>
    {#if !formAperto && !iscritto.archiviato}
      <button type="button" onclick={() => (formAperto = true)}>
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
    <FormPagamento
      iscrittoIniziale={iscritto}
      onRegistrato={registrato}
      onAnnulla={() => (formAperto = false)}
    />
  {/if}

  <TabellaPagamenti
    pagamenti={storico.elementi}
    totaleElementi={storico.totaleElementi}
    totaleImportoCent={storico.totaleImportoCent}
    mostraIscritto={false}
    onElimina={elimina}
  />
  {#if storico.totaleElementi > storico.elementiPerPagina}
    <Paginazione pagina={storico} onCambia={carica} />
  {/if}
</section>

<style>
  .scheda {
    display: grid;
    gap: 1rem;
    margin-bottom: 1.5rem;
    padding: 1.5rem;
    background: var(--colore-superficie);
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
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

  button {
    padding: 0.55rem 1.1rem;
    border-radius: var(--raggio);
    border: 1px solid var(--colore-primario);
    background: var(--colore-primario);
    color: var(--colore-primario-testo);
  }
</style>
