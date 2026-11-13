<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { tick } from "svelte";
  import {
    eliminaIscrittoDefinitivamente,
    impostaIscrittoArchiviato,
    messaggioErrore,
    modificaIscritto,
    trovaIscritto,
  } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import BadgeStato from "$lib/components/BadgeStato.svelte";
  import FormIscritto from "$lib/components/FormIscritto.svelte";
  import { confermaEliminazioneIscritto } from "$lib/conferma";
  import SezioneAbbonamento from "$lib/components/SezioneAbbonamento.svelte";
  import SezioneIscrizione from "$lib/components/SezioneIscrizione.svelte";
  import StrisciaCosaManca from "$lib/components/StrisciaCosaManca.svelte";
  import type { AzioneMancanza } from "$lib/cosa-manca";
  import { formattaData } from "$lib/format";
  import type { DatiIscritto, Iscritto } from "$lib/types";

  const idIscritto = $derived(Number(page.params.id));

  let iscritto = $state<Iscritto | null>(null);
  let inModifica = $state(false);
  let errore = $state<string | null>(null);
  let rinnovoIscrizioneAperto = $state(false);
  let rinnovoAbbonamentoAperto = $state(false);

  const SEZIONI_AZIONE: Record<AzioneMancanza, string> = {
    anagrafica: "sezione-anagrafica",
    iscrizione: "sezione-iscrizione",
    abbonamento: "sezione-abbonamento",
  };

  /** Dalla striscia «Da sistemare»: apre il modulo giusto e ci porta la vista. */
  async function sistema(azione: AzioneMancanza) {
    if (azione === "anagrafica") inModifica = true;
    if (azione === "iscrizione") rinnovoIscrizioneAperto = true;
    if (azione === "abbonamento") rinnovoAbbonamentoAperto = true;
    await tick();
    const sezione = document.getElementById(SEZIONI_AZIONE[azione]);
    sezione?.scrollIntoView({ behavior: "smooth", block: "start" });
  }

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

  async function eliminaDefinitivamente() {
    if (!iscritto || !(await confermaEliminazioneIscritto(iscritto))) return;
    try {
      await eliminaIscrittoDefinitivamente(iscritto.id);
      mostraAvviso("successo", "Dati dell'iscritto eliminati definitivamente");
      await goto(resolve("/iscritti"));
    } catch (erroreBackend) {
      mostraAvviso("errore", messaggioErrore(erroreBackend));
    }
  }
</script>

<div>
  {#if errore}
    <a class="indietro" href={resolve("/iscritti")}>← Iscritti</a>
    <p class="errore">{errore}</p>
  {:else if iscritto}
    <div class="intestazione">
      <a class="indietro" href={resolve("/iscritti")}>← Iscritti</a>
      <h1>{iscritto.nome} {iscritto.cognome}</h1>
      {#if iscritto.archiviato}<span class="archiviato">Archiviato</span>{/if}
    </div>

    <StrisciaCosaManca {iscritto} onAzione={sistema} />

    <!-- Anagrafica a sinistra; quota e abbonamento con lo storico a destra -->
    <div class="colonne">
      <div class="colonna">
        <section class="scheda" id="sezione-anagrafica">
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
              <dt>Certificato medico</dt>
              <dd class="certificato">
                {#if iscritto.dataScadenzaCertificato}
                  <BadgeStato stato={iscritto.statoCertificato} />
                  scade il {formattaData(iscritto.dataScadenzaCertificato)}
                {:else if iscritto.certificatoPresentato}
                  <BadgeStato stato="attivo" /> presentato, scadenza non indicata
                {:else}
                  <BadgeStato stato="nessuno" /> non presentato
                {/if}
              </dd>
              <dt>Note</dt>
              <dd class="note">{iscritto.note ?? "—"}</dd>
            </dl>
            <div class="azioni">
              <button class="pulsante" type="button" onclick={() => (inModifica = true)}
                >Modifica dati</button
              >
              <button type="button" class="pulsante secondario" onclick={alternaArchiviazione}>
                {iscritto.archiviato ? "Ripristina iscritto" : "Archivia iscritto"}
              </button>
              {#if iscritto.archiviato}
                <button type="button" class="pulsante pericolo" onclick={eliminaDefinitivamente}>
                  Elimina definitivamente
                </button>
              {/if}
            </div>
          {/if}
        </section>
      </div>
      <div class="colonna">
        {#key iscritto.id}
          <SezioneIscrizione
            {iscritto}
            bind:formAperto={rinnovoIscrizioneAperto}
            onCambiata={() => carica(idIscritto)}
          />
          <SezioneAbbonamento
            {iscritto}
            bind:formAperto={rinnovoAbbonamentoAperto}
            onCambiato={() => carica(idIscritto)}
          />
        {/key}
      </div>
    </div>
  {/if}
</div>

<style>
  .colonne {
    display: grid;
    gap: 0 1.25rem;
    align-items: start;
  }

  @media (min-width: 1100px) {
    .colonne {
      grid-template-columns: minmax(0, 2fr) minmax(0, 3fr);
    }
  }

  .colonna {
    min-width: 0;
  }

  .certificato {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .indietro {
    color: var(--colore-primario);
    text-decoration: none;
  }

  .intestazione {
    display: flex;
    flex-wrap: wrap;
    align-items: baseline;
    gap: 0.75rem;
    margin-bottom: 0.75rem;
  }

  .intestazione h1 {
    margin: 0;
  }

  .archiviato {
    color: var(--colore-testo-secondario);
  }

  /* Più raccolta delle schede normali: la scheda iscritto deve stare nella finestra */
  .scheda {
    display: grid;
    gap: 0.75rem;
    margin-bottom: 1rem;
    padding: 1.1rem 1.25rem;
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

  .azioni {
    display: flex;
    gap: 0.5rem;
  }

  .errore {
    color: var(--colore-errore);
  }
</style>
