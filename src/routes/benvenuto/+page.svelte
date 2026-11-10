<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";
  import { leggiImpostazioni, messaggioErrore, salvaImpostazioni } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import FormImpostazioni from "$lib/components/FormImpostazioni.svelte";
  import { palestra } from "$lib/palestra.svelte";
  import type { Impostazioni } from "$lib/types";

  let impostazioni = $state<Impostazioni | null>(null);

  onMount(async () => {
    try {
      impostazioni = await leggiImpostazioni();
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  });

  async function completa(scelte: Impostazioni) {
    try {
      palestra.nome = (await salvaImpostazioni(scelte)).nomePalestra;
      mostraAvviso("successo", "Configurazione completata. Puoi cambiarla in Impostazioni.");
      await goto(resolve("/"));
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
    }
  }
</script>

<div class="pagina-stretta">
  <h1>Configurazione iniziale</h1>
  <section class="scheda">
    <p>
      Prima di iniziare controlla queste opzioni. I valori proposti vanno bene nella maggior parte
      dei casi e potrai sempre cambiarli dalla pagina Impostazioni. I prezzi degli abbonamenti si
      impostano nel Listino.
    </p>
    {#if impostazioni}
      <FormImpostazioni
        iniziale={impostazioni}
        etichettaConferma="Inizia a usare il programma"
        onSalva={completa}
      />
    {/if}
  </section>
</div>
