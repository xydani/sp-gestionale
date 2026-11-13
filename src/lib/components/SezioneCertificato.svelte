<script lang="ts">
  import { messaggioErrore, modificaIscritto } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { datiConCertificato } from "$lib/certificato";
  import { formattaData } from "$lib/format";
  import { aggiornaConteggioScadenze } from "$lib/promemoria.svelte";
  import type { Iscritto } from "$lib/types";
  import BadgeStato from "./BadgeStato.svelte";
  import CampiCertificato from "./CampiCertificato.svelte";
  import FinestraSovrapposta from "./FinestraSovrapposta.svelte";

  let {
    iscritto,
    onAggiornato,
    formAperto = $bindable(false),
  }: {
    iscritto: Iscritto;
    onAggiornato: (iscritto: Iscritto) => void;
    /** Aperto anche dalla striscia «Da sistemare» e dagli «Aggiorna» delle scadenze. */
    formAperto?: boolean;
  } = $props();

  let presentato = $state(false);
  let scadenza = $state("");
  let inCorso = $state(false);

  // Il modulo riparte ogni volta dai dati salvati
  $effect(() => {
    if (!formAperto) return;
    presentato = iscritto.certificatoPresentato;
    scadenza = iscritto.dataScadenzaCertificato ?? "";
  });

  async function salva(evento: SubmitEvent) {
    evento.preventDefault();
    inCorso = true;
    try {
      const aggiornato = await modificaIscritto(
        iscritto.id,
        datiConCertificato(iscritto, {
          certificatoPresentato: presentato,
          dataScadenzaCertificato: scadenza,
        }),
      );
      formAperto = false;
      mostraAvviso("successo", "Certificato medico aggiornato");
      onAggiornato(aggiornato);
      aggiornaConteggioScadenze();
    } catch (erroreBackend) {
      mostraAvviso("errore", messaggioErrore(erroreBackend));
    } finally {
      inCorso = false;
    }
  }
</script>

<section class="scheda" id="sezione-certificato">
  <div class="intestazione-sezione">
    <h2>Certificato medico</h2>
    <BadgeStato stato={iscritto.statoCertificato} />
    <span class="spazio"></span>
    {#if !iscritto.archiviato}
      <button class="pulsante" type="button" onclick={() => (formAperto = true)}>
        {iscritto.certificatoPresentato ? "Aggiorna" : "Registra"}
      </button>
    {/if}
  </div>

  <p class="riepilogo">
    {#if iscritto.dataScadenzaCertificato}
      {iscritto.statoCertificato === "scaduto" ? "Scaduto il" : "Valido fino al"}
      <strong>{formattaData(iscritto.dataScadenzaCertificato)}</strong>
    {:else if iscritto.certificatoPresentato}
      Presentato, scadenza non indicata
    {:else}
      Non presentato
    {/if}
  </p>

  {#if formAperto}
    <FinestraSovrapposta
      titolo="Certificato medico"
      larghezza="stretta"
      sottotitolo="{iscritto.cognome} {iscritto.nome}"
      onChiusa={() => (formAperto = false)}
    >
      <form onsubmit={salva}>
        <CampiCertificato bind:presentato bind:scadenza />
        <div class="azioni">
          <button class="pulsante" type="submit" disabled={inCorso}>Salva certificato</button>
          <button type="button" class="pulsante secondario" onclick={() => (formAperto = false)}>
            Annulla
          </button>
        </div>
      </form>
    </FinestraSovrapposta>
  {/if}
</section>

<style>
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
    white-space: nowrap;
  }

  .riepilogo {
    margin: 0;
    color: var(--colore-testo-secondario);
  }

  form {
    display: grid;
    gap: 0.75rem;
  }

  .azioni {
    display: flex;
    gap: 0.5rem;
  }
</style>
