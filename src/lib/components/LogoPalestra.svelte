<script lang="ts">
  import { onMount } from "svelte";
  import { LOGO_PREDEFINITO } from "$lib/logo";
  import { caricaLogoPalestra, palestra } from "$lib/palestra.svelte";

  let { altezza }: { altezza: string } = $props();

  let caricato = $state(false);
  let logoIllegibile = $state(false);

  // Finché non si sa se c'è un logo scelto non si mostra nulla, per evitare un cambio a vista
  const sorgente = $derived(caricato ? sceltaLogo() : null);

  function sceltaLogo(): string {
    if (palestra.logo && !logoIllegibile) return palestra.logo;
    return LOGO_PREDEFINITO;
  }

  onMount(async () => {
    await caricaLogoPalestra();
    caricato = true;
  });

  $effect(() => {
    // Un nuovo logo scelto in Impostazioni va riprovato anche se il precedente era illeggibile
    void palestra.logo;
    logoIllegibile = false;
  });

  /** Un logo scelto ma illeggibile lascia il posto a quello predefinito. */
  function ripiegaSulPredefinito() {
    if (sorgente === palestra.logo) logoIllegibile = true;
  }
</script>

{#if sorgente}
  <img
    src={sorgente}
    alt="Logo della palestra"
    style:height={altezza}
    onerror={ripiegaSulPredefinito}
  />
{/if}

<style>
  img {
    display: block;
    max-width: 100%;
    object-fit: contain;
  }
</style>
