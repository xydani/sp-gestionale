<script lang="ts">
  import Tendina from "./Tendina.svelte";
  import { untrack } from "svelte";
  import { centesimiInTestoEuro, euroInCentesimi } from "$lib/format";
  import type { DatiTipologia, UnitaDurata } from "$lib/types";

  let {
    iniziale,
    etichettaConferma,
    onConferma,
    onAnnulla,
  }: {
    iniziale?: DatiTipologia;
    etichettaConferma: string;
    onConferma: (dati: DatiTipologia) => Promise<boolean>;
    onAnnulla?: () => void;
  } = $props();

  // Il form parte dai valori iniziali e poi li modifica in autonomia
  const valoriIniziali = untrack(() => iniziale);
  const eModifica = valoriIniziali !== undefined;

  let nome = $state(valoriIniziali?.nome ?? "");
  let durataValore = $state(valoriIniziali?.durataValore ?? 1);
  let durataUnita = $state<UnitaDurata>(valoriIniziali?.durataUnita ?? "mesi");
  let prezzo = $state(valoriIniziali ? centesimiInTestoEuro(valoriIniziali.prezzoCent) : "");
  let errore = $state<string | null>(null);

  async function conferma(evento: SubmitEvent) {
    evento.preventDefault();
    const prezzoCent = euroInCentesimi(prezzo);
    if (prezzoCent === null) {
      errore = "Prezzo non valido (esempio: 45 oppure 45,50)";
      return;
    }
    errore = null;
    const riuscito = await onConferma({ nome, durataValore, durataUnita, prezzoCent });
    if (riuscito && !eModifica) {
      nome = "";
      durataValore = 1;
      prezzo = "";
    }
  }
</script>

<form onsubmit={conferma}>
  <input class="nome" placeholder="Nome (es. Mensile)" bind:value={nome} required />
  <input class="numero" type="number" min="1" bind:value={durataValore} required />
  <div class="unita">
    <Tendina
      bind:valore={durataUnita}
      opzioni={[
        { valore: "mesi", etichetta: "mesi" },
        { valore: "giorni", etichetta: "giorni" },
      ]}
      etichettaAccessibile="Unità della durata"
    />
  </div>
  <input class="prezzo" placeholder="Prezzo €" bind:value={prezzo} inputmode="decimal" required />
  <button class="pulsante" type="submit">{etichettaConferma}</button>
  {#if onAnnulla}
    <button type="button" class="pulsante secondario" onclick={onAnnulla}>Annulla</button>
  {/if}
  {#if errore}<p class="errore" role="alert">{errore}</p>{/if}
</form>

<style>
  .unita {
    width: 7rem;
  }

  form {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  input {
    padding: 0.45rem 0.6rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
  }

  .nome {
    flex: 1 1 180px;
  }

  .numero {
    width: 5rem;
  }

  .prezzo {
    width: 7rem;
  }

  .errore {
    flex-basis: 100%;
    margin: 0;
    color: var(--colore-errore);
  }
</style>
