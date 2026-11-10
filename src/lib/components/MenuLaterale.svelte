<script lang="ts">
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { VOCI_MENU, eVoceAttiva } from "$lib/menu";
  import { scadenzeInArrivo } from "$lib/promemoria.svelte";

  let { onBlocca }: { onBlocca: () => void } = $props();

  const totaleInScadenza = $derived(scadenzeInArrivo.abbonamenti + scadenzeInArrivo.iscrizioni);
</script>

<nav>
  <div class="titolo">SP Gestionale</div>
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
    <button type="button" onclick={onBlocca}>Blocca</button>
  </div>
</nav>

<style>
  nav {
    width: var(--larghezza-menu);
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    background: var(--colore-menu);
    color: var(--colore-menu-testo);
    padding: 1rem 0.75rem;
  }

  .titolo {
    font-weight: 700;
    font-size: 1.1rem;
    padding: 0.5rem 0.75rem 1.25rem;
  }

  ul {
    list-style: none;
    margin: 0;
    padding: 0;
    flex: 1;
  }

  a {
    display: block;
    padding: 0.6rem 0.75rem;
    border-radius: var(--raggio);
    color: inherit;
    text-decoration: none;
  }

  .badge {
    float: right;
    min-width: 1.4rem;
    padding: 0 0.4rem;
    border-radius: 999px;
    background: #e0a100;
    color: #1d2330;
    font-size: 0.8rem;
    font-weight: 600;
    line-height: 1.4rem;
    text-align: center;
  }

  a:hover,
  a.attiva {
    background: var(--colore-menu-attivo);
    color: #fff;
  }

  .azioni {
    display: grid;
    gap: 0.5rem;
  }

  .azioni button {
    background: transparent;
    color: inherit;
    border: 1px solid var(--colore-menu-attivo);
    border-radius: var(--raggio);
    padding: 0.5rem;
  }
</style>
