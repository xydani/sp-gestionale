<script lang="ts">
  import { resolve } from "$app/paths";
  import { paginaDiElenco } from "$lib/paginazione";
  import { RIGHE_MINIME, seguiFinestra } from "$lib/righe-visibili";
  import Paginazione from "../Paginazione.svelte";
  import { formattaData, formattaPeriodo } from "$lib/format";
  import { bastanoPerUnTasso, formattaTasso, meseBreve, scadenzeDecise } from "$lib/statistiche";
  import type { IscrittoPerso, Rinnovi } from "$lib/types";
  import GraficoColonne from "../GraficoColonne.svelte";
  import RiquadroNumero from "../RiquadroNumero.svelte";

  let {
    abbonamenti,
    iscrizioni,
    persi,
    giorniTolleranza,
  }: {
    abbonamenti: Rinnovi;
    iscrizioni: Rinnovi;
    persi: IscrittoPerso[];
    giorniTolleranza: number;
  } = $props();

  /** Sotto l'elenco: le pagine, il bordo della scheda e il margine della pagina. */
  const SPAZIO_SOTTO_PX = 100;
  let tabellaPersi = $state<HTMLTableElement>();
  let numeroPaginaPersi = $state(1);
  let righePersi = $state(RIGHE_MINIME);
  const paginaPersi = $derived(paginaDiElenco(persi, numeroPaginaPersi, righePersi));

  $effect(() => {
    if (!tabellaPersi) return;
    return seguiFinestra(tabellaPersi, SPAZIO_SOTTO_PX, {
      numeroPagina: () => numeroPaginaPersi,
      carica: (numero, righe) => {
        righePersi = righe;
        numeroPaginaPersi = numero;
      },
    });
  });

  const abbastanzaDati = $derived(bastanoPerUnTasso(abbonamenti.totale));
  const tassoAbbandono = $derived(
    abbonamenti.tassoRinnovo === null ? null : 1 - abbonamenti.tassoRinnovo,
  );
  const colonne = $derived(
    abbonamenti.perMese.map((mese) => ({
      etichetta: meseBreve(mese.mese),
      valore: tassoDelMese(mese.conteggio),
      descrizione: descriviMese(mese.mese, mese.conteggio),
      debole: !bastanoPerUnTasso(mese.conteggio),
    })),
  );

  function tassoDelMese(conteggio: Rinnovi["totale"]): number {
    const decise = scadenzeDecise(conteggio);
    return decise === 0 ? 0 : conteggio.rinnovati / decise;
  }

  function descriviMese(chiave: string, conteggio: Rinnovi["totale"]): string {
    const decise = scadenzeDecise(conteggio);
    if (decise === 0) return `${formattaPeriodo(chiave)}: nessuna scadenza decisa`;
    return (
      `${formattaPeriodo(chiave)}: ${formattaTasso(tassoDelMese(conteggio))} rinnovati ` +
      `(${conteggio.rinnovati} su ${decise})` +
      (bastanoPerUnTasso(conteggio) ? "" : " · pochi dati")
    );
  }
</script>

<section class="scheda">
  <h2>Rinnovi e abbandoni</h2>
  <p class="spiegazione">
    Per ogni abbonamento scaduto nel periodo: è rinnovato se la persona riparte entro
    {giorniTolleranza} giorni dalla scadenza, altrimenti è un abbandono. Chi è scaduto da meno tempo è
    ancora «in attesa» e non entra nelle percentuali.
  </p>

  <!-- Percentuali e grafico a sinistra, chi non ha rinnovato a destra -->
  <div class="colonne">
    <div class="colonna">
      {#if !abbastanzaDati}
        <p class="pochi-dati">
          Servono almeno 5 abbonamenti scaduti e già decisi nel periodo per una percentuale
          affidabile: per ora sono {scadenzeDecise(abbonamenti.totale)}.
        </p>
      {:else}
        <div class="riquadri">
          <RiquadroNumero
            compatto
            etichetta="Rinnovo abbonamenti"
            valore={formattaTasso(abbonamenti.tassoRinnovo)}
            tono="attivo"
          >
            {#snippet dettaglio()}
              {abbonamenti.totale.rinnovati} rinnovati su {scadenzeDecise(abbonamenti.totale)}
            {/snippet}
          </RiquadroNumero>
          <RiquadroNumero
            compatto
            etichetta="Abbandono"
            valore={formattaTasso(tassoAbbandono)}
            tono="scaduto"
          >
            {#snippet dettaglio()}
              {abbonamenti.totale.persi} non hanno rinnovato · {abbonamenti.totale.inAttesa} in attesa
            {/snippet}
          </RiquadroNumero>
          <RiquadroNumero
            compatto
            etichetta="Rinnovo quota annuale"
            valore={bastanoPerUnTasso(iscrizioni.totale)
              ? formattaTasso(iscrizioni.tassoRinnovo)
              : "—"}
          >
            {#snippet dettaglio()}
              {iscrizioni.totale.rinnovati} rinnovate su {scadenzeDecise(iscrizioni.totale)}
            {/snippet}
          </RiquadroNumero>
        </div>

        <h3>Rinnovo degli abbonamenti mese per mese</h3>
        <p class="spiegazione">
          Le colonne chiare sono mesi con meno di 5 scadenze: poco indicativi.
        </p>
        <GraficoColonne
          voci={colonne}
          massimo={1}
          etichettaMassimo="100%"
          titoloAccessibile="Percentuale di rinnovo per mese di scadenza"
          altezzaArea="8rem"
        />
      {/if}
    </div>

    {#if persi.length > 0}
      <div class="colonna">
        <h3>Chi non ha rinnovato ({persi.length})</h3>
        <table bind:this={tabellaPersi} class="tabella riquadrata compatta">
          <thead>
            <tr><th>Cognome e nome</th><th>Telefono</th><th>Abbonamento</th><th>Scaduto il</th></tr>
          </thead>
          <tbody>
            {#each paginaPersi.elementi as perso (perso.iscrittoId + perso.dataScadenza)}
              <tr>
                <td class="elastica">
                  <a href={resolve("/iscritti/[id]", { id: String(perso.iscrittoId) })}>
                    {perso.cognome}
                    {perso.nome}
                  </a>
                  {#if perso.archiviato}<span class="nota">archiviato</span>{/if}
                </td>
                <td>{perso.telefono ?? "—"}</td>
                <td>{perso.tipologiaNome}</td>
                <td>{formattaData(perso.dataScadenza)}</td>
              </tr>
            {/each}
          </tbody>
        </table>
        {#if paginaPersi.totaleElementi > paginaPersi.elementiPerPagina}
          <Paginazione pagina={paginaPersi} onCambia={(numero) => (numeroPaginaPersi = numero)} />
        {/if}
      </div>
    {/if}
  </div>
</section>

<style>
  .colonne {
    display: grid;
    gap: 1.25rem;
    align-items: start;
  }

  @media (min-width: 1100px) {
    .colonne {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  .colonna {
    display: grid;
    gap: 0.75rem;
    min-width: 0;
  }

  .riquadri {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 0.75rem;
  }

  h3 {
    margin: 0.5rem 0 0;
    font-size: 1rem;
  }

  td a {
    color: inherit;
    font-weight: 500;
    text-decoration: none;
  }

  td a:hover {
    color: var(--colore-primario);
  }

  .nota {
    margin-left: 0.4rem;
    color: var(--colore-testo-secondario);
    font-size: 0.85rem;
  }
</style>
