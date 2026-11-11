<script lang="ts">
  import Tendina from "$lib/components/Tendina.svelte";
  import { resolve } from "$app/paths";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { elencaIscritti, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import BadgeStato from "$lib/components/BadgeStato.svelte";
  import FormIscrittoGiaAttivo from "$lib/components/FormIscrittoGiaAttivo.svelte";
  import FormNuovoIscritto from "$lib/components/FormNuovoIscritto.svelte";
  import Paginazione from "$lib/components/Paginazione.svelte";
  import { formattaData } from "$lib/format";
  import { filtriIscritti, senzaFiltriIscritti } from "$lib/filtri.svelte";
  import { ritarda } from "$lib/ritarda";
  import { opzioniStato } from "$lib/stato-validita";
  import type { Iscritto, Pagina } from "$lib/types";

  const PAUSA_RICERCA_MS = 200;

  let pagina = $state<Pagina<Iscritto>>({
    elementi: [],
    numero: 1,
    elementiPerPagina: 50,
    totaleElementi: 0,
  });
  /** Un solo modulo di inserimento aperto alla volta. */
  let moduloAperto = $state<"nuovo" | "gia-iscritto" | null>(null);
  let caricamento = $state(true);

  // Conta i caricamenti: una risposta lenta non deve sovrascrivere quella di una ricerca successiva
  let ultimoCaricamento = 0;

  /** Carica una pagina; cambiando i filtri si riparte dalla prima. */
  async function carica(numeroPagina = 1) {
    const questoCaricamento = ++ultimoCaricamento;
    try {
      const risultato = await elencaIscritti(
        {
          testo: filtriIscritti.testo,
          includiArchiviati: filtriIscritti.includiArchiviati,
          statoAbbonamento: filtriIscritti.statoAbbonamento || null,
          statoIscrizione: filtriIscritti.statoIscrizione || null,
          statoCertificato: filtriIscritti.statoCertificato || null,
          soloDaCompletare: filtriIscritti.soloDaCompletare,
        },
        numeroPagina,
      );
      if (questoCaricamento === ultimoCaricamento) pagina = risultato;
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    } finally {
      caricamento = false;
    }
  }

  const caricaDopoPausa = ritarda(() => carica(), PAUSA_RICERCA_MS);

  async function apriNuovoIscritto(creato: Iscritto) {
    mostraAvviso("successo", `${creato.nome} ${creato.cognome} iscritto, pagamento registrato`);
    moduloAperto = null;
    await goto(resolve("/iscritti/[id]", { id: String(creato.id) }));
  }

  async function chiudiGiaIscritto(inserito: Iscritto) {
    mostraAvviso("successo", `${inserito.nome} ${inserito.cognome} inserito`);
    moduloAperto = null;
    await carica();
  }

  /** Chiudendo l'inserimento rapido l'elenco mostra anche chi è stato appena aggiunto. */
  async function annullaGiaIscritto() {
    moduloAperto = null;
    await carica();
  }

  onMount(() => carica());
</script>

<div class="intestazione">
  <h1>Iscritti</h1>
  {#if moduloAperto === null}
    <div class="pulsanti">
      <button
        type="button"
        class="pulsante secondario"
        onclick={() => (moduloAperto = "gia-iscritto")}
      >
        + Aggiungi già iscritto
      </button>
      <button type="button" class="pulsante" onclick={() => (moduloAperto = "nuovo")}>
        + Nuovo iscritto
      </button>
    </div>
  {/if}
</div>

{#if moduloAperto === "nuovo"}
  <section class="pannello">
    <h2>Nuovo iscritto</h2>
    <FormNuovoIscritto onIscritto={apriNuovoIscritto} onAnnulla={() => (moduloAperto = null)} />
  </section>
{:else if moduloAperto === "gia-iscritto"}
  <section class="pannello">
    <h2>Aggiungi già iscritto</h2>
    <FormIscrittoGiaAttivo onSalvato={chiudiGiaIscritto} onAnnulla={annullaGiaIscritto} />
  </section>
{/if}

<div class="filtri">
  <input
    class="ricerca"
    type="search"
    placeholder="Cerca per nome o cognome…"
    bind:value={filtriIscritti.testo}
    oninput={caricaDopoPausa}
  />
  <div class="filtro-stato">
    <Tendina
      bind:valore={filtriIscritti.statoIscrizione}
      opzioni={opzioniStato("Iscrizione", "tutte")}
      etichettaAccessibile="Stato iscrizione"
      onCambia={() => carica()}
    />
  </div>
  <div class="filtro-stato">
    <Tendina
      bind:valore={filtriIscritti.statoAbbonamento}
      opzioni={opzioniStato("Abbonamento", "tutti")}
      etichettaAccessibile="Stato abbonamento"
      onCambia={() => carica()}
    />
  </div>
  <div class="filtro-stato">
    <Tendina
      bind:valore={filtriIscritti.statoCertificato}
      opzioni={opzioniStato("Certificato", "tutti")}
      etichettaAccessibile="Stato del certificato medico"
      onCambia={() => carica()}
    />
  </div>
  <label class="spunta">
    <input
      type="checkbox"
      bind:checked={filtriIscritti.includiArchiviati}
      onchange={() => carica()}
    />
    Mostra archiviati
  </label>
  <label class="spunta">
    <input
      type="checkbox"
      bind:checked={filtriIscritti.soloDaCompletare}
      onchange={() => carica()}
    />
    Solo da completare
  </label>
</div>

{#if !caricamento}
  <Paginazione {pagina} onCambia={carica} />
{/if}

<table class="tabella riquadrata">
  <thead>
    <tr>
      <th>Cognome e nome</th>
      <th>Telefono</th>
      <th>Iscrizione</th>
      <th>Abbonamento</th>
      <th>Scadenza abbonamento</th>
    </tr>
  </thead>
  <tbody>
    {#each pagina.elementi as iscritto (iscritto.id)}
      <tr
        class:archiviato={iscritto.archiviato}
        onclick={() => goto(resolve("/iscritti/[id]", { id: String(iscritto.id) }))}
      >
        <td>
          <a href={resolve("/iscritti/[id]", { id: String(iscritto.id) })}
            >{iscritto.cognome} {iscritto.nome}</a
          >
          {#if iscritto.archiviato}<span class="etichetta-archiviato">archiviato</span>{/if}
          {#if iscritto.daCompletare}<span class="etichetta-da-completare">da completare</span>{/if}
        </td>
        <td>{iscritto.telefono ?? "—"}</td>
        <td><BadgeStato stato={iscritto.statoIscrizione} /></td>
        <td><BadgeStato stato={iscritto.statoAbbonamento} /></td>
        <td>
          {iscritto.ultimaDataScadenzaAbbonamento
            ? formattaData(iscritto.ultimaDataScadenzaAbbonamento)
            : "—"}
        </td>
      </tr>
    {:else}
      {#if !caricamento}
        <tr>
          <td colspan="5" class="vuoto">
            {#if !senzaFiltriIscritti(filtriIscritti)}
              Nessun iscritto con questa ricerca o questi filtri.
            {:else}
              <strong>Nessun iscritto ancora.</strong><br />
              Se la palestra è già avviata, inizia da «Aggiungi già iscritto»: bastano nome e scadenza
              dell'abbonamento. Per una persona che si iscrive adesso usa «Nuovo iscritto».
            {/if}
          </td>
        </tr>
      {/if}
    {/each}
  </tbody>
</table>

{#if pagina.totaleElementi > pagina.elementiPerPagina}
  <Paginazione {pagina} onCambia={carica} />
{/if}

<style>
  .filtro-stato {
    width: 15rem;
  }

  .intestazione {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .pannello {
    display: grid;
    gap: 1rem;
    margin-bottom: 1.5rem;
    padding: 1.5rem;
    background: var(--colore-superficie);
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio-grande);
    box-shadow: var(--ombra-scheda);
  }

  h2 {
    margin: 0;
    font-size: 1.15rem;
  }

  .filtri {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }

  .ricerca {
    flex: 1 1 280px;
  }

  .filtri input:not([type="checkbox"]) {
    padding: 0.55rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
    background: var(--colore-superficie);
  }

  .spunta {
    display: flex;
    gap: 0.4rem;
    align-items: center;
  }

  tbody tr {
    cursor: pointer;
  }

  tbody tr:hover {
    background: var(--colore-sfondo-tenue);
  }

  tr.archiviato td {
    color: var(--colore-testo-secondario);
  }

  a {
    color: inherit;
    text-decoration: none;
    font-weight: 500;
  }

  .etichetta-archiviato {
    margin-left: 0.5rem;
    font-size: 0.8rem;
    color: var(--colore-testo-secondario);
  }

  .pulsanti {
    display: flex;
    gap: 0.5rem;
  }

  .etichetta-da-completare {
    margin-left: 0.5rem;
    padding: 0.1rem 0.45rem;
    border-radius: var(--raggio);
    font-size: 0.75rem;
    background: var(--colore-avviso-tenue);
    color: var(--colore-avviso);
  }

  .vuoto {
    text-align: center;
    color: var(--colore-testo-secondario);
    padding: 2rem;
  }
</style>
