<script lang="ts">
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { VOCI_MENU, eVoceAttiva } from "$lib/menu";
  import { onMount } from "svelte";
  import { palestra, caricaNomePalestra } from "$lib/palestra.svelte";
  import { scadenzeInArrivo } from "$lib/promemoria.svelte";
  import LogoPalestra from "./LogoPalestra.svelte";

  let { onBlocca }: { onBlocca: () => void } = $props();

  onMount(() => {
    // Il nome è solo decorativo: se non si legge il menu funziona lo stesso
    caricaNomePalestra().catch(() => {});
  });

  const totaleInScadenza = $derived(scadenzeInArrivo.abbonamenti + scadenzeInArrivo.iscrizioni);
</script>

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
          {#if voce.percorso === "/scadenze" && totaleInScadenza > 0}
            <span class="badge" title="Scadenze nei prossimi giorni">{totaleInScadenza}</span>
          {/if}
        </a>
      </li>
    {/each}
  </ul>
  <div class="azioni">
    <button type="button" class="pulsante secondario" onclick={onBlocca}>Blocca</button>
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
    border-right: 1px solid var(--colore-bordo);
    color: var(--colore-menu-testo);
    padding: 1rem 0.75rem;
  }

  .intestazione {
    display: grid;
    gap: 0.6rem;
    padding: 0.5rem 0.75rem 1.25rem;
  }

  .titolo {
    font-weight: 650;
    font-size: 1.05rem;
    color: var(--colore-testo);
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
    background: var(--colore-sfondo-tenue);
    color: var(--colore-testo);
  }

  a.attiva {
    background: var(--colore-menu-attivo);
    color: var(--colore-primario);
    font-weight: 600;
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
  }
</style>
