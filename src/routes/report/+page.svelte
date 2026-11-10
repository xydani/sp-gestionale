<script lang="ts">
  import Tendina from "$lib/components/Tendina.svelte";
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
  const OPZIONI_RAGGRUPPAMENTO = (Object.keys(ETICHETTE_RAGGRUPPAMENTO) as Raggruppamento[]).map(
    (valore) => ({ valore, etichetta: ETICHETTE_RAGGRUPPAMENTO[valore] }),
  );
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

<div class="segmenti periodi-rapidi" role="group" aria-label="Periodo">
  {#each PERIODI_RAPIDI as scelta (scelta)}
    <button
      type="button"
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
  <div class="raggruppamento">
    <Tendina
      bind:valore={raggruppamento}
      opzioni={OPZIONI_RAGGRUPPAMENTO}
      etichettaAccessibile="Raggruppamento"
      onCambia={carica}
    />
  </div>
  <button
    type="button"
    class="pulsante secondario esporta"
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
      <table class="tabella riquadrata">
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
      <table class="tabella riquadrata">
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
  .raggruppamento {
    width: 10rem;
  }

  h2 {
    margin: 0 0 0.75rem;
    font-size: 1.15rem;
  }

  section {
    margin-top: 2rem;
  }

  .periodi-rapidi {
    margin-bottom: 0.75rem;
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
    width: 10rem;
  }

  .esporta {
    margin-left: auto;
  }

  .riquadri {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(210px, 1fr));
    gap: 1rem;
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
