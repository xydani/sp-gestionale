<script lang="ts">
  import { onMount, tick } from "svelte";
  import {
    elencaIscritti,
    elencaTipologie,
    messaggioErrore,
    proponiIscrizione,
    registraIscrittoGiaAttivo,
  } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { centesimiInTestoEuro, euroInCentesimi } from "$lib/format";
  import {
    completaDateAbbonamento,
    dateDaInviare,
    limitiInizioPregresso,
    limitiScadenzaPregressa,
    trovaOmonimi,
    type DataIndicata,
  } from "$lib/gia-iscritto";
  import { OPZIONI_METODO } from "$lib/metodi-pagamento";
  import type {
    DatiIscrittoGiaAttivo,
    Iscritto,
    MetodoPagamento,
    TipologiaAbbonamento,
  } from "$lib/types";
  import {
    LUNGHEZZA_MASSIMA_NOME,
    erroreImportoEuro,
    erroreNomeProprio,
    erroreTelefono,
    limitiScadenzaCertificato,
    validita,
  } from "$lib/validazione";
  import CampoData from "./CampoData.svelte";
  import Tendina from "./Tendina.svelte";

  /**
   * Inserimento rapido di chi era già iscritto prima di usare il programma: abbonamento e
   * quota si registrano con i loro importi, l'anagrafica si completa dopo dalla scheda.
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
  let inizioAbbonamento = $state("");
  let scadenzaAbbonamento = $state("");
  /** Quale delle due date è stata scritta a mano: l'altra si calcola dalla durata. */
  let dataIndicata = $state<DataIndicata | null>(null);
  let importoAbbonamento = $state("");
  let quotaPagata = $state(true);
  let quota = $state("");
  let metodo = $state<MetodoPagamento>("contanti");
  let certificatoPresentato = $state(false);
  let scadenzaCertificato = $state("");
  let omonimi = $state<Iscritto[]>([]);
  let errore = $state<string | null>(null);
  let inCorso = $state(false);
  let campoNome = $state<HTMLInputElement>();

  const certificato = limitiScadenzaCertificato();
  const tipologiaScelta = $derived(tipologie.find((tipologia) => tipologia.id === tipologiaId));
  const limitiScadenza = $derived(limitiScadenzaPregressa(tipologiaScelta));
  const limitiInizio = $derived(limitiInizioPregresso(tipologiaScelta));

  onMount(async () => {
    try {
      tipologie = await elencaTipologie(true);
      tipologiaId = tipologie[0]?.id ?? null;
      proponiImportoAbbonamento();
      quota = centesimiInTestoEuro((await proponiIscrizione(null)).importoCent);
    } catch (erroreBackend) {
      errore = messaggioErrore(erroreBackend);
    }
  });

  /** Il prezzo del listino per la tipologia scelta, modificabile. */
  function proponiImportoAbbonamento() {
    const scelta = tipologie.find((tipologia) => tipologia.id === tipologiaId);
    if (scelta) importoAbbonamento = centesimiInTestoEuro(scelta.prezzoCent);
  }

  function cambiaTipologia() {
    proponiImportoAbbonamento();
    completaAltraData();
  }

  function indicaData(quale: DataIndicata) {
    dataIndicata = quale;
    completaAltraData();
  }

  function completaAltraData() {
    const scelta = tipologie.find((tipologia) => tipologia.id === tipologiaId);
    const date = { inizioIso: inizioAbbonamento, scadenzaIso: scadenzaAbbonamento };
    const complete = completaDateAbbonamento(date, dataIndicata, scelta);
    inizioAbbonamento = complete.inizioIso;
    scadenzaAbbonamento = complete.scadenzaIso;
  }

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

  function datiInseriti(
    tipologiaId: number,
    importoAbbonamentoCent: number,
    quotaCent: number,
  ): DatiIscrittoGiaAttivo {
    return {
      nome,
      cognome,
      telefono: telefono || null,
      dataScadenzaCertificato: certificatoPresentato ? scadenzaCertificato || null : null,
      certificatoPresentato,
      tipologiaId,
      ...dateDaInviare(
        { inizioIso: inizioAbbonamento, scadenzaIso: scadenzaAbbonamento },
        dataIndicata,
      ),
      importoAbbonamentoCent,
      quotaPagata,
      quotaCent,
      metodo,
    };
  }

  /** Svuota i dati della persona; tipologia, importi e metodo restano: si inseriscono a gruppi. */
  async function preparaPersonaSuccessiva() {
    nome = "";
    cognome = "";
    telefono = "";
    inizioAbbonamento = "";
    scadenzaAbbonamento = "";
    dataIndicata = null;
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
    const importoAbbonamentoCent = euroInCentesimi(importoAbbonamento);
    // Senza quota pagata l'importo della quota non conta
    const quotaCent = quotaPagata ? euroInCentesimi(quota) : 0;
    if (importoAbbonamentoCent === null || quotaCent === null) {
      errore = "Importo non valido (esempio: 45 oppure 45,50)";
      return;
    }
    const aggiungiAltro = evento.submitter?.getAttribute("name") === AZIONE_AGGIUNGI_ALTRO;
    inCorso = true;
    try {
      const iscritto = await registraIscrittoGiaAttivo(
        datiInseriti(tipologiaId, importoAbbonamentoCent, quotaCent),
      );
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
    Per chi era già iscritto prima del programma. Dell'abbonamento basta una data, inizio o
    scadenza: l'altra si calcola dalla durata. Gli importi entrano negli incassi: la quota a inizio
    anno sportivo, l'abbonamento nel mese in cui è iniziato. Data e luogo di nascita si completano
    poi dalla scheda.
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
        onCambia={cambiaTipologia}
      />
    </label>
    <label>
      Importo abbonamento (€)
      <input
        bind:value={importoAbbonamento}
        inputmode="decimal"
        required
        use:validita={erroreImportoEuro(importoAbbonamento)}
      />
    </label>
  </div>

  <div class="griglia">
    <label>
      Iniziato il
      <CampoData
        bind:valore={inizioAbbonamento}
        {...limitiInizio}
        obbligatoria={dataIndicata !== "scadenza"}
        onCambia={() => indicaData("inizio")}
      />
    </label>
    <label>
      Scade il
      <CampoData
        bind:valore={scadenzaAbbonamento}
        {...limitiScadenza}
        obbligatoria={dataIndicata !== "inizio"}
        onCambia={() => indicaData("scadenza")}
      />
    </label>
  </div>

  <div class="griglia">
    <label class="spunta">
      <input type="checkbox" bind:checked={quotaPagata} />
      Quota dell'anno sportivo già pagata
    </label>
    {#if quotaPagata}
      <label>
        Quota (€)
        <input
          bind:value={quota}
          inputmode="decimal"
          required
          use:validita={erroreImportoEuro(quota)}
        />
      </label>
    {/if}
    <label>
      Pagati con
      <Tendina bind:valore={metodo} opzioni={OPZIONI_METODO} />
    </label>
  </div>

  <div class="griglia">
    <label class="spunta">
      <input type="checkbox" bind:checked={certificatoPresentato} />
      Certificato medico presentato
    </label>
    {#if certificatoPresentato}
      <label>
        Il certificato scade il (facoltativo)
        <CampoData bind:valore={scadenzaCertificato} {...certificato} />
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
