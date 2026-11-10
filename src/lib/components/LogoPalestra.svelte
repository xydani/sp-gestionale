<script lang="ts">
  import { PERCORSI_LOGO, prossimoPercorsoLogo } from "$lib/logo";

  let { altezza }: { altezza: string } = $props();

  let percorso = $state<string | null>(PERCORSI_LOGO[0]);

  function provaIlProssimo() {
    percorso = percorso ? prossimoPercorsoLogo(percorso) : null;
  }
</script>

{#if percorso}
  <img src={percorso} alt="Logo della palestra" style:height={altezza} onerror={provaIlProssimo} />
{/if}

<style>
  img {
    display: block;
    max-width: 100%;
    object-fit: contain;
  }
</style>
