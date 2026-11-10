<script lang="ts">
  import { resolve } from "$app/paths";
  import { formattaData, formattaEuro } from "$lib/format";
  import { ETICHETTE_METODO } from "$lib/metodi-pagamento";
  import type { Pagamento } from "$lib/types";

  let {
    pagamenti,
    totaleElementi,
    totaleImportoCent,
    mostraIscritto = true,
    onElimina,
  }: {
    /** Pagamenti della pagina corrente. */
    pagamenti: Pagamento[];
    /** Conteggio e totale di tutti i pagamenti filtrati, su tutte le pagine. */
    totaleElementi: number;
    totaleImportoCent: number;
    mostraIscritto?: boolean;
    onElimina: (pagamento: Pagamento) => void;
  } = $props();
</script>

<table>
  <thead>
    <tr>
      <th>Data</th>
      {#if mostraIscritto}<th>Iscritto</th>{/if}
      <th>Tipologia</th>
      <th>Periodo</th>
      <th>Metodo</th>
      <th class="importo">Importo</th>
      <th></th>
    </tr>
  </thead>
  <tbody>
    {#each pagamenti as pagamento (pagamento.id)}
      <tr>
        <td>{formattaData(pagamento.dataPagamento)}</td>
        {#if mostraIscritto}
          <td>
            <a href={resolve("/iscritti/[id]", { id: String(pagamento.iscrittoId) })}>
              {pagamento.iscrittoCognome}
              {pagamento.iscrittoNome}
            </a>
          </td>
        {/if}
        <td>
          {pagamento.tipologiaNome}
          {#if pagamento.note}<span class="nota" title={pagamento.note}>📝</span>{/if}
        </td>
        <td>{formattaData(pagamento.dataInizio)} – {formattaData(pagamento.dataScadenza)}</td>
        <td>{ETICHETTE_METODO[pagamento.metodo]}</td>
        <td class="importo">{formattaEuro(pagamento.importoCent)}</td>
        <td class="azioni">
          <button type="button" onclick={() => onElimina(pagamento)} title="Elimina pagamento">
            Elimina
          </button>
        </td>
      </tr>
    {:else}
      <tr><td colspan={mostraIscritto ? 7 : 6} class="vuoto">Nessun pagamento</td></tr>
    {/each}
  </tbody>
  {#if pagamenti.length > 0}
    <tfoot>
      <tr>
        <td colspan={mostraIscritto ? 5 : 4}>Totale ({totaleElementi} pagamenti)</td>
        <td class="importo">{formattaEuro(totaleImportoCent)}</td>
        <td></td>
      </tr>
    </tfoot>
  {/if}
</table>

<style>
  table {
    width: 100%;
    border-collapse: collapse;
    background: var(--colore-superficie);
    border: 1px solid var(--colore-bordo);
    border-radius: var(--raggio);
    overflow: hidden;
  }

  th,
  td {
    text-align: left;
    padding: 0.6rem 1rem;
    border-bottom: 1px solid var(--colore-bordo);
  }

  th {
    color: var(--colore-testo-secondario);
    font-weight: 500;
    background: var(--colore-sfondo);
  }

  .importo {
    text-align: right;
    white-space: nowrap;
  }

  tfoot td {
    font-weight: 600;
    border-bottom: none;
  }

  a {
    color: inherit;
    text-decoration: none;
    font-weight: 500;
  }

  a:hover {
    color: var(--colore-primario);
  }

  .nota {
    cursor: help;
  }

  .azioni {
    text-align: right;
  }

  .azioni button {
    border: none;
    background: none;
    color: var(--colore-errore);
    padding: 0.2rem 0.4rem;
    opacity: 0.7;
  }

  .azioni button:hover {
    opacity: 1;
  }

  .vuoto {
    text-align: center;
    color: var(--colore-testo-secondario);
    padding: 1.5rem;
  }
</style>
