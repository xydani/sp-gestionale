<script lang="ts">
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";
  import { elencaMovimenti, messaggioErrore } from "$lib/api";
  import { sommaImporti, totaliPerMetodo } from "$lib/cassa";
  import { formattaEuro, oggiIso } from "$lib/format";
  import { ETICHETTE_METODO } from "$lib/metodi-pagamento";
  import type { Movimento } from "$lib/types";

  /** Incassi della giornata per la chiusura serale: totale, metodi e movimenti. */
  let {
    aggiornamento = 0,
  }: {
    /** Cresce quando la dashboard registra un incasso: la cassa si ricarica. */
    aggiornamento?: number;
  } = $props();

  let movimenti = $state<Movimento[]>([]);
  let errore = $state<string | null>(null);

  const totaleCent = $derived(sommaImporti(movimenti));
  const perMetodo = $derived(totaliPerMetodo(movimenti));

  async function carica() {
    const oggi = oggiIso();
    try {
      movimenti = await elencaMovimenti(oggi, oggi);
      errore = null;
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
    }
  }

  onMount(carica);

  $effect(() => {
    if (aggiornamento > 0) carica();
  });
</script>

<section class="cassa">
  <div class="intestazione">
    <h2>Cassa di oggi</h2>
    <strong class="totale">{formattaEuro(totaleCent)}</strong>
  </div>

  {#if errore}
    <p class="errore">{errore}</p>
  {:else if movimenti.length === 0}
    <p class="vuoto">Nessun incasso registrato oggi.</p>
  {:else}
    <div class="metodi">
      {#each perMetodo as voce (voce.metodo)}
        <span class="metodo">
          {ETICHETTE_METODO[voce.metodo]}
          <strong>{formattaEuro(voce.totaleCent)}</strong>
          <span class="nota">({voce.numeroMovimenti})</span>
        </span>
      {/each}
    </div>
    <table class="tabella riquadrata">
      <thead>
        <tr><th>Cognome e nome</th><th>Cosa</th><th>Metodo</th><th class="importo">Importo</th></tr>
      </thead>
      <tbody>
        {#each movimenti as movimento, indice (indice)}
          <tr>
            <td>
              <a href={resolve("/iscritti/[id]", { id: String(movimento.iscrittoId) })}>
                {movimento.cognome}
                {movimento.nome}
              </a>
            </td>
            <td>{movimento.descrizione}</td>
            <td>{ETICHETTE_METODO[movimento.metodo]}</td>
            <td class="importo">{formattaEuro(movimento.importoCent)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</section>

<style>
  .cassa {
    display: grid;
    gap: 0.75rem;
    margin-bottom: 2rem;
  }

  .intestazione {
    display: flex;
    align-items: baseline;
    gap: 1rem;
  }

  h2 {
    margin: 0;
    font-size: 1.15rem;
  }

  .totale {
    font-size: 1.35rem;
    font-variant-numeric: tabular-nums;
  }

  .metodi {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .metodo {
    display: inline-flex;
    gap: 0.4rem;
    align-items: baseline;
    padding: 0.35rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: 999px;
    background: var(--colore-superficie);
  }

  .nota,
  .vuoto {
    color: var(--colore-testo-secondario);
  }

  .vuoto,
  .errore {
    margin: 0;
  }

  .errore {
    color: var(--colore-errore);
  }

  .importo {
    text-align: right;
    font-variant-numeric: tabular-nums;
  }

  td a {
    color: inherit;
    text-decoration: none;
    font-weight: 500;
  }

  td a:hover {
    color: var(--colore-primario);
  }
</style>
