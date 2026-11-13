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
  import { descriviPrezzoAlMese, dividiTipologie } from "$lib/listino";
  import type { DatiTipologia, TipologiaAbbonamento } from "$lib/types";
  import FinestraSovrapposta from "./FinestraSovrapposta.svelte";
  import FormTipologia from "./FormTipologia.svelte";

  let tipologie = $state<TipologiaAbbonamento[]>([]);
  let caricato = $state(false);
  /** La tipologia da modificare, «nuova» per aggiungerne una, null a modulo chiuso. */
  let moduloAperto = $state<TipologiaAbbonamento | "nuova" | null>(null);
  /** Le disattivate servono di rado: restano chiuse per non affollare il listino. */
  let disattivateAperte = $state(false);

  const divise = $derived(dividiTipologie(tipologie));

  async function ricarica() {
    tipologie = await elencaTipologie(false);
    caricato = true;
  }

  /** Esegue un'operazione, mostra l'eventuale errore e ricarica l'elenco. */
  async function esegui(operazione: () => Promise<unknown>, successo: string) {
    try {
      await operazione();
      await ricarica();
      moduloAperto = null;
      mostraAvviso("successo", successo);
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }

  function salva(dati: DatiTipologia) {
    const daModificare = moduloAperto;
    if (daModificare === null) return Promise.resolve();
    if (daModificare === "nuova") {
      return esegui(() => creaTipologia(dati), `${dati.nome} aggiunta al listino`);
    }
    return esegui(() => modificaTipologia(daModificare.id, dati), `${dati.nome} aggiornata`);
  }

  function alternaAttiva(tipologia: TipologiaAbbonamento) {
    return esegui(
      () => impostaTipologiaAttiva(tipologia.id, !tipologia.attiva),
      tipologia.attiva
        ? `${tipologia.nome} disattivata: la trovi tra le disattivate`
        : `${tipologia.nome} di nuovo in vendita`,
    );
  }

  onMount(ricarica);
</script>

{#snippet righe(elenco: TipologiaAbbonamento[])}
  {#each elenco as tipologia (tipologia.id)}
    {@const alMese = descriviPrezzoAlMese(tipologia)}
    <tr>
      <td class="nome">{tipologia.nome}</td>
      <td>{descriviDurata(tipologia.durataValore, tipologia.durataUnita)}</td>
      <td class="prezzo">
        {formattaEuro(tipologia.prezzoCent)}
        {#if alMese}<span class="al-mese">· {alMese}</span>{/if}
      </td>
      <td class="azioni">
        <button type="button" class="pulsante-testo" onclick={() => (moduloAperto = tipologia)}>
          Modifica
        </button>
        <button type="button" class="pulsante-testo" onclick={() => alternaAttiva(tipologia)}>
          {tipologia.attiva ? "Disattiva" : "Riattiva"}
        </button>
      </td>
    </tr>
  {/each}
{/snippet}

<section class="scheda">
  <div class="intestazione-sezione">
    <h2>Abbonamenti</h2>
    <span class="spazio"></span>
    <button type="button" class="pulsante" onclick={() => (moduloAperto = "nuova")}>
      + Nuova tipologia
    </button>
  </div>
  <p class="nota">
    Le modifiche valgono per i nuovi pagamenti: lo storico mantiene i prezzi del momento. Una
    tipologia non più in uso si disattiva invece di cancellarla.
  </p>

  {#if caricato && divise.attive.length === 0}
    <p class="avviso">Nessun abbonamento in vendita: aggiungine uno per registrare i pagamenti.</p>
  {/if}

  {#if tipologie.length > 0}
    <table class="tabella compatta">
      <thead>
        <tr><th>Nome</th><th>Durata</th><th>Prezzo</th><th></th></tr>
      </thead>
      <tbody>{@render righe(divise.attive)}</tbody>
      {#if divise.disattivate.length > 0}
        <tbody class="disattivate">
          <tr>
            <td colspan="4" class="riga-apertura">
              <button
                type="button"
                class="pulsante-testo"
                aria-expanded={disattivateAperte}
                onclick={() => (disattivateAperte = !disattivateAperte)}
              >
                {disattivateAperte
                  ? "Nascondi le disattivate"
                  : `Disattivate, non più in vendita (${divise.disattivate.length})`}
              </button>
            </td>
          </tr>
          {#if disattivateAperte}{@render righe(divise.disattivate)}{/if}
        </tbody>
      {/if}
    </table>
  {/if}
</section>

{#if moduloAperto !== null}
  <FinestraSovrapposta
    titolo={moduloAperto === "nuova" ? "Nuova tipologia" : "Modifica tipologia"}
    sottotitolo={moduloAperto === "nuova" ? null : moduloAperto.nome}
    onChiusa={() => (moduloAperto = null)}
  >
    <FormTipologia
      iniziale={moduloAperto === "nuova" ? undefined : moduloAperto}
      etichettaConferma={moduloAperto === "nuova" ? "Aggiungi" : "Salva"}
      onConferma={salva}
      onAnnulla={() => (moduloAperto = null)}
    />
  </FinestraSovrapposta>
{/if}

<style>
  section {
    margin-bottom: 1.5rem;
  }

  .intestazione-sezione {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .spazio {
    flex: 1;
  }

  h2 {
    margin: 0;
    font-size: 1.15rem;
  }

  p {
    margin: 0;
  }

  .nota {
    color: var(--colore-testo-secondario);
  }

  .avviso {
    color: var(--colore-avviso);
  }

  .nome {
    font-weight: 500;
  }

  .al-mese {
    margin-left: 0.3rem;
    color: var(--colore-testo-secondario);
    font-size: 0.9rem;
  }

  .azioni {
    text-align: right;
    white-space: nowrap;
  }

  .riga-apertura {
    padding-left: 0.4rem;
  }

  .disattivate td {
    color: var(--colore-testo-secondario);
  }
</style>
