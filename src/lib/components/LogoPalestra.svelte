<script lang="ts">
  import { onMount } from "svelte";
  import { PERCORSI_LOGO_PREDEFINITI, prossimoPercorsoLogo } from "$lib/logo";
  import { caricaLogoPalestra, palestra } from "$lib/palestra.svelte";

  let { altezza }: { altezza: string } = $props();

  let caricato = $state(false);
  let logoIllegibile = $state(false);
  let predefinito = $state<string | null>(PERCORSI_LOGO_PREDEFINITI[0]);

  // Finché non si sa se c'è un logo scelto non si mostra nulla, per evitare un cambio a vista
  const sorgente = $derived(
    caricato ? (palestra.logo && !logoIllegibile ? palestra.logo : predefinito) : null,
  );

  onMount(async () => {
    await caricaLogoPalestra();
    caricato = true;
  });

  $effect(() => {
    // Un nuovo logo scelto in Impostazioni va riprovato anche se il precedente era illeggibile
    void palestra.logo;
    logoIllegibile = false;
  });

  function provaIlProssimo() {
    if (sorgente === palestra.logo) logoIllegibile = true;
    else if (predefinito) predefinito = prossimoPercorsoLogo(predefinito);
  }
</script>

{#if sorgente}
  <img src={sorgente} alt="Logo della palestra" style:height={altezza} onerror={provaIlProssimo} />
{/if}

<style>
  img {
    display: block;
    max-width: 100%;
    object-fit: contain;
  }
</style>
