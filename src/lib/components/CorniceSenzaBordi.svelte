<script lang="ts">
  import type { Snippet } from "svelte";
  import { chiudiApp } from "$lib/finestra";

  let { children }: { children: Snippet } = $props();
</script>

<!-- Finestra trasparente senza barra del titolo: la scheda arrotondata è tutta la finestra -->
<div class="cornice">
  <div class="barra-trascinamento" data-tauri-drag-region>
    <button type="button" class="chiudi" onclick={chiudiApp} aria-label="Chiudi" title="Chiudi">
      <svg viewBox="0 0 24 24" width="16" height="16" aria-hidden="true">
        <path d="M6 6l12 12M18 6L6 18" />
      </svg>
    </button>
  </div>

  <div class="contenuto">
    <!-- Il logo non è nel repository: se manca, l'immagine si nasconde -->
    <img
      class="logo"
      src="/logo-palestra.png"
      alt="Logo della palestra"
      onerror={(evento) => ((evento.currentTarget as HTMLImageElement).hidden = true)}
    />
    {@render children()}
  </div>
</div>

<style>
  :global(html.finestra-compatta),
  :global(html.finestra-compatta body) {
    background: transparent;
  }

  .cornice {
    height: 100vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    border-radius: 18px;
    border: 1px solid var(--colore-bordo);
    background: linear-gradient(180deg, #eef1fb 0%, var(--colore-superficie) 38%);
  }

  .barra-trascinamento {
    height: 40px;
    flex-shrink: 0;
    display: flex;
    justify-content: flex-end;
    align-items: center;
    padding: 0 0.6rem;
    user-select: none;
  }

  .chiudi {
    display: grid;
    place-items: center;
    width: 28px;
    height: 28px;
    border: none;
    border-radius: 50%;
    background: transparent;
    color: var(--colore-testo-secondario);
  }

  .chiudi:hover {
    background: #e5484d;
    color: #fff;
  }

  .chiudi path {
    fill: none;
    stroke: currentColor;
    stroke-width: 2;
    stroke-linecap: round;
  }

  .contenuto {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 0 2.25rem 2rem;
    overflow-y: auto;
  }

  .logo {
    width: 112px;
    height: 112px;
    margin-bottom: 1.25rem;
    border-radius: 50%;
    box-shadow: 0 6px 20px rgb(26 47 176 / 0.18);
    user-select: none;
    pointer-events: none;
  }

  /* Contenuto delle schermate di sblocco e primo avvio */
  .contenuto :global(.riquadro) {
    width: 100%;
    display: grid;
    gap: 1rem;
    text-align: center;
  }

  .contenuto :global(h1) {
    margin: 0;
    font-size: 1.4rem;
  }

  .contenuto :global(p) {
    margin: 0;
    color: var(--colore-testo-secondario);
    font-size: 0.95rem;
  }

  .contenuto :global(label) {
    display: grid;
    gap: 0.35rem;
    font-weight: 500;
    font-size: 0.9rem;
    text-align: left;
  }

  .contenuto :global(input) {
    padding: 0.7rem 0.9rem;
    border: 1px solid var(--colore-bordo);
    border-radius: 10px;
    background: var(--colore-superficie);
    transition: border-color 0.15s;
  }

  .contenuto :global(input:focus) {
    outline: none;
    border-color: var(--colore-primario);
    box-shadow: 0 0 0 3px rgb(31 111 235 / 0.15);
  }

  .contenuto :global(button[type="submit"]) {
    margin-top: 0.25rem;
    padding: 0.75rem;
    border: none;
    border-radius: 10px;
    background: var(--colore-primario);
    color: var(--colore-primario-testo);
    font-weight: 600;
  }

  .contenuto :global(button[type="submit"]:hover) {
    filter: brightness(1.08);
  }

  .contenuto :global(button[type="submit"]:disabled) {
    opacity: 0.6;
  }

  .contenuto :global(.avviso) {
    padding: 0.6rem 0.8rem;
    border-radius: 10px;
    background: #fff6e5;
    color: var(--colore-avviso);
    text-align: left;
  }

  .contenuto :global(.errore) {
    color: var(--colore-errore);
  }
</style>
