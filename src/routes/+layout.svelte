<script lang="ts">
  import "../app.css";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { onDestroy, onMount, type Snippet } from "svelte";
  import { page } from "$app/state";
  import { bloccaDatabase, leggiStatoDatabase } from "$lib/api";
  import {
    configurazione,
    percorsoObbligato,
    verificaConfigurazione,
  } from "$lib/configurazione-guidata.svelte";
  import {
    adattaFinestraDiAccesso,
    eFinestraDiAccesso,
    passaAllaFinestraDiAccesso,
    passaAllaFinestraPrincipale,
    quandoCambiaStatoDatabase,
    type CambioStatoDatabase,
  } from "$lib/finestra";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { INTERVALLO_CONTROLLO_MS, eseguiControlloPromemoria } from "$lib/promemoria.svelte";
  import {
    INTERVALLO_CONTROLLO_AGGIORNAMENTI_MS,
    verificaAggiornamenti,
  } from "$lib/aggiornamento.svelte";
  import BannerAggiornamento from "$lib/components/BannerAggiornamento.svelte";
  import BarraAvvisi from "$lib/components/BarraAvvisi.svelte";
  import CorniceSenzaBordi from "$lib/components/CorniceSenzaBordi.svelte";
  import MenuLaterale from "$lib/components/MenuLaterale.svelte";
  import PrimoAvvio from "$lib/components/PrimoAvvio.svelte";
  import PulsanteSchermoIntero from "$lib/components/PulsanteSchermoIntero.svelte";
  import SchermataSblocco from "$lib/components/SchermataSblocco.svelte";
  import {
    alternaSchermoIntero,
    eTastoSchermoIntero,
    esciDaSchermoIntero,
  } from "$lib/schermo-intero";
  import type { StatoDatabase } from "$lib/types";

  let { children }: { children: Snippet } = $props();

  let statoDatabase = $state<StatoDatabase | null>(null);
  let pulsanteSchermoIntero = $state<PulsanteSchermoIntero>();

  // Ogni finestra mostra solo le sue schermate: accesso (password) oppure programma
  const finestraDiAccesso = eFinestraDiAccesso();
  let smettiDiAscoltare: (() => void) | undefined;
  let timerPromemoria: ReturnType<typeof setInterval> | undefined;
  let timerAggiornamenti: ReturnType<typeof setInterval> | undefined;

  /** Nella finestra principale cerca nuove versioni all'avvio e poi una volta al giorno. */
  function avviaControlloAggiornamenti() {
    if (timerAggiornamenti) return;
    verificaAggiornamenti(true);
    timerAggiornamenti = setInterval(
      () => verificaAggiornamenti(true),
      INTERVALLO_CONTROLLO_AGGIORNAMENTI_MS,
    );
  }

  /** Nella finestra principale, da sbloccata, controlla le scadenze subito e poi ogni ora. */
  function avviaControlloPromemoria() {
    fermaControlloPromemoria();
    controllaPromemoriaSenzaInterrompere();
    timerPromemoria = setInterval(controllaPromemoriaSenzaInterrompere, INTERVALLO_CONTROLLO_MS);
  }

  function fermaControlloPromemoria() {
    clearInterval(timerPromemoria);
    timerPromemoria = undefined;
  }

  async function controllaPromemoriaSenzaInterrompere() {
    try {
      await eseguiControlloPromemoria();
    } catch {
      // Un promemoria non riuscito (es. archivio appena bloccato) non deve disturbare il lavoro
    }
  }

  async function aggiornaStato() {
    statoDatabase = await leggiStatoDatabase();
    if (!finestraDiAccesso) {
      if (statoDatabase === "sbloccato") {
        avviaControlloPromemoria();
        avviaControlloAggiornamenti();
        await proponiConfigurazioneIniziale();
      } else {
        fermaControlloPromemoria();
      }
    }
    if (finestraDiAccesso && statoDatabase !== "sbloccato") {
      await adattaFinestraDiAccesso(statoDatabase === "da_creare");
    }
  }

  /** A ogni sblocco si controlla se la configurazione guidata è stata completata. */
  async function proponiConfigurazioneIniziale() {
    configurazione.completata = null;
    await verificaConfigurazione();
  }

  // Finché la configurazione non è completa si resta nella guida, qualunque pagina si apra
  $effect(() => {
    if (finestraDiAccesso || statoDatabase !== "sbloccato" || configurazione.completata === null) {
      return;
    }
    const destinazione = percorsoObbligato(configurazione.completata, page.url.pathname);
    if (destinazione) goto(resolve(destinazione));
  });

  async function gestisciCambioStato(cambio: CambioStatoDatabase) {
    if (cambio.avviso && !finestraDiAccesso) mostraAvviso("avviso", cambio.avviso);
    await aggiornaStato();
  }

  async function blocca() {
    await bloccaDatabase();
    await passaAllaFinestraDiAccesso();
  }

  async function gestisciTasti(evento: KeyboardEvent) {
    if (finestraDiAccesso) return;
    if (eTastoSchermoIntero(evento)) {
      evento.preventDefault();
      await alternaSchermoIntero();
    } else if (evento.key === "Escape") {
      await esciDaSchermoIntero();
    } else {
      return;
    }
    await pulsanteSchermoIntero?.aggiornaStato();
  }

  onMount(async () => {
    document.documentElement.classList.toggle("finestra-compatta", finestraDiAccesso);
    smettiDiAscoltare = await quandoCambiaStatoDatabase(gestisciCambioStato);
    await aggiornaStato();
  });

  onDestroy(() => {
    smettiDiAscoltare?.();
    fermaControlloPromemoria();
    clearInterval(timerAggiornamenti);
  });
</script>

<svelte:window onkeydown={gestisciTasti} />

{#if finestraDiAccesso}
  {#if statoDatabase === "bloccato"}
    <CorniceSenzaBordi>
      <SchermataSblocco onSbloccato={(avviso) => passaAllaFinestraPrincipale(avviso)} />
    </CorniceSenzaBordi>
  {:else if statoDatabase === "da_creare"}
    <CorniceSenzaBordi
      ><PrimoAvvio onCompletato={() => passaAllaFinestraPrincipale()} /></CorniceSenzaBordi
    >
  {/if}
{:else if statoDatabase === "sbloccato"}
  <div class="app">
    {#if configurazione.completata}<MenuLaterale onBlocca={blocca} />{/if}
    <main>
      <div class="barra-superiore">
        <PulsanteSchermoIntero bind:this={pulsanteSchermoIntero} />
      </div>
      <div class="pagina">
        <BannerAggiornamento />
        <BarraAvvisi />
        {#if configurazione.completata !== null}{@render children()}{/if}
      </div>
    </main>
  </div>
{/if}

<style>
  .app {
    display: flex;
    min-height: 100vh;
  }

  main {
    flex: 1;
    min-width: 0;
    padding: 1rem 2rem 1.5rem;
  }

  /* Il contenuto resta centrato e leggibile anche su schermi molto larghi */
  .pagina {
    max-width: 1280px;
    margin: 0 auto;
  }

  .barra-superiore {
    display: flex;
    justify-content: flex-end;
    margin-bottom: 0.5rem;
  }
</style>
