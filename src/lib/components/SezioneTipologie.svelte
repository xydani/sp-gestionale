<script lang="ts">
  import { onMount } from "svelte";
  import {
    creaTipologia,
    elencaTipologie,
    impostaTipologiaAttiva,
    messaggioErrore,
    modificaTipologia,
  } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { descriviDurata, formattaEuro } from "$lib/format";
  import type { DatiTipologia, TipologiaAbbonamento } from "$lib/types";
  import FormTipologia from "./FormTipologia.svelte";

  let tipologie = $state<TipologiaAbbonamento[]>([]);
  let idInModifica = $state<number | null>(null);

  async function ricarica() {
    tipologie = await elencaTipologie(false);
  }

  /** Esegue un'operazione, mostra l'eventuale errore e ricarica l'elenco. */
  async function esegui(operazione: () => Promise<unknown>): Promise<boolean> {
    try {
      await operazione();
      await ricarica();
      return true;
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
      return false;
    }
  }

  function aggiungi(dati: DatiTipologia) {
    return esegui(() => creaTipologia(dati));
  }

  async function salvaModifica(id: number, dati: DatiTipologia) {
    const riuscito = await esegui(() => modificaTipologia(id, dati));
    if (riuscito) idInModifica = null;
    return riuscito;
  }

  function alternaAttiva(tipologia: TipologiaAbbonamento) {
    return esegui(() => impostaTipologiaAttiva(tipologia.id, !tipologia.attiva));
  }

  onMount(ricarica);
</script>

<section>
  <h2>Listino abbonamenti</h2>
  <p class="nota">
    Le modifiche valgono per i nuovi pagamenti; lo storico mantiene i valori del momento. Una
    tipologia non più in uso si disattiva invece di cancellarla.
  </p>

  <table>
    <thead>
      <tr><th>Nome</th><th>Durata</th><th>Prezzo</th><th>Stato</th><th></th></tr>
    </thead>
    <tbody>
      {#each tipologie as tipologia (tipologia.id)}
        {#if idInModifica === tipologia.id}
          <tr>
            <td colspan="5">
              <FormTipologia
                iniziale={tipologia}
                etichettaConferma="Salva"
                onConferma={(dati) => salvaModifica(tipologia.id, dati)}
                onAnnulla={() => (idInModifica = null)}
              />
            </td>
          </tr>
        {:else}
          <tr class:disattiva={!tipologia.attiva}>
            <td>{tipologia.nome}</td>
            <td>{descriviDurata(tipologia.durataValore, tipologia.durataUnita)}</td>
            <td>{formattaEuro(tipologia.prezzoCent)}</td>
            <td>{tipologia.attiva ? "Attiva" : "Disattivata"}</td>
            <td class="azioni">
              <button type="button" onclick={() => (idInModifica = tipologia.id)}>Modifica</button>
              <button type="button" onclick={() => alternaAttiva(tipologia)}>
                {tipologia.attiva ? "Disattiva" : "Riattiva"}
              </button>
            </td>
          </tr>
        {/if}
      {/each}
    </tbody>
  </table>

  <h3>Nuova tipologia</h3>
  <FormTipologia etichettaConferma="Aggiungi" onConferma={aggiungi} />
</section>

<style>
  section {
    display: grid;
    gap: 0.75rem;
    max-width: 720px;
    background: var(--colore-superficie);
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    padding: 1.5rem;
    margin-bottom: 1.5rem;
  }

  h2,
  h3,
  p {
    margin: 0;
  }

  h3 {
    font-size: 1rem;
  }

  .nota {
    color: var(--colore-testo-secondario);
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  th,
  td {
    text-align: left;
    padding: 0.5rem;
    border-bottom: 1px solid var(--colore-bordo);
  }

  th {
    color: var(--colore-testo-secondario);
    font-weight: 500;
  }

  tr.disattiva td {
    color: var(--colore-testo-secondario);
  }

  .azioni {
    text-align: right;
    white-space: nowrap;
  }

  .azioni button {
    background: none;
    border: none;
    color: var(--colore-primario);
    padding: 0.25rem 0.5rem;
  }
</style>
