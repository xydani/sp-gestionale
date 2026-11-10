<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { impostaLogoPalestra, messaggioErrore, rimuoviLogoPalestra } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { ESTENSIONI_LOGO } from "$lib/logo";
  import { caricaLogoPalestra, palestra } from "$lib/palestra.svelte";
  import LogoPalestra from "./LogoPalestra.svelte";

  let inCorso = $state(false);

  async function esegui(operazione: () => Promise<void>, conferma: string) {
    inCorso = true;
    try {
      await operazione();
      await caricaLogoPalestra();
      mostraAvviso("successo", conferma);
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    } finally {
      inCorso = false;
    }
  }

  async function scegliImmagine() {
    const file = await open({
      title: "Scegli il logo della palestra",
      filters: [{ name: "Immagini", extensions: [...ESTENSIONI_LOGO] }],
    });
    if (file) await esegui(() => impostaLogoPalestra(file), "Logo aggiornato");
  }
</script>

<section class="scheda">
  <h2>Logo</h2>
  <div class="contenuto">
    <div class="anteprima"><LogoPalestra altezza="64px" /></div>
    <div class="testo">
      <p>
        Il logo compare nel menu e nella schermata della password. Va bene un'immagine PNG, JPG,
        WEBP o SVG fino a 2 MB.
      </p>
      <div class="azioni">
        <button type="button" class="pulsante" onclick={scegliImmagine} disabled={inCorso}>
          Scegli immagine…
        </button>
        {#if palestra.logo}
          <button
            type="button"
            class="pulsante secondario"
            onclick={() => esegui(rimuoviLogoPalestra, "Ripristinato lo stemma predefinito")}
            disabled={inCorso}
          >
            Usa lo stemma predefinito
          </button>
        {/if}
      </div>
    </div>
  </div>
</section>

<style>
  .contenuto {
    display: flex;
    gap: 1.25rem;
    align-items: center;
  }

  .anteprima {
    display: grid;
    place-items: center;
    flex: none;
    width: 96px;
    height: 96px;
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio-grande);
    background: var(--colore-superficie);
  }

  p {
    margin: 0 0 0.75rem;
    color: var(--colore-testo-secondario);
  }

  .azioni {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
</style>
