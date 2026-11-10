<script lang="ts">
  import Tendina from "./Tendina.svelte";
  import CampoData from "./CampoData.svelte";
  import { onMount, untrack } from "svelte";
  import { elencaTipologie, messaggioErrore, proponiPagamento, registraPagamento } from "$lib/api";
  import { centesimiInTestoEuro, euroInCentesimi, formattaEuro, oggiIso } from "$lib/format";
  import { iscrizioneNonValida } from "$lib/iscrizione";
  import { OPZIONI_METODO } from "$lib/metodi-pagamento";
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

  function cambiaDataInizio(nuovaDataInizio: string) {
    applicaProposta(nuovaDataInizio);
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
          <button type="button" class="pulsante-testo" onclick={() => (iscritto = null)}
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
      <Tendina
        bind:valore={tipologiaId}
        opzioni={tipologie.map((tipologia) => ({
          valore: tipologia.id,
          etichetta: tipologia.nome,
          dettaglio: formattaEuro(tipologia.prezzoCent),
        }))}
        obbligatoria
        onCambia={() => applicaProposta()}
      />
    </label>
    <label>
      Importo (€)
      <input bind:value={importo} inputmode="decimal" required />
    </label>
    <label>
      Metodo
      <Tendina bind:valore={metodo} opzioni={OPZIONI_METODO} />
    </label>
    <label>
      Data pagamento
      <CampoData bind:valore={dataPagamento} obbligatoria />
    </label>
    <label>
      Inizio abbonamento
      <CampoData bind:valore={dataInizio} onCambia={cambiaDataInizio} obbligatoria />
    </label>
    <label>
      Scadenza
      <CampoData bind:valore={dataScadenza} minimaIso={dataInizio} obbligatoria />
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
    <button class="pulsante" type="submit" disabled={inCorso}>Registra pagamento</button>
    <button type="button" class="pulsante secondario" onclick={onAnnulla}>Annulla</button>
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

  input {
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
</style>
