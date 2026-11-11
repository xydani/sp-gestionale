<script lang="ts" module>
  export interface VoceGrafico {
    /** Sotto la colonna, breve (es. "nov"). */
    etichetta: string;
    valore: number;
    /** Il dettaglio completo, mostrato al passaggio (es. "novembre 2026: 172 attivi"). */
    descrizione: string;
    /** Valore poco affidabile (pochi dati): la colonna è più chiara. */
    debole?: boolean;
  }
</script>

<script lang="ts">
  /** Una colonna per voce (di solito per mese), una sola serie: il dettaglio appare al passaggio. */

  let {
    voci,
    massimo,
    etichettaMassimo,
    titoloAccessibile,
  }: {
    voci: VoceGrafico[];
    /** Valore in cima all'asse; se assente, il più alto tra le voci. */
    massimo?: number;
    /** Testo della linea in cima (es. "100%" o "204"). */
    etichettaMassimo: string;
    titoloAccessibile: string;
  } = $props();

  let evidenziata = $state<number | null>(null);

  const cima = $derived(massimo ?? Math.max(1, ...voci.map((voce) => voce.valore)));
  const dettaglio = $derived(
    evidenziata === null ? voci[voci.length - 1]?.descrizione : voci[evidenziata]?.descrizione,
  );

  function altezza(valore: number): string {
    return `${Math.max(0, Math.min(100, (valore / cima) * 100))}%`;
  }
</script>

<figure class="grafico" aria-label={titoloAccessibile}>
  <p class="dettaglio" aria-live="polite">{dettaglio ?? ""}</p>
  <div class="area">
    <span class="riga-cima" aria-hidden="true"><span>{etichettaMassimo}</span></span>
    <div class="colonne" role="group" onpointerleave={() => (evidenziata = null)}>
      {#each voci as voce, indice (indice)}
        <!-- Tutta la colonna è sensibile al passaggio, anche sopra la barra; con Tab si scorre -->
        <button
          type="button"
          class="colonna"
          class:attiva={evidenziata === indice}
          aria-label={voce.descrizione}
          onpointerenter={() => (evidenziata = indice)}
          onfocus={() => (evidenziata = indice)}
          onblur={() => (evidenziata = null)}
        >
          <span class="barra" class:debole={voce.debole} style:height={altezza(voce.valore)}></span>
        </button>
      {/each}
    </div>
  </div>
  <div class="etichette" aria-hidden="true">
    {#each voci as voce, indice (indice)}<span>{voce.etichetta}</span>{/each}
  </div>
  <details>
    <summary>Mostra i numeri</summary>
    <ul class="numeri">
      {#each voci as voce, indice (indice)}<li>{voce.descrizione}</li>{/each}
    </ul>
  </details>
</figure>

<style>
  .grafico {
    margin: 0;
    display: grid;
    gap: 0.35rem;
  }

  .dettaglio {
    min-height: 1.4em;
    margin: 0;
    font-weight: 500;
    font-variant-numeric: tabular-nums;
  }

  .area {
    position: relative;
    height: 10rem;
    border-bottom: 1px solid var(--colore-bordo-hover);
  }

  /* Linea guida leggera in cima: dà la scala senza appesantire */
  .riga-cima {
    position: absolute;
    inset: 0 0 auto;
    border-top: 1px dashed var(--colore-bordo);
  }

  .riga-cima span {
    position: absolute;
    top: -0.65rem;
    right: 0;
    padding-left: 0.3rem;
    background: var(--colore-superficie);
    color: var(--colore-testo-secondario);
    font-size: 0.75rem;
  }

  .colonne,
  .etichette {
    display: grid;
    grid-auto-flow: column;
    grid-auto-columns: 1fr;
    gap: 2px;
  }

  .colonne {
    height: 100%;
    padding-top: 0.25rem;
  }

  /* Spazio a destra per l'etichetta della scala, così non copre l'ultima colonna */
  .colonne,
  .etichette {
    padding-right: 2.75rem;
  }

  /* La barra è posizionata dentro la colonna: più affidabile di flex dentro un pulsante */
  .colonna {
    position: relative;
    padding: 0;
    border: none;
    background: transparent;
    height: 100%;
    border-radius: 4px 4px 0 0;
    cursor: default;
    outline-offset: -2px;
  }

  .colonna.attiva {
    background: var(--colore-sfondo-tenue);
  }

  .barra {
    position: absolute;
    bottom: 0;
    left: 50%;
    transform: translateX(-50%);
    width: min(70%, 2.25rem);
    min-height: 1px;
    border-radius: 4px 4px 0 0;
    background: var(--colore-primario);
    opacity: 0.85;
  }

  .colonna.attiva .barra {
    opacity: 1;
  }

  .barra.debole {
    opacity: 0.3;
  }

  .colonna.attiva .barra.debole {
    opacity: 0.45;
  }

  summary {
    width: fit-content;
    color: var(--colore-primario);
    font-size: 0.85rem;
    cursor: pointer;
  }

  .numeri {
    margin: 0.5rem 0 0;
    padding-left: 1.25rem;
    columns: 2;
    font-size: 0.9rem;
    font-variant-numeric: tabular-nums;
  }

  .etichette span {
    overflow: hidden;
    color: var(--colore-testo-secondario);
    font-size: 0.75rem;
    text-align: center;
    white-space: nowrap;
  }
</style>
