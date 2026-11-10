<script lang="ts">
  import Tendina from "./Tendina.svelte";
  import CampoData from "./CampoData.svelte";
  import { onMount } from "svelte";
  import { messaggioErrore, proponiIscrizione, registraIscrizione } from "$lib/api";
  import { centesimiInTestoEuro, euroInCentesimi, oggiIso } from "$lib/format";
  import { OPZIONI_METODO } from "$lib/metodi-pagamento";
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
      <Tendina bind:valore={metodo} opzioni={OPZIONI_METODO} />
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
    <button class="pulsante" type="submit" disabled={inCorso}>Registra iscrizione</button>
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
    grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
    gap: 1rem;
  }

  label {
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

  .errore {
    margin: 0;
    color: var(--colore-errore);
  }

  .azioni {
    display: flex;
    gap: 0.5rem;
  }
</style>
