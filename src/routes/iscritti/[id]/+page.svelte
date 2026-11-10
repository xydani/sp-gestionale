<script lang="ts">
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import {
    impostaIscrittoArchiviato,
    messaggioErrore,
    modificaIscritto,
    trovaIscritto,
  } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import BadgeStato from "$lib/components/BadgeStato.svelte";
  import FormIscritto from "$lib/components/FormIscritto.svelte";
  import { formattaData } from "$lib/format";
  import type { DatiIscritto, Iscritto } from "$lib/types";

  const idIscritto = $derived(Number(page.params.id));

  let iscritto = $state<Iscritto | null>(null);
  let inModifica = $state(false);
  let errore = $state<string | null>(null);

  $effect(() => {
    carica(idIscritto);
  });

  async function carica(id: number) {
    try {
      iscritto = await trovaIscritto(id);
      errore = null;
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
    }
  }

  async function salvaModifiche(dati: DatiIscritto): Promise<boolean> {
    if (!iscritto) return false;
    try {
      iscritto = await modificaIscritto(iscritto.id, dati);
      inModifica = false;
      mostraAvviso("successo", "Dati aggiornati");
      return true;
    } catch (erroreBackend) {
      mostraAvviso("errore", messaggioErrore(erroreBackend));
      return false;
    }
  }

  async function alternaArchiviazione() {
    if (!iscritto) return;
    try {
      iscritto = await impostaIscrittoArchiviato(iscritto.id, !iscritto.archiviato);
      mostraAvviso(
        "successo",
        iscritto.archiviato ? "Iscritto archiviato" : "Iscritto ripristinato",
      );
    } catch (erroreBackend) {
      mostraAvviso("errore", messaggioErrore(erroreBackend));
    }
  }
</script>

<div class="pagina-stretta">
  <a class="indietro" href={resolve("/iscritti")}>← Iscritti</a>

  {#if errore}
    <p class="errore">{errore}</p>
  {:else if iscritto}
    <div class="intestazione">
      <h1>{iscritto.nome} {iscritto.cognome}</h1>
      <BadgeStato stato={iscritto.stato} />
      {#if iscritto.archiviato}<span class="archiviato">Archiviato</span>{/if}
    </div>

    <section class="scheda">
      {#if inModifica}
        <FormIscritto
          iniziale={iscritto}
          etichettaConferma="Salva modifiche"
          onConferma={salvaModifiche}
          onAnnulla={() => (inModifica = false)}
        />
      {:else}
        <dl>
          <dt>Telefono</dt>
          <dd>{iscritto.telefono ?? "—"}</dd>
          <dt>Iscritto dal</dt>
          <dd>{formattaData(iscritto.dataIscrizione)}</dd>
          <dt>Ultima scadenza</dt>
          <dd>{iscritto.ultimaDataScadenza ? formattaData(iscritto.ultimaDataScadenza) : "—"}</dd>
          <dt>Note</dt>
          <dd class="note">{iscritto.note ?? "—"}</dd>
        </dl>
        <div class="azioni">
          <button type="button" onclick={() => (inModifica = true)}>Modifica dati</button>
          <button type="button" class="secondario" onclick={alternaArchiviazione}>
            {iscritto.archiviato ? "Ripristina iscritto" : "Archivia iscritto"}
          </button>
        </div>
      {/if}
    </section>

    <section class="scheda">
      <h2>Pagamenti</h2>
      <p class="nota">Lo storico dei pagamenti e il rinnovo arrivano nella prossima fase.</p>
    </section>
  {/if}
</div>

<style>
  .indietro {
    display: inline-block;
    margin-bottom: 0.5rem;
    color: var(--colore-primario);
    text-decoration: none;
  }

  .intestazione {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.75rem;
  }

  .archiviato {
    color: var(--colore-testo-secondario);
  }

  .scheda {
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

  dl {
    display: grid;
    grid-template-columns: max-content 1fr;
    gap: 0.6rem 1.5rem;
    margin: 0;
  }

  dt {
    color: var(--colore-testo-secondario);
  }

  dd {
    margin: 0;
  }

  .note {
    white-space: pre-wrap;
  }

  .azioni {
    display: flex;
    gap: 0.5rem;
  }

  button {
    padding: 0.55rem 1.1rem;
    border-radius: var(--raggio);
    border: 1px solid var(--colore-primario);
    background: var(--colore-primario);
    color: var(--colore-primario-testo);
  }

  button.secondario {
    background: transparent;
    color: var(--colore-primario);
  }

  .nota {
    margin: 0;
    color: var(--colore-testo-secondario);
  }

  .errore {
    color: var(--colore-errore);
  }
</style>
