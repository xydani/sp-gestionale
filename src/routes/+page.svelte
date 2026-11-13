<script lang="ts">
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";
  import { leggiRiepilogoDashboard, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import CassaDiOggi from "$lib/components/CassaDiOggi.svelte";
  import ElencoScadenze from "$lib/components/ElencoScadenze.svelte";
  import FinestraRinnovo from "$lib/components/FinestraRinnovo.svelte";
  import RiquadroNumero from "$lib/components/RiquadroNumero.svelte";
  import {
    descriviScadenza,
    formattaDataBreve,
    formattaEuro,
    nomeMese,
    oggiIso,
    variazionePercentuale,
  } from "$lib/format";
  import { aggiornaConteggioScadenze } from "$lib/promemoria.svelte";
  import { RIGHE_MINIME, osservaRigheVisibili } from "$lib/righe-visibili";
  import { eRinnovabile, type TipoRinnovabile } from "$lib/rinnovo";
  import { gruppiConScaduti, gruppoDaMostrare } from "$lib/scadenze";
  import type { RiepilogoDashboard, TipoScadenza } from "$lib/types";

  let riepilogo = $state<RiepilogoDashboard | null>(null);

  /** Brevi, perché gli elenchi della dashboard stanno affiancati. */
  const ETICHETTE_TIPO_SCADENZA: Record<TipoScadenza, string> = {
    abbonamento: "Abbonamento",
    iscrizione: "Iscrizione",
    certificato: "Certificato",
  };
  const ETICHETTE_GRUPPO_SCADUTI: Record<TipoScadenza, string> = {
    abbonamento: "Abbonamenti",
    iscrizione: "Iscrizioni",
    certificato: "Certificati",
  };

  /** Sotto le prossime scadenze: il collegamento a tutte e il margine. */
  const SPAZIO_SOTTO_PROSSIME_PX = 80;
  let tabellaProssime = $state<HTMLTableElement>();
  /** Quante prossime scadenze stanno nella finestra; le altre sono nella pagina Scadenze. */
  let prossimeVisibili = $state<number>(RIGHE_MINIME);
  const prossimeMostrate = $derived(
    riepilogo ? riepilogo.prossimeScadenze.slice(0, prossimeVisibili) : [],
  );

  $effect(() => {
    if (!tabellaProssime) return;
    return osservaRigheVisibili(
      tabellaProssime,
      SPAZIO_SOTTO_PROSSIME_PX,
      (righe) => (prossimeVisibili = righe),
    );
  });

  const gruppiScaduti = $derived(riepilogo ? gruppiConScaduti(riepilogo) : []);
  let gruppoScelto = $state<TipoScadenza | null>(null);
  const gruppoMostrato = $derived(gruppoDaMostrare(gruppiScaduti, gruppoScelto));

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

  /** Scadenza di cui è aperta la finestra di rinnovo. */
  let daRinnovare = $state<{ iscrittoId: number; tipo: TipoRinnovabile } | null>(null);
  /** Cresce a ogni rinnovo: la cassa di oggi si ricarica. */
  let rinnoviRegistrati = $state(0);

  async function carica() {
    try {
      riepilogo = await leggiRiepilogoDashboard();
      await aggiornaConteggioScadenze();
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }

  async function chiudiRinnovo(registrato: boolean) {
    daRinnovare = null;
    if (!registrato) return;
    rinnoviRegistrati++;
    await carica();
  }

  onMount(carica);
</script>

<h1 class="titolo">Dashboard</h1>

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
      compatto
      etichetta="Iscritti"
      valore={String(riepilogo.iscritti)}
      href={resolve("/iscritti")}
    >
      {#snippet dettaglio()}
        {iscrizioniValide} con iscrizione valida
      {/snippet}
    </RiquadroNumero>

    <RiquadroNumero
      compatto
      etichetta="Abbonamenti attivi"
      valore={String(abbonamentiValidi)}
      tono="attivo"
    >
      {#snippet dettaglio()}
        {riepilogo!.abbonamenti.assenti} senza abbonamento
      {/snippet}
    </RiquadroNumero>

    <RiquadroNumero
      compatto
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
      compatto
      etichetta="Certificati da sistemare"
      valore={String(certificatiDaSistemare)}
      tono={certificatiDaSistemare > 0 ? "scaduto" : "attivo"}
      href={resolve("/scadenze")}
    >
      {#snippet dettaglio()}
        {riepilogo!.certificati.assenti} mancanti · {riepilogo!.certificati.scaduti} scaduti
      {/snippet}
    </RiquadroNumero>

    <RiquadroNumero
      compatto
      etichetta="Incassi di {meseCorrente}"
      valore={formattaEuro(riepilogo.incassiMese.totaleCent)}
      href={resolve("/pagamenti")}
    >
      {#snippet dettaglio()}
        {#if variazioneIncassi}
          {variazioneIncassi} sul mese prima
        {:else}
          Abbonamenti e quote
        {/if}
      {/snippet}
    </RiquadroNumero>
  </div>

  <div class="pannelli" class:affiancati={gruppiScaduti.length > 0}>
    {#if gruppoMostrato}
      <section class="pannello">
        <h2>Già scaduti <span class="nota">(i più recenti)</span></h2>
        <div class="segmenti" role="group" aria-label="Tipo di scadenza">
          {#each gruppiScaduti as gruppo (gruppo.tipo)}
            <button
              type="button"
              aria-pressed={gruppo.tipo === gruppoMostrato}
              onclick={() => (gruppoScelto = gruppo.tipo)}
            >
              {ETICHETTE_GRUPPO_SCADUTI[gruppo.tipo]} ({gruppo.numero})
            </button>
          {/each}
        </div>
        {#key gruppoMostrato}
          <ElencoScadenze
            tipo={gruppoMostrato}
            stato="scaduto"
            anteprima
            compatto
            onCambiato={carica}
          />
        {/key}
      </section>
    {/if}

    <section class="pannello">
      <h2>Prossime scadenze <span class="nota">(30 giorni)</span></h2>
      {#if riepilogo.prossimeScadenze.length === 0}
        <p class="vuoto">Nessuna scadenza nei prossimi 30 giorni.</p>
      {:else}
        <table bind:this={tabellaProssime} class="tabella riquadrata compatta prossime">
          <thead>
            <tr><th>Cognome e nome</th><th>Cosa scade</th><th>Quando</th><th></th></tr>
          </thead>
          <tbody>
            {#each prossimeMostrate as scadenza (scadenza.tipo + scadenza.iscrittoId)}
              <tr>
                <td class="elastica">
                  <a
                    href={resolve("/iscritti/[id]", { id: String(scadenza.iscrittoId) })}
                    title="{scadenza.cognome} {scadenza.nome}"
                  >
                    {scadenza.cognome}
                    {scadenza.nome}
                  </a>
                </td>
                <td>{ETICHETTE_TIPO_SCADENZA[scadenza.tipo]}</td>
                <td>
                  {descriviScadenza(scadenza.dataScadenza, oggi)}
                  <span class="distanza">· {formattaDataBreve(scadenza.dataScadenza, oggi)}</span>
                </td>
                <td class="azione">
                  {#if eRinnovabile(scadenza.tipo)}
                    {@const tipo = scadenza.tipo}
                    <button
                      type="button"
                      class="pulsante piccolo"
                      onclick={() => (daRinnovare = { iscrittoId: scadenza.iscrittoId, tipo })}
                    >
                      Rinnova
                    </button>
                  {:else}
                    <a
                      class="pulsante piccolo secondario"
                      href={resolve("/iscritti/[id]", { id: String(scadenza.iscrittoId) })}
                    >
                      Aggiorna
                    </a>
                  {/if}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
        <a class="tutte" href={resolve("/scadenze")}>Tutte le scadenze →</a>
      {/if}
    </section>
  </div>

  <CassaDiOggi aggiornamento={rinnoviRegistrati} />
{/if}

{#if daRinnovare}
  <FinestraRinnovo
    iscrittoId={daRinnovare.iscrittoId}
    tipo={daRinnovare.tipo}
    onChiusa={chiudiRinnovo}
  />
{/if}

<style>
  .titolo {
    margin: 0 0 0.9rem;
  }

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
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 0.75rem;
    margin-bottom: 1.25rem;
  }

  /* Scaduti e prossime scadenze affiancati: tutto ciò che va fatto è visibile senza scorrere */
  .pannelli {
    display: grid;
    gap: 1.25rem;
    align-items: start;
    margin-bottom: 1.5rem;
  }

  @media (min-width: 1100px) {
    .pannelli.affiancati {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  .pannello {
    display: grid;
    gap: 0.6rem;
    min-width: 0;
  }

  .segmenti {
    justify-self: start;
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

  .azione {
    width: 1%;
    text-align: right;
  }

  .tutte {
    justify-self: end;
    color: var(--colore-primario);
    text-decoration: none;
  }

  /* Bordo giallo come gli elenchi in scadenza, rosso per i già scaduti */
  .prossime {
    border-left: 4px solid #e0a100;
  }

  .vuoto {
    margin: 0;
    color: var(--colore-testo-secondario);
  }
</style>
