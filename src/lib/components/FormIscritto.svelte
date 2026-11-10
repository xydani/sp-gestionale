<script lang="ts">
  import { untrack } from "svelte";
  import { oggiIso } from "$lib/format";
  import type { DatiIscritto } from "$lib/types";

  let {
    iniziale,
    etichettaConferma,
    onConferma,
    onAnnulla,
  }: {
    iniziale?: DatiIscritto;
    etichettaConferma: string;
    onConferma: (dati: DatiIscritto) => Promise<boolean>;
    onAnnulla: () => void;
  } = $props();

  // Il form parte dai valori iniziali e poi li modifica in autonomia
  const valoriIniziali = untrack(() => iniziale);

  let nome = $state(valoriIniziali?.nome ?? "");
  let cognome = $state(valoriIniziali?.cognome ?? "");
  let telefono = $state(valoriIniziali?.telefono ?? "");
  let dataIscrizione = $state(valoriIniziali?.dataIscrizione ?? oggiIso());
  let note = $state(valoriIniziali?.note ?? "");
  let inCorso = $state(false);

  async function conferma(evento: SubmitEvent) {
    evento.preventDefault();
    inCorso = true;
    try {
      await onConferma({
        nome,
        cognome,
        telefono: telefono || null,
        dataIscrizione,
        note: note || null,
      });
    } finally {
      inCorso = false;
    }
  }
</script>

<form onsubmit={conferma}>
  <div class="griglia">
    <label>
      Nome
      <!-- svelte-ignore a11y_autofocus -->
      <input bind:value={nome} required autofocus />
    </label>
    <label>
      Cognome
      <input bind:value={cognome} required />
    </label>
    <label>
      Telefono
      <input type="tel" bind:value={telefono} placeholder="+39 …" />
    </label>
    <label>
      Data di iscrizione
      <input type="date" bind:value={dataIscrizione} required />
    </label>
  </div>
  <label>
    Note
    <textarea bind:value={note} rows="3"></textarea>
  </label>
  <div class="azioni">
    <button type="submit" disabled={inCorso}>{etichettaConferma}</button>
    <button type="button" class="secondario" onclick={onAnnulla}>Annulla</button>
  </div>
</form>

<style>
  form {
    display: grid;
    gap: 1rem;
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
</style>
