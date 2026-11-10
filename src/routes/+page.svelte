<script lang="ts">
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";
  import { leggiRiepilogoDashboard, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import RiquadroNumero from "$lib/components/RiquadroNumero.svelte";
  import {
    descriviScadenza,
    formattaData,
    formattaEuro,
    nomeMese,
    oggiIso,
    variazionePercentuale,
  } from "$lib/format";
  import { aggiornaConteggioScadenze } from "$lib/promemoria.svelte";
  import type { RiepilogoDashboard } from "$lib/types";

  let riepilogo = $state<RiepilogoDashboard | null>(null);

  const oggi = oggiIso();
  const meseCorrente = nomeMese(oggi);

  /** Un abbonamento in scadenza è ancora valido: conta tra gli attivi. */
  const abbonamentiValidi = $derived(
    riepilogo ? riepilogo.abbonamenti.attivi + riepilogo.abbonamenti.inScadenza : 0,
  );
  const variazioneIncassi = $derived(
    riepilogo
      ? variazionePercentuale(
          riepilogo.incassiMese.totaleCent,
          riepilogo.incassiMesePrecedente.totaleCent,
        )
      : null,
  );

  onMount(async () => {
    try {
      riepilogo = await leggiRiepilogoDashboard();
      await aggiornaConteggioScadenze();
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  });
</script>

<h1>Dashboard</h1>

{#if riepilogo}
  <div class="riquadri">
    <RiquadroNumero
      etichetta="Iscritti"
      valore={String(riepilogo.iscritti)}
      href={resolve("/iscritti")}
    >
      {#snippet dettaglio()}
        {riepilogo!.iscrizioni.attivi + riepilogo!.iscrizioni.inScadenza} con iscrizione valida
      {/snippet}
    </RiquadroNumero>

    <RiquadroNumero etichetta="Abbonamenti attivi" valore={String(abbonamentiValidi)} tono="attivo">
      {#snippet dettaglio()}
        {riepilogo!.abbonamenti.assenti} iscritti senza abbonamento
      {/snippet}
    </RiquadroNumero>

    <RiquadroNumero
      etichetta="In scadenza"
      valore={String(riepilogo.abbonamenti.inScadenza + riepilogo.iscrizioni.inScadenza)}
      tono="in_scadenza"
      href={resolve("/scadenze")}
    >
      {#snippet dettaglio()}
        {riepilogo!.abbonamenti.inScadenza} abbonamenti · {riepilogo!.iscrizioni.inScadenza} iscrizioni
      {/snippet}
    </RiquadroNumero>

    <RiquadroNumero
      etichetta="Abbonamenti scaduti"
      valore={String(riepilogo.abbonamenti.scaduti)}
      tono="scaduto"
      href={resolve("/scadenze")}
    >
      {#snippet dettaglio()}
        {riepilogo!.iscrizioni.scaduti} iscrizioni annuali scadute
      {/snippet}
    </RiquadroNumero>

    <RiquadroNumero
      etichetta="Incassi di {meseCorrente}"
      valore={formattaEuro(riepilogo.incassiMese.totaleCent)}
      href={resolve("/pagamenti")}
    >
      {#snippet dettaglio()}
        Abbonamenti {formattaEuro(riepilogo!.incassiMese.abbonamentiCent)} · Iscrizioni
        {formattaEuro(riepilogo!.incassiMese.iscrizioniCent)}
        {#if variazioneIncassi}
          <br />{variazioneIncassi} rispetto al mese precedente
        {/if}
      {/snippet}
    </RiquadroNumero>
  </div>

  <section class="prossime">
    <h2>Prossime scadenze <span class="nota">(30 giorni)</span></h2>
    {#if riepilogo.prossimeScadenze.length === 0}
      <p class="vuoto">Nessuna scadenza nei prossimi 30 giorni.</p>
    {:else}
      <table class="tabella riquadrata">
        <thead>
          <tr><th>Cognome e nome</th><th>Cosa scade</th><th>Data</th><th></th></tr>
        </thead>
        <tbody>
          {#each riepilogo.prossimeScadenze as scadenza (scadenza.tipo + scadenza.iscrittoId)}
            <tr>
              <td>
                <a href={resolve("/iscritti/[id]", { id: String(scadenza.iscrittoId) })}>
                  {scadenza.cognome}
                  {scadenza.nome}
                </a>
              </td>
              <td>{scadenza.tipo === "abbonamento" ? "Abbonamento" : "Iscrizione annuale"}</td>
              <td>{formattaData(scadenza.dataScadenza)}</td>
              <td class="distanza">{descriviScadenza(scadenza.dataScadenza, oggi)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
      <a class="tutte" href={resolve("/scadenze")}>Tutte le scadenze →</a>
    {/if}
  </section>
{/if}

<style>
  .riquadri {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(210px, 1fr));
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .prossime {
    display: grid;
    gap: 0.75rem;
  }

  h2 {
    margin: 0;
    font-size: 1.15rem;
  }

  .nota {
    color: var(--colore-testo-secondario);
    font-weight: 400;
    font-size: 0.95rem;
  }

  td a {
    color: inherit;
    text-decoration: none;
    font-weight: 500;
  }

  td a:hover {
    color: var(--colore-primario);
  }

  .distanza {
    color: var(--colore-testo-secondario);
  }

  .tutte {
    justify-self: end;
    color: var(--colore-primario);
    text-decoration: none;
  }

  .vuoto {
    margin: 0;
    color: var(--colore-testo-secondario);
  }
</style>
