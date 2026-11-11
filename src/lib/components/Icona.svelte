<script lang="ts">
  import { ICONE, type NomeIcona } from "$lib/icone";

  /**
   * Icona a tratto nel colore del testo. Dimensioni, colore e tratto sono scritti
   * nell'SVG stesso: si vedono giusti anche se uno stile non arriva (WebView2, WebKitGTK).
   */
  let {
    nome,
    dimensione = 18,
    spessore = 2,
  }: {
    nome: NomeIcona;
    /** In pixel. */
    dimensione?: number;
    spessore?: number;
  } = $props();
</script>

<svg
  viewBox="0 0 24 24"
  width={dimensione}
  height={dimensione}
  fill="none"
  stroke="currentColor"
  stroke-width={spessore}
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
  focusable="false"
  style="display: block; flex-shrink: 0"
>
  <path d={ICONE[nome]} />
</svg>
