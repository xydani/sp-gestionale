<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";
  import { leggiImpostazioni, messaggioErrore, salvaImpostazioni } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import FormImpostazioni from "$lib/components/FormImpostazioni.svelte";
  import SezioneTipologie from "$lib/components/SezioneTipologie.svelte";
  import { descriviPasso } from "$lib/configurazione-guidata";
  import { palestra } from "$lib/palestra.svelte";
  import type { Impostazioni } from "$lib/types";

  let impostazioni = $state<Impostazioni | null>(null);
  let passo = $state(0);

  onMount(async () => {
    try {
      impostazioni = await leggiImpostazioni();
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  });

  async function salvaOpzioni(scelte: Impostazioni) {
    try {
      impostazioni = await salvaImpostazioni(scelte);
      palestra.nome = impostazioni.nomePalestra;
      passo = 1;
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }

  async function termina() {
    mostraAvviso(
      "successo",
      "Configurazione completata. Puoi cambiarla in Impostazioni e Listino.",
    );
    await goto(resolve("/"));
  }
</script>

<div class="pagina-stretta">
  <h1>Configurazione iniziale</h1>
  <p class="passo">{descriviPasso(passo)}</p>

  {#if passo === 0}
    <section class="scheda">
      <p>
        Prima di iniziare controlla queste opzioni. I valori proposti vanno bene nella maggior parte
        dei casi e potrai sempre cambiarli dalla pagina Impostazioni.
      </p>
      {#if impostazioni}
        <FormImpostazioni
          iniziale={impostazioni}
          etichettaConferma="Avanti"
          onSalva={salvaOpzioni}
        />
      {/if}
    </section>
  {:else}
    <p class="introduzione">
      Questi sono gli abbonamenti proposti quando si registra un pagamento. Correggi i prezzi,
      aggiungi quelli che mancano e disattiva quelli che non usi. Potrai cambiarli in ogni momento
      dalla pagina Listino.
    </p>
    <SezioneTipologie />
    <div class="azioni">
      <button type="button" class="secondario" onclick={() => (passo = 0)}>Indietro</button>
      <button type="button" onclick={termina}>Inizia a usare il programma</button>
    </div>
  {/if}
</div>

<style>
  .passo {
    margin: -0.5rem 0 1rem;
    color: var(--colore-testo-secondario);
    font-weight: 500;
  }

  .introduzione {
    margin: 0 0 1rem;
  }

  .azioni {
    display: flex;
    justify-content: space-between;
    margin-top: 1.25rem;
  }

  .azioni button {
    padding: 0.6rem 1.2rem;
    border-radius: var(--raggio);
    border: 1px solid var(--colore-primario);
    background: var(--colore-primario);
    color: var(--colore-primario-testo);
  }

  .azioni button.secondario {
    background: transparent;
    color: var(--colore-primario);
  }
</style>
