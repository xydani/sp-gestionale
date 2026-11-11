<script lang="ts">
  import { onMount, tick } from "svelte";
  import {
    elencaIscritti,
    elencaTipologie,
    messaggioErrore,
    registraIscrittoGiaAttivo,
  } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { limitiScadenzaPregressa, trovaOmonimi } from "$lib/gia-iscritto";
  import type { DatiIscrittoGiaAttivo, Iscritto, TipologiaAbbonamento } from "$lib/types";
  import {
    LUNGHEZZA_MASSIMA_NOME,
    erroreNomeProprio,
    erroreTelefono,
    limitiScadenzaCertificato,
    validita,
  } from "$lib/validazione";
  import CampoData from "./CampoData.svelte";
  import Tendina from "./Tendina.svelte";

  /**
   * Inserimento rapido di chi era già iscritto prima di usare il programma:
   * abbonamento e quota risultano pagati ma non sono incassi; l'anagrafica si completa dopo.
   */
  let {
    onSalvato,
    onAnnulla,
  }: {
    onSalvato: (iscritto: Iscritto) => void;
    onAnnulla: () => void;
  } = $props();

  /** Nome del pulsante che salva e lascia il modulo aperto per la persona successiva. */
  const AZIONE_AGGIUNGI_ALTRO = "aggiungi-altro";

  let tipologie = $state<TipologiaAbbonamento[]>([]);
  let nome = $state("");
  let cognome = $state("");
  let telefono = $state("");
  let tipologiaId = $state<number | null>(null);
  let scadenzaAbbonamento = $state("");
  let quotaPagata = $state(true);
  let certificatoPresentato = $state(false);
  let scadenzaCertificato = $state("");
  let omonimi = $state<Iscritto[]>([]);
  let errore = $state<string | null>(null);
  let inCorso = $state(false);
  let campoNome = $state<HTMLInputElement>();

  const certificato = limitiScadenzaCertificato();
  const tipologiaScelta = $derived(tipologie.find((tipologia) => tipologia.id === tipologiaId));
  const limitiScadenza = $derived(limitiScadenzaPregressa(tipologiaScelta));

  onMount(async () => {
    try {
      tipologie = await elencaTipologie(true);
      tipologiaId = tipologie[0]?.id ?? null;
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
    }
  });

  /** Avvisa, senza bloccare, se esiste già qualcuno con lo stesso nome e cognome. */
  async function cercaOmonimi() {
    if (!nome.trim() || !cognome.trim()) {
      omonimi = [];
      return;
    }
    try {
      const trovati = await elencaIscritti({
        testo: `${nome} ${cognome}`,
        includiArchiviati: true,
        statoAbbonamento: null,
        statoIscrizione: null,
      });
      omonimi = trovaOmonimi(trovati.elementi, nome, cognome);
    } catch {
      // Solo un aiuto: se la ricerca fallisce si può comunque salvare
      omonimi = [];
    }
  }

  function datiInseriti(tipologiaId: number): DatiIscrittoGiaAttivo {
    return {
      nome,
      cognome,
      telefono: telefono || null,
      dataScadenzaCertificato: certificatoPresentato ? scadenzaCertificato : null,
      tipologiaId,
      dataScadenzaAbbonamento: scadenzaAbbonamento,
      quotaPagata,
    };
  }

  /** Svuota i dati della persona; tipologia e quota restano, di solito si inseriscono a gruppi. */
  async function preparaPersonaSuccessiva() {
    nome = "";
    cognome = "";
    telefono = "";
    scadenzaAbbonamento = "";
    certificatoPresentato = false;
    scadenzaCertificato = "";
    omonimi = [];
    await tick();
    campoNome?.focus();
  }

  async function salva(evento: SubmitEvent) {
    evento.preventDefault();
    if (tipologiaId === null) {
      errore = "Scegli l'abbonamento";
      return;
    }
    const aggiungiAltro = evento.submitter?.getAttribute("name") === AZIONE_AGGIUNGI_ALTRO;
    inCorso = true;
    try {
      const iscritto = await registraIscrittoGiaAttivo(datiInseriti(tipologiaId));
      errore = null;
      if (aggiungiAltro) {
        mostraAvviso("successo", `${iscritto.nome} ${iscritto.cognome} inserito`);
        await preparaPersonaSuccessiva();
      } else {
        onSalvato(iscritto);
      }
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
    } finally {
      inCorso = false;
    }
  }
</script>

<form onsubmit={salva}>
  <p class="spiegazione">
    Per chi era già iscritto prima del programma: abbonamento e quota risultano pagati ma non
    entrano negli incassi. Data e luogo di nascita si completano poi dalla scheda.
  </p>

  <div class="griglia">
    <label>
      Nome
      <!-- svelte-ignore a11y_autofocus -->
      <input
        bind:this={campoNome}
        bind:value={nome}
        required
        autofocus
        maxlength={LUNGHEZZA_MASSIMA_NOME}
        use:validita={erroreNomeProprio("Il nome", nome)}
        onchange={cercaOmonimi}
      />
    </label>
    <label>
      Cognome
      <input
        bind:value={cognome}
        required
        maxlength={LUNGHEZZA_MASSIMA_NOME}
        use:validita={erroreNomeProprio("Il cognome", cognome)}
        onchange={cercaOmonimi}
      />
    </label>
    <label>
      Telefono (facoltativo)
      <input
        type="tel"
        bind:value={telefono}
        placeholder="+39 …"
        maxlength="25"
        use:validita={erroreTelefono(telefono)}
      />
    </label>
  </div>

  {#if omonimi.length > 0}
    <p class="avviso" role="status">
      Esiste già un iscritto «{omonimi[0].cognome}
      {omonimi[0].nome}»: controlla che non sia la stessa persona.
    </p>
  {/if}

  <div class="griglia">
    <label>
      Abbonamento in corso
      <Tendina
        bind:valore={tipologiaId}
        opzioni={tipologie.map((tipologia) => ({
          valore: tipologia.id,
          etichetta: tipologia.nome,
        }))}
        obbligatoria
      />
    </label>
    <label>
      Scade il
      <CampoData bind:valore={scadenzaAbbonamento} {...limitiScadenza} obbligatoria />
    </label>
    <label class="spunta">
      <input type="checkbox" bind:checked={quotaPagata} />
      Quota dell'anno sportivo già pagata
    </label>
  </div>

  <div class="griglia">
    <label class="spunta">
      <input type="checkbox" bind:checked={certificatoPresentato} />
      Certificato medico presentato
    </label>
    {#if certificatoPresentato}
      <label>
        Il certificato scade il
        <CampoData bind:valore={scadenzaCertificato} {...certificato} obbligatoria />
      </label>
    {/if}
  </div>

  {#if errore}<p class="errore" role="alert">{errore}</p>{/if}

  <div class="azioni">
    <button class="pulsante" type="submit" name={AZIONE_AGGIUNGI_ALTRO} disabled={inCorso}>
      Salva e aggiungi un altro
    </button>
    <button class="pulsante secondario" type="submit" disabled={inCorso}>Salva e chiudi</button>
    <button type="button" class="pulsante secondario" onclick={onAnnulla}>Annulla</button>
  </div>
</form>

<style>
  form {
    display: grid;
    gap: 1rem;
  }

  .spiegazione {
    margin: 0;
    color: var(--colore-testo-secondario);
    font-size: 0.9rem;
  }

  .griglia {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 1rem;
    align-items: end;
  }

  label {
    display: grid;
    gap: 0.35rem;
    font-weight: 500;
    font-size: 0.9rem;
  }

  input {
    padding: 0.55rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
    font-weight: 400;
  }

  label.spunta {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-height: 2.5rem;
  }

  label.spunta input {
    width: 1.1rem;
    height: 1.1rem;
    padding: 0;
  }

  .avviso {
    margin: 0;
    padding: 0.6rem 0.8rem;
    border-radius: var(--raggio);
    background: var(--colore-avviso-tenue);
    color: var(--colore-avviso);
    font-size: 0.9rem;
  }

  .errore {
    margin: 0;
    color: var(--colore-errore);
  }

  .azioni {
    display: flex;
    gap: 0.5rem;
  }
</style>
