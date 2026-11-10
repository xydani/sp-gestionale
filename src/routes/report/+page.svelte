<script lang="ts">
  import { save } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";
  import { esportaIncassiCsv, leggiRiepilogoIncassi, messaggioErrore } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import CampoData from "$lib/components/CampoData.svelte";
  import RiquadroNumero from "$lib/components/RiquadroNumero.svelte";
  import { formattaEuro, formattaPercentuale, formattaPeriodo } from "$lib/format";
  import { ETICHETTE_METODO } from "$lib/metodi-pagamento";
  import {
    ETICHETTE_PERIODO_RAPIDO,
    calcolaPeriodoRapido,
    raggruppamentoAdatto,
    type PeriodoRapido,
  } from "$lib/periodo";
  import type { Raggruppamento, RiepilogoIncassi } from "$lib/types";

  const ETICHETTE_RAGGRUPPAMENTO: Record<Raggruppamento, string> = {
    giorno: "Per giorno",
    mese: "Per mese",
    anno: "Per anno",
  };
  const PERIODI_RAPIDI = Object.keys(ETICHETTE_PERIODO_RAPIDO) as PeriodoRapido[];

  const iniziale = calcolaPeriodoRapido("mese");
  let dal = $state(iniziale.dal);
  let al = $state(iniziale.al);
  let periodoRapido = $state<PeriodoRapido | null>("mese");
  let raggruppamento = $state<Raggruppamento>(raggruppamentoAdatto(iniziale));
  let riepilogo = $state<RiepilogoIncassi | null>(null);

  const totaleMassimoPeriodo = $derived(
    Math.max(0, ...(riepilogo?.perPeriodo.map((riga) => riga.incassi.totaleCent) ?? [])),
  );

  async function carica() {
    if (!dal || !al || dal > al) return;
    try {
      riepilogo = await leggiRiepilogoIncassi(dal, al, raggruppamento);
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }

  function scegliPeriodoRapido(scelta: PeriodoRapido) {
    ({ dal, al } = calcolaPeriodoRapido(scelta));
    periodoRapido = scelta;
    raggruppamento = raggruppamentoAdatto({ dal, al });
    carica();
  }

  function cambiaDate() {
    periodoRapido = null;
    if (dal && al) raggruppamento = raggruppamentoAdatto({ dal, al });
    carica();
  }

  async function esporta() {
    const percorso = await save({
      defaultPath: `incassi_${dal}_${al}.csv`,
      filters: [{ name: "CSV", extensions: ["csv"] }],
    });
    if (!percorso) return;
    try {
      await esportaIncassiCsv(dal, al, percorso);
      mostraAvviso("successo", "Esportazione completata");
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }

  onMount(carica);
</script>

<h1>Incassi</h1>

<div class="periodi-rapidi" role="group" aria-label="Periodo">
  {#each PERIODI_RAPIDI as scelta (scelta)}
    <button
      type="button"
      class:scelto={periodoRapido === scelta}
      aria-pressed={periodoRapido === scelta}
      onclick={() => scegliPeriodoRapido(scelta)}>{ETICHETTE_PERIODO_RAPIDO[scelta]}</button
    >
  {/each}
</div>

<div class="filtri">
  <label
    >Dal <span class="campo-data"><CampoData bind:valore={dal} onCambia={cambiaDate} /></span
    ></label
  >
  <label
    >Al <span class="campo-data"
      ><CampoData bind:valore={al} minimaIso={dal} onCambia={cambiaDate} /></span
    ></label
  >
  <select bind:value={raggruppamento} onchange={carica} aria-label="Raggruppamento">
    {#each Object.entries(ETICHETTE_RAGGRUPPAMENTO) as [valore, etichetta] (valore)}
      <option value={valore}>{etichetta}</option>
    {/each}
  </select>
  <button
    type="button"
    class="esporta"
    onclick={esporta}
    disabled={!riepilogo || riepilogo.numeroMovimenti === 0}>Esporta CSV</button
  >
</div>

{#if riepilogo}
  <div class="riquadri">
    <RiquadroNumero etichetta="Totale incassato" valore={formattaEuro(riepilogo.totale.totaleCent)}>
      {#snippet dettaglio()}
        {riepilogo!.numeroMovimenti}
        {riepilogo!.numeroMovimenti === 1 ? "movimento" : "movimenti"}
      {/snippet}
    </RiquadroNumero>
    <RiquadroNumero etichetta="Abbonamenti" valore={formattaEuro(riepilogo.totale.abbonamentiCent)}>
      {#snippet dettaglio()}
        {formattaPercentuale(riepilogo!.totale.abbonamentiCent, riepilogo!.totale.totaleCent)} del totale
      {/snippet}
    </RiquadroNumero>
    <RiquadroNumero
      etichetta="Iscrizioni annuali"
      valore={formattaEuro(riepilogo.totale.iscrizioniCent)}
    >
      {#snippet dettaglio()}
        {formattaPercentuale(riepilogo!.totale.iscrizioniCent, riepilogo!.totale.totaleCent)} del totale
      {/snippet}
    </RiquadroNumero>
  </div>

  {#if riepilogo.numeroMovimenti === 0}
    <p class="vuoto">Nessun incasso nel periodo scelto.</p>
  {:else}
    <section>
      <h2>Per metodo di pagamento</h2>
      <table>
        <thead>
          <tr>
            <th>Metodo</th>
            <th class="importo">Abbonamenti</th>
            <th class="importo">Iscrizioni</th>
            <th class="importo">Totale</th>
            <th class="importo">Quota</th>
          </tr>
        </thead>
        <tbody>
          {#each riepilogo.perMetodo as riga (riga.metodo)}
            <tr>
              <td>{ETICHETTE_METODO[riga.metodo]}</td>
              <td class="importo">{formattaEuro(riga.incassi.abbonamentiCent)}</td>
              <td class="importo">{formattaEuro(riga.incassi.iscrizioniCent)}</td>
              <td class="importo totale">{formattaEuro(riga.incassi.totaleCent)}</td>
              <td class="importo"
                >{formattaPercentuale(riga.incassi.totaleCent, riepilogo.totale.totaleCent)}</td
              >
            </tr>
          {/each}
        </tbody>
      </table>
    </section>

    <section>
      <h2>{ETICHETTE_RAGGRUPPAMENTO[raggruppamento]}</h2>
      <table>
        <thead>
          <tr>
            <th>Periodo</th>
            <th class="importo">Abbonamenti</th>
            <th class="importo">Iscrizioni</th>
            <th class="importo">Totale</th>
            <th class="colonna-barra"><span class="nascosto">Andamento</span></th>
          </tr>
        </thead>
        <tbody>
          {#each riepilogo.perPeriodo as riga (riga.periodo)}
            <tr>
              <td>{formattaPeriodo(riga.periodo)}</td>
              <td class="importo">{formattaEuro(riga.incassi.abbonamentiCent)}</td>
              <td class="importo">{formattaEuro(riga.incassi.iscrizioniCent)}</td>
              <td class="importo totale">{formattaEuro(riga.incassi.totaleCent)}</td>
              <td class="colonna-barra" aria-hidden="true">
                <span
                  class="barra"
                  style:width="{(riga.incassi.totaleCent / totaleMassimoPeriodo) * 100}%"
                ></span>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </section>
  {/if}
{/if}

<style>
  h2 {
    margin: 0 0 0.75rem;
    font-size: 1.15rem;
  }

  section {
    margin-top: 2rem;
  }

  .periodi-rapidi {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
    margin-bottom: 0.75rem;
  }

  .periodi-rapidi button {
    padding: 0.4rem 0.9rem;
    border: 1px solid var(--colore-bordo);
    border-radius: 999px;
    background: var(--colore-superficie);
    font: inherit;
    cursor: pointer;
  }

  .periodi-rapidi button.scelto {
    background: var(--colore-primario);
    border-color: var(--colore-primario);
    color: var(--colore-primario-testo);
  }

  .filtri {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .filtri label {
    display: flex;
    gap: 0.4rem;
    align-items: center;
  }

  .campo-data {
    width: 8.5rem;
  }

  select {
    padding: 0.5rem 0.7rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
    background: var(--colore-superficie);
  }

  .esporta {
    margin-left: auto;
    padding: 0.55rem 1.1rem;
    border-radius: var(--raggio);
    border: 1px solid var(--colore-primario);
    background: var(--colore-superficie);
    color: var(--colore-primario);
    font: inherit;
    cursor: pointer;
  }

  .esporta:disabled {
    opacity: 0.5;
    cursor: default;
  }

  .riquadri {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(210px, 1fr));
    gap: 1rem;
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
    padding: 0.55rem 1rem;
    border-bottom: 1px solid var(--colore-bordo);
  }

  th {
    color: var(--colore-testo-secondario);
    font-weight: 500;
    background: var(--colore-sfondo);
  }

  .importo {
    text-align: right;
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
  }

  .totale {
    font-weight: 600;
  }

  .colonna-barra {
    width: 28%;
  }

  .barra {
    display: block;
    height: 8px;
    min-width: 2px;
    border-radius: 0 4px 4px 0;
    background: var(--colore-primario);
  }

  .nascosto {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
  }

  .vuoto {
    margin-top: 1.5rem;
    color: var(--colore-testo-secondario);
  }
</style>
