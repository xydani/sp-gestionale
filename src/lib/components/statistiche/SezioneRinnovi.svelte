<script lang="ts">
  import { resolve } from "$app/paths";
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

  /** All'inizio si vedono pochi nomi: l'elenco completo si apre a richiesta. */
  const PERSI_VISIBILI = 10;
  let tuttiIPersi = $state(false);

  const abbastanzaDati = $derived(bastanoPerUnTasso(abbonamenti.totale));
  const tassoAbbandono = $derived(
    abbonamenti.tassoRinnovo === null ? null : 1 - abbonamenti.tassoRinnovo,
  );
  const persiMostrati = $derived(tuttiIPersi ? persi : persi.slice(0, PERSI_VISIBILI));
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

  {#if !abbastanzaDati}
    <p class="pochi-dati">
      Servono almeno 5 abbonamenti scaduti e già decisi nel periodo per una percentuale affidabile:
      per ora sono {scadenzeDecise(abbonamenti.totale)}.
    </p>
  {:else}
    <div class="riquadri">
      <RiquadroNumero
        etichetta="Rinnovo abbonamenti"
        valore={formattaTasso(abbonamenti.tassoRinnovo)}
        tono="attivo"
      >
        {#snippet dettaglio()}
          {abbonamenti.totale.rinnovati} rinnovati su {scadenzeDecise(abbonamenti.totale)}
        {/snippet}
      </RiquadroNumero>
      <RiquadroNumero etichetta="Abbandono" valore={formattaTasso(tassoAbbandono)} tono="scaduto">
        {#snippet dettaglio()}
          {abbonamenti.totale.persi} non hanno rinnovato · {abbonamenti.totale.inAttesa} in attesa
        {/snippet}
      </RiquadroNumero>
      <RiquadroNumero
        etichetta="Rinnovo quota annuale"
        valore={bastanoPerUnTasso(iscrizioni.totale) ? formattaTasso(iscrizioni.tassoRinnovo) : "—"}
      >
        {#snippet dettaglio()}
          {iscrizioni.totale.rinnovati} rinnovate su {scadenzeDecise(iscrizioni.totale)}
        {/snippet}
      </RiquadroNumero>
    </div>

    <h3>Rinnovo degli abbonamenti mese per mese</h3>
    <p class="spiegazione">Le colonne chiare sono mesi con meno di 5 scadenze: poco indicativi.</p>
    <GraficoColonne
      voci={colonne}
      massimo={1}
      etichettaMassimo="100%"
      titoloAccessibile="Percentuale di rinnovo per mese di scadenza"
    />
  {/if}

  {#if persi.length > 0}
    <h3>Chi non ha rinnovato ({persi.length})</h3>
    <table class="tabella riquadrata">
      <thead>
        <tr><th>Cognome e nome</th><th>Telefono</th><th>Abbonamento</th><th>Scaduto il</th></tr>
      </thead>
      <tbody>
        {#each persiMostrati as perso (perso.iscrittoId + perso.dataScadenza)}
          <tr>
            <td>
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
    {#if persi.length > PERSI_VISIBILI}
      <button type="button" class="pulsante-testo" onclick={() => (tuttiIPersi = !tuttiIPersi)}>
        {tuttiIPersi ? "Mostra meno" : `Mostra tutti (${persi.length})`}
      </button>
    {/if}
  {/if}
</section>

<style>
  .riquadri {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
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

  .pulsante-testo {
    justify-self: start;
  }
</style>
