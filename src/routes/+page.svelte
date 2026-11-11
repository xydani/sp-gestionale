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
  import type { RiepilogoDashboard, TipoScadenza } from "$lib/types";

  let riepilogo = $state<RiepilogoDashboard | null>(null);

  const ETICHETTE_TIPO_SCADENZA: Record<TipoScadenza, string> = {
    abbonamento: "Abbonamento",
    iscrizione: "Iscrizione annuale",
    certificato: "Certificato medico",
  };

  const oggi = oggiIso();
  const meseCorrente = nomeMese(oggi);

  /** Ciò che è in scadenza è ancora valido: conta tra gli attivi. */
  const abbonamentiValidi = $derived(
    riepilogo ? riepilogo.abbonamenti.attivi + riepilogo.abbonamenti.inScadenza : 0,
  );
  const iscrizioniValide = $derived(
    riepilogo ? riepilogo.iscrizioni.attivi + riepilogo.iscrizioni.inScadenza : 0,
  );
  const totaleInScadenza = $derived(
    riepilogo
      ? riepilogo.abbonamenti.inScadenza +
          riepilogo.iscrizioni.inScadenza +
          riepilogo.certificati.inScadenza
      : 0,
  );
  const certificatiDaSistemare = $derived(
    riepilogo ? riepilogo.certificati.assenti + riepilogo.certificati.scaduti : 0,
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

{#if riepilogo && riepilogo.iscritti === 0}
  <section class="per-iniziare">
    <h2>Per iniziare</h2>
    <ol>
      <li>
        Controlla prezzi e durate nel <a href={resolve("/listino")}>Listino</a>.
      </li>
      <li>
        Inserisci chi è già iscritto da <a href={resolve("/iscritti")}>Iscritti</a> › «Aggiungi già iscritto»:
        bastano nome, cognome e scadenza dell'abbonamento.
      </li>
      <li>Per chi si iscrive da oggi usa «Nuovo iscritto»: registra anche quota e pagamento.</li>
      <li>
        Fai un backup su una chiavetta da <a href={resolve("/impostazioni")}>Impostazioni</a>.
      </li>
    </ol>
  </section>
{/if}

{#if riepilogo}
  <div class="riquadri">
    <RiquadroNumero
      etichetta="Iscritti"
      valore={String(riepilogo.iscritti)}
      href={resolve("/iscritti")}
    >
      {#snippet dettaglio()}
        {iscrizioniValide} con iscrizione valida
      {/snippet}
    </RiquadroNumero>

    <RiquadroNumero etichetta="Abbonamenti attivi" valore={String(abbonamentiValidi)} tono="attivo">
      {#snippet dettaglio()}
        {riepilogo!.abbonamenti.assenti} iscritti senza abbonamento
      {/snippet}
    </RiquadroNumero>

    <RiquadroNumero
      etichetta="In scadenza"
      valore={String(totaleInScadenza)}
      tono="in_scadenza"
      href={resolve("/scadenze")}
    >
      {#snippet dettaglio()}
        {riepilogo!.abbonamenti.inScadenza} abbonamenti · {riepilogo!.iscrizioni.inScadenza} iscrizioni
        · {riepilogo!.certificati.inScadenza} certificati
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
      etichetta="Certificati da sistemare"
      valore={String(certificatiDaSistemare)}
      tono={certificatiDaSistemare > 0 ? "scaduto" : "attivo"}
      href={resolve("/scadenze")}
    >
      {#snippet dettaglio()}
        {riepilogo!.certificati.assenti} non presentati · {riepilogo!.certificati.scaduti} scaduti
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
              <td>{ETICHETTE_TIPO_SCADENZA[scadenza.tipo]}</td>
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
  .per-iniziare {
    margin-bottom: 2rem;
    padding: 1.25rem 1.5rem;
    border: 1px solid var(--colore-bordo);
    border-left: 4px solid var(--colore-primario);
    border-radius: var(--raggio-grande);
    background: var(--colore-primario-tenue);
  }

  .per-iniziare ol {
    margin: 0.75rem 0 0;
    padding-left: 1.25rem;
    display: grid;
    gap: 0.4rem;
  }

  .per-iniziare a {
    color: var(--colore-primario);
    font-weight: 500;
  }

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
