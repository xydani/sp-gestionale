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
  import FormIscritto from "$lib/components/FormIscritto.svelte";
  import SezioneAbbonamento from "$lib/components/SezioneAbbonamento.svelte";
  import SezioneIscrizione from "$lib/components/SezioneIscrizione.svelte";
  import { formattaData } from "$lib/format";
  import type { DatiIscritto, Iscritto } from "$lib/types";

  const idIscritto = $derived(Number(page.params.id));

  let iscritto = $state<Iscritto | null>(null);
  let inModifica = $state(false);
  let errore = $state<string | null>(null);

  $effect(() => {
    carica(idIscritto);
  });

  /** Ricarica i dati e gli stati dopo ogni modifica di iscrizione o abbonamento. */
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
      {#if iscritto.archiviato}<span class="archiviato">Archiviato</span>{/if}
    </div>

    <section class="scheda">
      <h2>Anagrafica</h2>
      {#if inModifica}
        <FormIscritto
          iniziale={{
            ...iscritto,
            dataNascita: iscritto.dataNascita ?? "",
            luogoNascita: iscritto.luogoNascita ?? "",
          }}
          etichettaConferma="Salva modifiche"
          onConferma={salvaModifiche}
          onAnnulla={() => (inModifica = false)}
        />
      {:else}
        <dl>
          <dt>Nato/a il</dt>
          <dd>
            {iscritto.dataNascita ? formattaData(iscritto.dataNascita) : "—"}
            {#if iscritto.luogoNascita}a {iscritto.luogoNascita}{/if}
          </dd>
          <dt>Telefono</dt>
          <dd>{iscritto.telefono ?? "—"}</dd>
          <dt>Email</dt>
          <dd>{iscritto.email ?? "—"}</dd>
          <dt>Note</dt>
          <dd class="note">{iscritto.note ?? "—"}</dd>
        </dl>
        {#if !iscritto.dataNascita || !iscritto.luogoNascita}
          <p class="avviso">Mancano data o luogo di nascita: completali con "Modifica dati".</p>
        {/if}
        <div class="azioni">
          <button class="pulsante" type="button" onclick={() => (inModifica = true)}
            >Modifica dati</button
          >
          <button type="button" class="pulsante secondario" onclick={alternaArchiviazione}>
            {iscritto.archiviato ? "Ripristina iscritto" : "Archivia iscritto"}
          </button>
        </div>
      {/if}
    </section>

    {#key iscritto.id}
      <SezioneIscrizione {iscritto} onCambiata={() => carica(idIscritto)} />
      <SezioneAbbonamento {iscritto} onCambiato={() => carica(idIscritto)} />
    {/key}
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
    border-radius: var(--raggio-grande);
    box-shadow: var(--ombra-scheda);
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

  .avviso {
    margin: 0;
    color: var(--colore-avviso);
  }

  .azioni {
    display: flex;
    gap: 0.5rem;
  }

  .errore {
    color: var(--colore-errore);
  }
</style>
