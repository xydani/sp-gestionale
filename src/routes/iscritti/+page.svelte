<script lang="ts">
  import { resolve } from "$app/paths";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { creaIscritto, elencaIscritti, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import BadgeStato from "$lib/components/BadgeStato.svelte";
  import FormIscritto from "$lib/components/FormIscritto.svelte";
  import { formattaData } from "$lib/format";
  import { ritarda } from "$lib/ritarda";
  import { ETICHETTE_STATO, STATI_ABBONAMENTO } from "$lib/stato-abbonamento";
  import type { DatiIscritto, Iscritto, StatoAbbonamento } from "$lib/types";

  const PAUSA_RICERCA_MS = 200;

  let iscritti = $state<Iscritto[]>([]);
  let testoRicerca = $state("");
  let statoFiltro = $state<StatoAbbonamento | "">("");
  let includiArchiviati = $state(false);
  let nuovoIscrittoAperto = $state(false);
  let caricamento = $state(true);

  async function carica() {
    try {
      iscritti = await elencaIscritti({
        testo: testoRicerca,
        includiArchiviati,
        stato: statoFiltro || null,
      });
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    } finally {
      caricamento = false;
    }
  }

  const caricaDopoPausa = ritarda(carica, PAUSA_RICERCA_MS);

  async function salvaNuovoIscritto(dati: DatiIscritto): Promise<boolean> {
    try {
      const creato = await creaIscritto(dati);
      mostraAvviso("successo", `${creato.nome} ${creato.cognome} aggiunto agli iscritti`);
      nuovoIscrittoAperto = false;
      await goto(resolve("/iscritti/[id]", { id: String(creato.id) }));
      return true;
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
      return false;
    }
  }

  onMount(carica);
</script>

<div class="intestazione">
  <h1>Iscritti</h1>
  {#if !nuovoIscrittoAperto}
    <button type="button" class="primario" onclick={() => (nuovoIscrittoAperto = true)}>
      + Nuovo iscritto
    </button>
  {/if}
</div>

{#if nuovoIscrittoAperto}
  <section class="pannello">
    <h2>Nuovo iscritto</h2>
    <FormIscritto
      etichettaConferma="Salva iscritto"
      onConferma={salvaNuovoIscritto}
      onAnnulla={() => (nuovoIscrittoAperto = false)}
    />
  </section>
{/if}

<div class="filtri">
  <input
    class="ricerca"
    type="search"
    placeholder="Cerca per nome o cognome…"
    bind:value={testoRicerca}
    oninput={caricaDopoPausa}
  />
  <select bind:value={statoFiltro} onchange={carica}>
    <option value="">Tutti gli stati</option>
    {#each STATI_ABBONAMENTO as stato (stato)}
      <option value={stato}>{ETICHETTE_STATO[stato]}</option>
    {/each}
  </select>
  <label class="spunta">
    <input type="checkbox" bind:checked={includiArchiviati} onchange={carica} />
    Mostra archiviati
  </label>
</div>

<p class="conteggio">
  {caricamento ? "Caricamento…" : `${iscritti.length} iscritti`}
</p>

<table>
  <thead>
    <tr><th>Cognome e nome</th><th>Telefono</th><th>Scadenza</th><th>Stato</th></tr>
  </thead>
  <tbody>
    {#each iscritti as iscritto (iscritto.id)}
      <tr
        class:archiviato={iscritto.archiviato}
        onclick={() => goto(resolve("/iscritti/[id]", { id: String(iscritto.id) }))}
      >
        <td>
          <a href={resolve("/iscritti/[id]", { id: String(iscritto.id) })}
            >{iscritto.cognome} {iscritto.nome}</a
          >
          {#if iscritto.archiviato}<span class="etichetta-archiviato">archiviato</span>{/if}
        </td>
        <td>{iscritto.telefono ?? "—"}</td>
        <td>{iscritto.ultimaDataScadenza ? formattaData(iscritto.ultimaDataScadenza) : "—"}</td>
        <td><BadgeStato stato={iscritto.stato} /></td>
      </tr>
    {:else}
      {#if !caricamento}
        <tr><td colspan="4" class="vuoto">Nessun iscritto trovato</td></tr>
      {/if}
    {/each}
  </tbody>
</table>

<style>
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
    border-radius: var(--raggio);
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

  .filtri input:not([type="checkbox"]),
  select {
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

  .conteggio {
    color: var(--colore-testo-secondario);
    font-size: 0.9rem;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    background: var(--colore-superficie);
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    overflow: hidden;
  }

  th,
  td {
    text-align: left;
    padding: 0.65rem 1rem;
    border-bottom: 1px solid var(--colore-bordo);
  }

  th {
    color: var(--colore-testo-secondario);
    font-weight: 500;
    background: var(--colore-sfondo);
  }

  tbody tr {
    cursor: pointer;
  }

  tbody tr:hover {
    background: #f0f4ff;
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

  .vuoto {
    text-align: center;
    color: var(--colore-testo-secondario);
    padding: 2rem;
  }

  .primario {
    padding: 0.55rem 1.1rem;
    border-radius: var(--raggio);
    border: none;
    background: var(--colore-primario);
    color: var(--colore-primario-testo);
    font-weight: 500;
  }
</style>
