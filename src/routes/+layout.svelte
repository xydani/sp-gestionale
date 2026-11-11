<script lang="ts">
  import "../app.css";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { onDestroy, onMount, type Snippet } from "svelte";
  import { page } from "$app/state";
  import { motivoBloccoInattivita } from "$lib/sblocco";
  import { completaEliminazioniInSospeso } from "$lib/eliminazione-differita";
  import { applicaDimensioneTesto, leggiDimensioneTesto } from "$lib/dimensione-testo";
  import { bloccaDatabase, leggiImpostazioni, leggiStatoDatabase } from "$lib/api";
  import {
    EVENTI_ATTIVITA,
    avviaTimerInattivita,
    bloccoAutomatico,
  } from "$lib/blocco-automatico.svelte";
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
  /** Nella finestra di accesso: perché il programma si è bloccato, se non è stato a mano. */
  let motivoBlocco = $state<string | null>(null);

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
        await leggiMinutiBloccoAutomatico();
        await proponiConfigurazioneIniziale();
      } else {
        fermaControlloPromemoria();
      }
    }
    if (finestraDiAccesso && statoDatabase !== "sbloccato") {
      await adattaFinestraDiAccesso(statoDatabase === "da_creare");
    }
  }

  async function leggiMinutiBloccoAutomatico() {
    try {
      bloccoAutomatico.minuti = (await leggiImpostazioni()).minutiBloccoAutomatico;
    } catch {
      // Senza impostazioni leggibili resta il valore precedente: non si blocca il lavoro
    }
  }

  // Ad archivio aperto, dopo i minuti scelti senza attività si torna alla password
  $effect(() => {
    if (finestraDiAccesso || statoDatabase !== "sbloccato") return;
    const minuti = bloccoAutomatico.minuti;
    const timer = avviaTimerInattivita(minuti, () => blocca(motivoBloccoInattivita(minuti)));
    const registra = () => timer.registraAttivita();
    for (const evento of EVENTI_ATTIVITA)
      window.addEventListener(evento, registra, { passive: true });
    return () => {
      timer.ferma();
      for (const evento of EVENTI_ATTIVITA) window.removeEventListener(evento, registra);
    };
  });

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
    if (finestraDiAccesso) motivoBlocco = cambio.motivoBlocco ?? null;
    await aggiornaStato();
  }

  async function blocca(motivo: string | null = null) {
    // Dopo il blocco l'archivio è chiuso: ciò che è stato eliminato va cancellato adesso
    await completaEliminazioniInSospeso();
    await bloccaDatabase();
    await passaAllaFinestraDiAccesso(motivo);
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
    // La finestra di accesso ha dimensioni fisse: il testo si ingrandisce solo nella principale
    if (!finestraDiAccesso) applicaDimensioneTesto(leggiDimensioneTesto());
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
      <SchermataSblocco
        {motivoBlocco}
        onSbloccato={(avviso) => passaAllaFinestraPrincipale(avviso)}
      />
    </CorniceSenzaBordi>
  {:else if statoDatabase === "da_creare"}
    <CorniceSenzaBordi
      ><PrimoAvvio onCompletato={() => passaAllaFinestraPrincipale()} /></CorniceSenzaBordi
    >
  {/if}
{:else if statoDatabase === "sbloccato"}
  <div class="app">
    {#if configurazione.completata}<MenuLaterale onBlocca={() => blocca()} />{/if}
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
