<script lang="ts">
  import Tendina from "./Tendina.svelte";
  import { untrack } from "svelte";
  import { centesimiInTestoEuro, euroInCentesimi, nomeMeseDaNumero } from "$lib/format";
  import type { Impostazioni } from "$lib/types";

  /** Opzioni della palestra; usato sia nelle Impostazioni sia nella configurazione iniziale. */
  let {
    iniziale,
    etichettaConferma,
    onSalva,
  }: {
    iniziale: Impostazioni;
    etichettaConferma: string;
    onSalva: (impostazioni: Impostazioni) => Promise<void>;
  } = $props();

  const MESI = Array.from({ length: 12 }, (_, indice) => indice + 1);
  const valoriIniziali = untrack(() => iniziale);

  let nomePalestra = $state(valoriIniziali.nomePalestra);
  let quota = $state(centesimiInTestoEuro(valoriIniziali.quotaIscrizioneCent));
  let meseInizioAnnoSportivo = $state(valoriIniziali.meseInizioAnnoSportivo);
  let giorniPreavviso = $state(valoriIniziali.giorniPreavviso);
  let backupConservati = $state(valoriIniziali.backupConservati);
  let errore = $state<string | null>(null);
  let inCorso = $state(false);

  async function conferma(evento: SubmitEvent) {
    evento.preventDefault();
    const quotaIscrizioneCent = euroInCentesimi(quota);
    if (quotaIscrizioneCent === null) {
      errore = "Quota non valida (esempio: 30 oppure 30,50)";
      return;
    }
    errore = null;
    inCorso = true;
    try {
      await onSalva({
        nomePalestra,
        quotaIscrizioneCent,
        meseInizioAnnoSportivo,
        giorniPreavviso,
        backupConservati,
      });
    } finally {
      inCorso = false;
    }
  }
</script>

<form onsubmit={conferma}>
  <label class="largo">
    Nome della palestra
    <input bind:value={nomePalestra} maxlength="80" placeholder="Es. Palestra Centrale" />
  </label>

  <label>
    Quota di iscrizione annuale (€)
    <input bind:value={quota} inputmode="decimal" required />
  </label>
  <label>
    L'anno sportivo inizia a
    <Tendina
      bind:valore={meseInizioAnnoSportivo}
      opzioni={MESI.map((mese) => ({ valore: mese, etichetta: nomeMeseDaNumero(mese) }))}
    />
  </label>

  <label>
    Avvisa delle scadenze con giorni di anticipo
    <input type="number" min="1" max="60" bind:value={giorniPreavviso} required />
  </label>
  <label>
    Backup automatici da conservare
    <input type="number" min="1" max="365" bind:value={backupConservati} required />
  </label>

  {#if errore}<p class="errore largo" role="alert">{errore}</p>{/if}

  <div class="largo">
    <button class="pulsante" type="submit" disabled={inCorso}
      >{inCorso ? "Salvataggio…" : etichettaConferma}</button
    >
  </div>
</form>

<style>
  form {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 1rem;
  }

  .largo {
    grid-column: 1 / -1;
  }

  label {
    display: grid;
    gap: 0.35rem;
    align-content: end;
  }

  input {
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    font: inherit;
    background: var(--colore-superficie);
  }

  .errore {
    margin: 0;
    color: var(--colore-errore);
  }
</style>
