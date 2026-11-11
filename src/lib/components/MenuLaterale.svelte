<script lang="ts">
  import { getVersion } from "@tauri-apps/api/app";
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { SCORCIATOIA_BLOCCA, VOCI_MENU, eTastoBlocca, eVoceAttiva } from "$lib/menu";
  import { onMount } from "svelte";
  import { palestra, caricaNomePalestra } from "$lib/palestra.svelte";
  import { scadenzeInArrivo, totaleInScadenza } from "$lib/promemoria.svelte";
  import LogoPalestra from "./LogoPalestra.svelte";

  let { onBlocca }: { onBlocca: () => void } = $props();

  let versione = $state("");

  onMount(() => {
    // Il nome è solo decorativo: se non si legge il menu funziona lo stesso
    caricaNomePalestra().catch(() => {});
    getVersion()
      .then((letta) => (versione = letta))
      .catch(() => {});
  });

  function gestisciTasti(evento: KeyboardEvent) {
    if (!eTastoBlocca(evento)) return;
    evento.preventDefault();
    onBlocca();
  }

  const totaleScadenze = $derived(totaleInScadenza(scadenzeInArrivo));
</script>

<svelte:window onkeydown={gestisciTasti} />

<nav>
  <div class="intestazione">
    <div class="logo"><LogoPalestra altezza="44px" /></div>
    <div class="titolo">{palestra.nome || "SP Gestionale"}</div>
  </div>
  <ul>
    {#each VOCI_MENU as voce (voce.percorso)}
      <li>
        <a href={resolve(voce.percorso)} class:attiva={eVoceAttiva(voce, page.url.pathname)}>
          {voce.etichetta}
          {#if voce.percorso === "/scadenze" && totaleScadenze > 0}
            <span class="badge" title="Scadenze nei prossimi giorni">{totaleScadenze}</span>
          {/if}
        </a>
      </li>
    {/each}
  </ul>
  <div class="azioni">
    <button
      type="button"
      class="blocca"
      onclick={onBlocca}
      title="Blocca il programma: serve la password per riaprirlo"
    >
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <rect x="5" y="11" width="14" height="10" rx="2" />
        <path d="M8 11V7a4 4 0 0 1 8 0v4" />
      </svg>
      <span>Blocca</span>
      <kbd>{SCORCIATOIA_BLOCCA}</kbd>
    </button>
    {#if versione}<span class="versione">Versione {versione}</span>{/if}
  </div>
</nav>

<style>
  nav {
    position: sticky;
    top: 0;
    height: 100vh;
    width: var(--larghezza-menu);
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    background: var(--colore-menu);
    color: var(--colore-menu-testo);
    padding: 1rem 0.75rem;
  }

  .intestazione {
    display: grid;
    gap: 0.6rem;
    padding: 0.5rem 0.75rem 1.25rem;
  }

  /* Il logo è blu: sul fondo scuro sta in un riquadro bianco */
  .logo:has(:global(img)) {
    justify-self: start;
    padding: 0.4rem 0.6rem;
    border-radius: var(--raggio);
    background: #ffffff;
  }

  .titolo {
    font-weight: 650;
    font-size: 1.05rem;
    color: #ffffff;
    overflow-wrap: anywhere;
  }

  ul {
    display: grid;
    gap: 2px;
    align-content: start;
    list-style: none;
    margin: 0;
    padding: 0;
    flex: 1;
  }

  a {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.55rem 0.75rem;
    border-radius: var(--raggio);
    color: inherit;
    text-decoration: none;
    font-weight: 500;
    transition:
      background-color 0.15s,
      color 0.15s;
  }

  a:hover {
    background: var(--colore-menu-hover);
    color: #ffffff;
  }

  a.attiva {
    background: var(--colore-menu-attivo);
    color: #ffffff;
    font-weight: 600;
    box-shadow: 0 1px 3px rgb(0 0 0 / 0.25);
  }

  a:focus-visible {
    outline-color: #ffffff;
  }

  .badge {
    min-width: 1.4rem;
    padding: 0 0.4rem;
    border-radius: 999px;
    background: #fbbf24;
    color: #3b2a00;
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1.35rem;
    text-align: center;
  }

  .azioni {
    display: grid;
    gap: 0.35rem;
    padding-top: 0.75rem;
    border-top: 1px solid var(--colore-menu-bordo);
  }

  .blocca {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    padding: 0.55rem 0.75rem;
    border: none;
    border-radius: var(--raggio);
    background: transparent;
    color: inherit;
    font: inherit;
    font-weight: 500;
    text-align: left;
    cursor: pointer;
    transition:
      background-color 0.15s,
      color 0.15s;
  }

  .blocca:hover {
    background: var(--colore-menu-hover);
    color: #ffffff;
  }

  .blocca:focus-visible {
    outline-color: #ffffff;
  }

  .blocca svg {
    width: 1.1rem;
    height: 1.1rem;
    flex-shrink: 0;
    fill: none;
    stroke: currentColor;
    stroke-width: 2;
    stroke-linecap: round;
    stroke-linejoin: round;
  }

  .blocca span {
    flex: 1;
  }

  kbd {
    font: inherit;
    font-size: 0.75rem;
    opacity: 0.6;
  }

  .versione {
    padding-left: 0.75rem;
    font-size: 0.75rem;
    opacity: 0.6;
  }
</style>
