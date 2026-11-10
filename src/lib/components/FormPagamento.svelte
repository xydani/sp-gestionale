<script lang="ts">
  import { onMount, untrack } from "svelte";
  import { elencaTipologie, messaggioErrore, proponiPagamento, registraPagamento } from "$lib/api";
  import { centesimiInTestoEuro, euroInCentesimi, oggiIso } from "$lib/format";
  import { iscrizioneNonValida } from "$lib/iscrizione";
  import { ETICHETTE_METODO, METODI_PAGAMENTO } from "$lib/metodi-pagamento";
  import type { Iscritto, MetodoPagamento, Pagamento, TipologiaAbbonamento } from "$lib/types";
  import RicercaIscritto from "./RicercaIscritto.svelte";

  let {
    iscrittoIniziale,
    onRegistrato,
    onAnnulla,
  }: {
    /** Se indicato, l'iscritto è già scelto (es. dalla scheda iscritto). */
    iscrittoIniziale?: Iscritto;
    onRegistrato: (pagamento: Pagamento) => void;
    onAnnulla: () => void;
  } = $props();

  let iscritto = $state<Iscritto | null>(untrack(() => iscrittoIniziale) ?? null);
  let tipologie = $state<TipologiaAbbonamento[]>([]);
  let tipologiaId = $state<number | null>(null);
  let importo = $state("");
  let dataPagamento = $state(oggiIso());
  let metodo = $state<MetodoPagamento>("contanti");
  let dataInizio = $state("");
  let dataScadenza = $state("");
  let note = $state("");
  let rinnovoDiId = $state<number | null>(null);
  let errore = $state<string | null>(null);
  let inCorso = $state(false);

  onMount(async () => {
    tipologie = await elencaTipologie(true);
  });

  /** Importo, date e collegamento al rinnovo proposti dal programma. */
  async function applicaProposta(dataInizioScelta: string | null = null) {
    if (!iscritto || tipologiaId === null) return;
    try {
      const proposta = await proponiPagamento(iscritto.id, tipologiaId, dataInizioScelta);
      if (dataInizioScelta === null) {
        importo = centesimiInTestoEuro(proposta.importoCent);
        rinnovoDiId = proposta.rinnovoDiId;
      }
      dataInizio = proposta.dataInizio;
      dataScadenza = proposta.dataScadenza;
      errore = null;
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
    }
  }

  function scegliIscritto(scelto: Iscritto) {
    iscritto = scelto;
    applicaProposta();
  }

  function cambiaDataInizio() {
    if (dataInizio) applicaProposta(dataInizio);
  }

  async function conferma(evento: SubmitEvent) {
    evento.preventDefault();
    if (!iscritto || tipologiaId === null) {
      errore = "Scegli l'iscritto e la tipologia di abbonamento";
      return;
    }
    const importoCent = euroInCentesimi(importo);
    if (importoCent === null) {
      errore = "Importo non valido (esempio: 45 oppure 45,50)";
      return;
    }
    inCorso = true;
    try {
      const pagamento = await registraPagamento({
        iscrittoId: iscritto.id,
        tipologiaId,
        importoCent,
        dataPagamento,
        metodo,
        dataInizio,
        dataScadenza,
        note: note || null,
        rinnovoDiId,
      });
      onRegistrato(pagamento);
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
    } finally {
      inCorso = false;
    }
  }
</script>

<form onsubmit={conferma}>
  <div class="campo-intero">
    <span class="etichetta">Iscritto</span>
    {#if iscritto}
      <div class="iscritto-scelto">
        <strong>{iscritto.cognome} {iscritto.nome}</strong>
        {#if !iscrittoIniziale}
          <button type="button" class="collegamento" onclick={() => (iscritto = null)}
            >Cambia</button
          >
        {/if}
      </div>
    {:else}
      <RicercaIscritto onScelto={scegliIscritto} />
    {/if}
  </div>

  {#if iscritto && iscrizioneNonValida(iscritto)}
    <p class="avviso" role="status">
      Attenzione: l'iscrizione annuale di {iscritto.nome} è
      {iscritto.statoIscrizione === "scaduto" ? "scaduta" : "assente"}. Puoi registrare
      l'abbonamento, ma ricordati di rinnovare l'iscrizione dalla scheda.
    </p>
  {/if}

  <div class="griglia">
    <label>
      Tipologia
      <select bind:value={tipologiaId} onchange={() => applicaProposta()} required>
        <option value={null} disabled>Scegli…</option>
        {#each tipologie as tipologia (tipologia.id)}
          <option value={tipologia.id}>{tipologia.nome}</option>
        {/each}
      </select>
    </label>
    <label>
      Importo (€)
      <input bind:value={importo} inputmode="decimal" required />
    </label>
    <label>
      Metodo
      <select bind:value={metodo}>
        {#each METODI_PAGAMENTO as valore (valore)}
          <option value={valore}>{ETICHETTE_METODO[valore]}</option>
        {/each}
      </select>
    </label>
    <label>
      Data pagamento
      <input type="date" bind:value={dataPagamento} required />
    </label>
    <label>
      Inizio abbonamento
      <input type="date" bind:value={dataInizio} onchange={cambiaDataInizio} required />
    </label>
    <label>
      Scadenza
      <input type="date" bind:value={dataScadenza} min={dataInizio} required />
    </label>
  </div>

  <label>
    Note
    <input bind:value={note} placeholder="Facoltative" />
  </label>

  {#if rinnovoDiId !== null}
    <p class="informazione">
      Rinnovo: il nuovo periodo parte dopo la scadenza attuale, così i giorni rimasti non si
      perdono.
    </p>
  {/if}
  {#if errore}<p class="errore" role="alert">{errore}</p>{/if}

  <div class="azioni">
    <button type="submit" disabled={inCorso}>Registra pagamento</button>
    <button type="button" class="secondario" onclick={onAnnulla}>Annulla</button>
  </div>
</form>

<style>
  form {
    display: grid;
    gap: 1rem;
  }

  .griglia {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 1rem;
  }

  label,
  .campo-intero {
    display: grid;
    gap: 0.35rem;
    font-weight: 500;
    font-size: 0.9rem;
  }

  input,
  select {
    padding: 0.55rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
    font-weight: 400;
    background: var(--colore-superficie);
  }

  .iscritto-scelto {
    display: flex;
    gap: 1rem;
    align-items: center;
    font-weight: 400;
  }

  .collegamento {
    border: none;
    background: none;
    color: var(--colore-primario);
    padding: 0;
  }

  .informazione {
    margin: 0;
    color: var(--colore-testo-secondario);
    font-size: 0.9rem;
  }

  .errore {
    margin: 0;
    color: var(--colore-errore);
  }

  .avviso {
    margin: 0;
    padding: 0.6rem 0.8rem;
    border-radius: var(--raggio);
    background: #fff6e5;
    color: var(--colore-avviso);
  }

  .azioni {
    display: flex;
    gap: 0.5rem;
  }

  .azioni button {
    padding: 0.55rem 1.1rem;
    border-radius: var(--raggio);
    border: 1px solid var(--colore-primario);
    background: var(--colore-primario);
    color: var(--colore-primario-testo);
  }

  .azioni button.secondario {
    background: transparent;
    color: var(--colore-primario);
  }
</style>
