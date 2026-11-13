<script lang="ts">
  import Tendina from "./Tendina.svelte";
  import { untrack } from "svelte";
  import { centesimiInTestoEuro, euroInCentesimi } from "$lib/format";
  import type { DatiTipologia, UnitaDurata } from "$lib/types";
  import {
    DURATA_MASSIMA_ABBONAMENTO_MESI,
    LUNGHEZZA_MASSIMA_NOME,
    validita,
    erroreImportoEuro,
  } from "$lib/validazione";

  /** Gli stessi limiti del programma: 36 mesi, cioè 1095 giorni. */
  const DURATA_MASSIMA: Record<UnitaDurata, number> = {
    mesi: DURATA_MASSIMA_ABBONAMENTO_MESI,
    giorni: 1095,
  };

  let {
    iniziale,
    etichettaConferma,
    onConferma,
    onAnnulla,
  }: {
    iniziale?: DatiTipologia;
    etichettaConferma: string;
    /** Chi apre il modulo lo chiude quando il salvataggio riesce. */
    onConferma: (dati: DatiTipologia) => Promise<unknown>;
    onAnnulla: () => void;
  } = $props();

  // Il form parte dai valori iniziali e poi li modifica in autonomia
  const valoriIniziali = untrack(() => iniziale);

  let nome = $state(valoriIniziali?.nome ?? "");
  let durataValore = $state(valoriIniziali?.durataValore ?? 1);
  let durataUnita = $state<UnitaDurata>(valoriIniziali?.durataUnita ?? "mesi");
  let prezzo = $state(valoriIniziali ? centesimiInTestoEuro(valoriIniziali.prezzoCent) : "");
  let errore = $state<string | null>(null);
  let inCorso = $state(false);

  async function conferma(evento: SubmitEvent) {
    evento.preventDefault();
    const prezzoCent = euroInCentesimi(prezzo);
    if (prezzoCent === null) {
      errore = "Prezzo non valido (esempio: 45 oppure 45,50)";
      return;
    }
    errore = null;
    inCorso = true;
    try {
      await onConferma({ nome, durataValore, durataUnita, prezzoCent });
    } finally {
      inCorso = false;
    }
  }
</script>

<form onsubmit={conferma}>
  <div class="campi">
    <label class="nome">
      Nome
      <!-- svelte-ignore a11y_autofocus -->
      <input
        placeholder="Es. Trimestrale"
        bind:value={nome}
        required
        autofocus
        maxlength={LUNGHEZZA_MASSIMA_NOME}
      />
    </label>
    <label>
      Durata
      <span class="durata">
        <input
          type="number"
          min="1"
          max={DURATA_MASSIMA[durataUnita]}
          step="1"
          bind:value={durataValore}
          required
        />
        <Tendina
          bind:valore={durataUnita}
          opzioni={[
            { valore: "mesi", etichetta: "mesi" },
            { valore: "giorni", etichetta: "giorni" },
          ]}
          etichettaAccessibile="Unità della durata"
        />
      </span>
    </label>
    <label>
      Prezzo (€)
      <input
        placeholder="Es. 135"
        bind:value={prezzo}
        inputmode="decimal"
        required
        use:validita={erroreImportoEuro(prezzo)}
      />
    </label>
  </div>
  {#if errore}<p class="errore" role="alert">{errore}</p>{/if}
  <div class="azioni">
    <button class="pulsante" type="submit" disabled={inCorso}>{etichettaConferma}</button>
    <button type="button" class="pulsante secondario" onclick={onAnnulla}>Annulla</button>
  </div>
</form>

<style>
  form {
    display: grid;
    gap: 1rem;
  }

  .campi {
    display: grid;
    grid-template-columns: minmax(0, 1.4fr) minmax(0, 1.2fr) minmax(0, 1fr);
    gap: 1rem;
  }

  label {
    display: grid;
    min-width: 0;
    gap: 0.35rem;
    font-weight: 500;
    font-size: 0.9rem;
  }

  .durata {
    display: grid;
    grid-template-columns: 5rem minmax(0, 1fr);
    gap: 0.5rem;
    font-weight: 400;
  }

  input {
    min-width: 0;
    padding: 0.55rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
    font-weight: 400;
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
