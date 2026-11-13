<script lang="ts">
  import CampoData from "./CampoData.svelte";
  import { untrack, type Snippet } from "svelte";
  import {
    LUNGHEZZA_MASSIMA_LUOGO,
    LUNGHEZZA_MASSIMA_NOME,
    LUNGHEZZA_MASSIMA_NOTE,
    validita,
    erroreEmail,
    erroreNomeProprio,
    erroreTelefono,
    limitiDataNascita,
    limitiScadenzaCertificato,
  } from "$lib/validazione";
  import type { DatiIscritto } from "$lib/types";

  let {
    iniziale,
    etichettaConferma,
    onConferma,
    onAnnulla,
    sezioniAggiuntive,
  }: {
    iniziale?: Partial<DatiIscritto>;
    etichettaConferma: string;
    onConferma: (dati: DatiIscritto) => Promise<boolean>;
    onAnnulla: () => void;
    /** Altre parti del modulo prima dei pulsanti (es. i pagamenti del nuovo iscritto). */
    sezioniAggiuntive?: Snippet;
  } = $props();

  // Il form parte dai valori iniziali e poi li modifica in autonomia
  const valoriIniziali = untrack(() => iniziale);

  let nome = $state(valoriIniziali?.nome ?? "");
  let cognome = $state(valoriIniziali?.cognome ?? "");
  let dataNascita = $state(valoriIniziali?.dataNascita ?? "");
  let luogoNascita = $state(valoriIniziali?.luogoNascita ?? "");
  let telefono = $state(valoriIniziali?.telefono ?? "");
  let email = $state(valoriIniziali?.email ?? "");
  let note = $state(valoriIniziali?.note ?? "");
  let certificatoPresentato = $state(
    valoriIniziali?.certificatoPresentato ?? Boolean(valoriIniziali?.dataScadenzaCertificato),
  );
  let scadenzaCertificato = $state(valoriIniziali?.dataScadenzaCertificato ?? "");
  let inCorso = $state(false);

  const nascita = limitiDataNascita();
  const certificato = limitiScadenzaCertificato();

  async function conferma(evento: SubmitEvent) {
    evento.preventDefault();
    inCorso = true;
    try {
      await onConferma({
        nome,
        cognome,
        dataNascita: dataNascita || null,
        luogoNascita: luogoNascita || null,
        telefono: telefono || null,
        email: email || null,
        note: note || null,
        dataScadenzaCertificato: certificatoPresentato ? scadenzaCertificato || null : null,
        certificatoPresentato,
      });
    } finally {
      inCorso = false;
    }
  }
</script>

<form onsubmit={conferma}>
  <fieldset>
    <legend>Anagrafica (obbligatori solo nome e cognome)</legend>
    <div class="griglia">
      <label>
        Nome
        <!-- svelte-ignore a11y_autofocus -->
        <input
          bind:value={nome}
          required
          autofocus
          maxlength={LUNGHEZZA_MASSIMA_NOME}
          use:validita={erroreNomeProprio("Il nome", nome)}
        />
      </label>
      <label>
        Cognome
        <input
          bind:value={cognome}
          required
          maxlength={LUNGHEZZA_MASSIMA_NOME}
          use:validita={erroreNomeProprio("Il cognome", cognome)}
        />
      </label>
      <label>
        Data di nascita
        <CampoData bind:valore={dataNascita} {...nascita} />
      </label>
      <label>
        Luogo di nascita
        <input
          bind:value={luogoNascita}
          maxlength={LUNGHEZZA_MASSIMA_LUOGO}
          use:validita={erroreNomeProprio("Il luogo di nascita", luogoNascita)}
        />
      </label>
    </div>
  </fieldset>

  <fieldset>
    <legend>Contatti (facoltativi)</legend>
    <div class="griglia">
      <label>
        Telefono
        <input
          type="tel"
          bind:value={telefono}
          placeholder="+39 …"
          maxlength="25"
          use:validita={erroreTelefono(telefono)}
        />
      </label>
      <label>
        Email
        <input
          type="email"
          bind:value={email}
          placeholder="nome@esempio.it"
          maxlength="254"
          use:validita={erroreEmail(email)}
        />
      </label>
    </div>
  </fieldset>

  <fieldset>
    <legend>Certificato medico</legend>
    <div class="griglia certificato">
      <label class="spunta">
        <input type="checkbox" bind:checked={certificatoPresentato} />
        Certificato presentato
      </label>
      {#if certificatoPresentato}
        <label>
          Scade il (facoltativo)
          <CampoData bind:valore={scadenzaCertificato} {...certificato} />
        </label>
      {:else}
        <p class="avviso">Da portare: finché manca, l'iscritto compare tra le scadenze.</p>
      {/if}
    </div>
  </fieldset>

  <label>
    Note
    <textarea bind:value={note} rows="3" maxlength={LUNGHEZZA_MASSIMA_NOTE}></textarea>
  </label>
  {@render sezioniAggiuntive?.()}
  <div class="azioni">
    <button class="pulsante" type="submit" disabled={inCorso}>{etichettaConferma}</button>
    <button type="button" class="pulsante secondario" onclick={onAnnulla}>Annulla</button>
  </div>
</form>

<style>
  form {
    display: grid;
    gap: 1rem;
  }

  fieldset {
    margin: 0;
    padding: 0;
    border: none;
    display: grid;
    gap: 0.6rem;
  }

  legend {
    padding: 0;
    margin-bottom: 0.5rem;
    color: var(--colore-testo-secondario);
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .griglia {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 1rem;
  }

  label {
    display: grid;
    gap: 0.35rem;
    font-weight: 500;
    font-size: 0.9rem;
  }

  input,
  textarea {
    padding: 0.55rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
    font-weight: 400;
  }

  .certificato {
    align-items: end;
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
    align-self: center;
    color: var(--colore-avviso);
    font-size: 0.9rem;
  }

  .azioni {
    display: flex;
    gap: 0.5rem;
  }
</style>
