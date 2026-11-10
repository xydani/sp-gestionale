<script lang="ts">
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { VOCI_MENU, eVoceAttiva } from "$lib/menu";
  import { alternaSchermoIntero } from "$lib/schermo-intero";

  let { onBlocca }: { onBlocca: () => void } = $props();
</script>

<nav>
  <div class="titolo">SP Gestionale</div>
  <ul>
    {#each VOCI_MENU as voce (voce.percorso)}
      <li>
        <a href={resolve(voce.percorso)} class:attiva={eVoceAttiva(voce, page.url.pathname)}>
          {voce.etichetta}
        </a>
      </li>
    {/each}
  </ul>
  <div class="azioni">
    <button type="button" onclick={alternaSchermoIntero} title="Schermo intero (F11)">
      Schermo intero
    </button>
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
