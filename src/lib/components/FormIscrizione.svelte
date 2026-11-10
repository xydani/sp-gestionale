<script lang="ts">
  import CampoData from "./CampoData.svelte";
  import { onMount } from "svelte";
  import { messaggioErrore, proponiIscrizione, registraIscrizione } from "$lib/api";
  import { centesimiInTestoEuro, euroInCentesimi, oggiIso } from "$lib/format";
  import { ETICHETTE_METODO, METODI_PAGAMENTO } from "$lib/metodi-pagamento";
  import type { MetodoPagamento } from "$lib/types";

  let {
    iscrittoId,
    onRegistrata,
    onAnnulla,
  }: {
    iscrittoId: number;
    onRegistrata: () => void;
    onAnnulla: () => void;
  } = $props();

  let quota = $state("");
  let dataPagamento = $state(oggiIso());
  let metodo = $state<MetodoPagamento>("contanti");
  let dataInizio = $state("");
  let dataFine = $state("");
  let note = $state("");
  let errore = $state<string | null>(null);
  let inCorso = $state(false);

  onMount(async () => {
    try {
      const proposta = await proponiIscrizione(iscrittoId);
      quota = centesimiInTestoEuro(proposta.importoCent);
      dataInizio = proposta.dataInizio;
      dataFine = proposta.dataFine;
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
    }
  });

  async function conferma(evento: SubmitEvent) {
    evento.preventDefault();
    const importoCent = euroInCentesimi(quota);
    if (importoCent === null) {
      errore = "Quota non valida (esempio: 30 oppure 30,50)";
      return;
    }
    inCorso = true;
    try {
      await registraIscrizione({
        iscrittoId,
        importoCent,
        dataPagamento,
        metodo,
        dataInizio,
        dataFine,
        note: note || null,
      });
      onRegistrata();
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
    } finally {
      inCorso = false;
    }
  }
</script>

<form onsubmit={conferma}>
  <div class="griglia">
    <label>
      Valida dal
      <CampoData bind:valore={dataInizio} obbligatoria />
    </label>
    <label>
      Al
      <CampoData bind:valore={dataFine} minimaIso={dataInizio} obbligatoria />
    </label>
    <label>
      Quota (€)
      <input bind:value={quota} inputmode="decimal" required />
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
      <CampoData bind:valore={dataPagamento} obbligatoria />
    </label>
  </div>
  <label>
    Note
    <input bind:value={note} placeholder="Facoltative" />
  </label>
  {#if errore}<p class="errore" role="alert">{errore}</p>{/if}
  <div class="azioni">
    <button type="submit" disabled={inCorso}>Registra iscrizione</button>
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
    grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
    gap: 1rem;
  }

  label {
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

  .errore {
    margin: 0;
    color: var(--colore-errore);
  }

  .azioni {
    display: flex;
    gap: 0.5rem;
  }

  button {
    padding: 0.55rem 1.1rem;
    border-radius: var(--raggio);
    border: 1px solid var(--colore-primario);
    background: var(--colore-primario);
    color: var(--colore-primario-testo);
  }

  button.secondario {
    background: transparent;
    color: var(--colore-primario);
  }
</style>
