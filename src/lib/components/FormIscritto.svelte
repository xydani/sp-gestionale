<script lang="ts">
  import CampiCertificato from "./CampiCertificato.svelte";
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
  } from "$lib/validazione";
  import type { DatiIscritto } from "$lib/types";

  let {
    iniziale,
    etichettaConferma,
    onConferma,
    onAnnulla,
    sezioniAggiuntive,
    conCertificato = true,
  }: {
    iniziale?: Partial<DatiIscritto>;
    etichettaConferma: string;
    onConferma: (dati: DatiIscritto) => Promise<boolean>;
    onAnnulla: () => void;
    /** Altre parti del modulo prima dei pulsanti (es. i pagamenti del nuovo iscritto). */
    sezioniAggiuntive?: Snippet;
    /** Nella scheda iscritto il certificato ha una sezione sua e qui non compare. */
    conCertificato?: boolean;
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
  <!-- Con le sezioni aggiuntive (nuovo iscritto) i dati della persona stanno a sinistra -->
  <div class="corpo" class:affiancato={Boolean(sezioniAggiuntive)}>
    <div class="colonna">
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
        <legend>{conCertificato ? "Contatti e certificato medico" : "Contatti"}</legend>
        <div class="griglia">
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
          <label>
            Email (facoltativa)
            <input
              type="email"
              bind:value={email}
              placeholder="nome@esempio.it"
              maxlength="254"
              use:validita={erroreEmail(email)}
            />
          </label>
        </div>
        {#if conCertificato}
          <CampiCertificato
            bind:presentato={certificatoPresentato}
            bind:scadenza={scadenzaCertificato}
          />
        {/if}
      </fieldset>

      <label>
        Note
        <textarea
          bind:value={note}
          rows={sezioniAggiuntive ? 2 : 3}
          maxlength={LUNGHEZZA_MASSIMA_NOTE}></textarea>
      </label>
    </div>
    {#if sezioniAggiuntive}
      <div class="colonna">{@render sezioniAggiuntive()}</div>
    {/if}
  </div>
  <div class="azioni">
    <button class="pulsante" type="submit" disabled={inCorso}>{etichettaConferma}</button>
    <button type="button" class="pulsante secondario" onclick={onAnnulla}>Annulla</button>
  </div>
</form>

<style>
  form,
  .colonna {
    display: grid;
    gap: 1rem;
    align-content: start;
  }

  .corpo {
    display: grid;
    gap: 1rem 2rem;
  }

  @media (min-width: 1100px) {
    .corpo.affiancato {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    /* In mezza larghezza tre campi per riga, così il modulo sta nella finestra */
    .corpo.affiancato .griglia {
      grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    }
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
    min-width: 0;
    gap: 0.35rem;
    font-weight: 500;
    font-size: 0.9rem;
  }

  input,
  textarea {
    min-width: 0;
    padding: 0.55rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
    font-weight: 400;
  }

  .azioni {
    display: flex;
    gap: 0.5rem;
  }
</style>
