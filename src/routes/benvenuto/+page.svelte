<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";
  import {
    completaConfigurazioneIniziale,
    leggiImpostazioni,
    messaggioErrore,
    salvaImpostazioni,
  } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import FormImpostazioni from "$lib/components/FormImpostazioni.svelte";
  import SezioneLogo from "$lib/components/SezioneLogo.svelte";
  import SezioneTipologie from "$lib/components/SezioneTipologie.svelte";
  import { configurazione, descriviPasso } from "$lib/configurazione-guidata.svelte";
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
    try {
      await completaConfigurazioneIniziale();
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
      return;
    }
    configurazione.completata = true;
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
    <SezioneLogo />
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
      <button type="button" class="pulsante secondario" onclick={() => (passo = 0)}>Indietro</button
      >
      <button class="pulsante" type="button" onclick={termina}>Inizia a usare il programma</button>
    </div>
  {/if}
</div>

<style>
  .passo {
    margin: -0.5rem 0 1rem;
    color: var(--colore-testo-secondario);
    font-weight: 500;
  }

  :global(.scheda) + .scheda {
    margin-top: 1.25rem;
  }

  .introduzione {
    margin: 0 0 1rem;
  }

  .azioni {
    display: flex;
    justify-content: space-between;
    margin-top: 1.25rem;
  }
</style>
