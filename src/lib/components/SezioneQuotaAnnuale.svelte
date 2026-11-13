<script lang="ts">
  import { onMount } from "svelte";
  import { leggiImpostazioni, messaggioErrore, salvaImpostazioni } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import {
    centesimiInTestoEuro,
    euroInCentesimi,
    formattaEuro,
    nomeMeseDaNumero,
  } from "$lib/format";
  import { descriviAnnoSportivo } from "$lib/listino";
  import type { Impostazioni } from "$lib/types";
  import { erroreImportoEuro, validita } from "$lib/validazione";
  import FinestraSovrapposta from "./FinestraSovrapposta.svelte";
  import Tendina from "./Tendina.svelte";

  const MESI = Array.from({ length: 12 }, (_, indice) => indice + 1);

  let impostazioni = $state<Impostazioni | null>(null);
  let moduloAperto = $state(false);
  let quota = $state("");
  let meseInizio = $state(9);
  let inCorso = $state(false);

  onMount(async () => {
    try {
      impostazioni = await leggiImpostazioni();
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  });

  function apri(attuali: Impostazioni) {
    quota = centesimiInTestoEuro(attuali.quotaIscrizioneCent);
    meseInizio = attuali.meseInizioAnnoSportivo;
    moduloAperto = true;
  }

  async function salva(evento: SubmitEvent) {
    evento.preventDefault();
    const quotaIscrizioneCent = euroInCentesimi(quota);
    if (!impostazioni || quotaIscrizioneCent === null) return;
    inCorso = true;
    try {
      // Le impostazioni si salvano tutte insieme: le altre restano com'erano
      impostazioni = await salvaImpostazioni({
        ...impostazioni,
        quotaIscrizioneCent,
        meseInizioAnnoSportivo: meseInizio,
      });
      moduloAperto = false;
      mostraAvviso("successo", "Quota annuale aggiornata");
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    } finally {
      inCorso = false;
    }
  }
</script>

<section class="scheda">
  <div class="intestazione-sezione">
    <h2>Quota di iscrizione annuale</h2>
    <span class="spazio"></span>
    {#if impostazioni}
      {@const attuali = impostazioni}
      <button type="button" class="pulsante secondario" onclick={() => apri(attuali)}>
        Modifica
      </button>
    {/if}
  </div>
  {#if impostazioni}
    <p class="riepilogo">
      <strong class="importo">{formattaEuro(impostazioni.quotaIscrizioneCent)}</strong>
      <span>
        per l'anno sportivo, {descriviAnnoSportivo(impostazioni.meseInizioAnnoSportivo)}
      </span>
    </p>
  {/if}
</section>

{#if moduloAperto}
  <FinestraSovrapposta
    titolo="Quota di iscrizione annuale"
    larghezza="stretta"
    onChiusa={() => (moduloAperto = false)}
  >
    <form onsubmit={salva}>
      <div class="campi">
        <label>
          Quota (€)
          <!-- svelte-ignore a11y_autofocus -->
          <input
            bind:value={quota}
            inputmode="decimal"
            required
            autofocus
            use:validita={erroreImportoEuro(quota)}
          />
        </label>
        <label>
          L'anno sportivo inizia a
          <Tendina
            bind:valore={meseInizio}
            opzioni={MESI.map((mese) => ({ valore: mese, etichetta: nomeMeseDaNumero(mese) }))}
          />
        </label>
      </div>
      <p class="nota">
        Vale per le nuove quote: quelle già pagate mantengono importo e periodo del momento.
      </p>
      <div class="azioni">
        <button class="pulsante" type="submit" disabled={inCorso}>Salva</button>
        <button type="button" class="pulsante secondario" onclick={() => (moduloAperto = false)}>
          Annulla
        </button>
      </div>
    </form>
  </FinestraSovrapposta>
{/if}

<style>
  section {
    margin-bottom: 1.5rem;
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
    align-items: baseline;
    gap: 0.5rem;
    margin: 0;
    color: var(--colore-testo-secondario);
  }

  .importo {
    color: var(--colore-testo);
    font-size: 1.25rem;
  }

  form {
    display: grid;
    gap: 1rem;
  }

  .campi {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 1rem;
  }

  label {
    display: grid;
    min-width: 0;
    gap: 0.35rem;
    font-weight: 500;
    font-size: 0.9rem;
  }

  input {
    min-width: 0;
    padding: 0.55rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
    font-weight: 400;
  }

  .nota {
    margin: 0;
    color: var(--colore-testo-secondario);
    font-size: 0.9rem;
  }

  .azioni {
    display: flex;
    gap: 0.5rem;
  }
</style>
