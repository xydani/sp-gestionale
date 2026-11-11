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
    messaggioVuoto = "Nessun pagamento",
    onElimina,
  }: {
    /** Pagamenti della pagina corrente. */
    pagamenti: Pagamento[];
    /** Conteggio e totale di tutti i pagamenti filtrati, su tutte le pagine. */
    totaleElementi: number;
    totaleImportoCent: number;
    mostraIscritto?: boolean;
    /** Cosa dire, e cosa fare, quando non ci sono pagamenti. */
    messaggioVuoto?: string;
    onElimina: (pagamento: Pagamento) => void;
  } = $props();
</script>

<table class="tabella riquadrata">
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
        <td>
          {#if pagamento.pregresso}
            <span class="pregresso">Situazione iniziale</span>
          {:else}
            {formattaData(pagamento.dataPagamento)}
          {/if}
        </td>
        {#if mostraIscritto}
          <td>
            {#if pagamento.iscrittoEliminato}
              <span class="eliminato">Iscritto eliminato</span>
            {:else}
              <a href={resolve("/iscritti/[id]", { id: String(pagamento.iscrittoId) })}>
                {pagamento.iscrittoCognome}
                {pagamento.iscrittoNome}
              </a>
            {/if}
          </td>
        {/if}
        <td>
          {pagamento.tipologiaNome}
          {#if pagamento.note}<span class="nota" title={pagamento.note}>📝</span>{/if}
        </td>
        <td>{formattaData(pagamento.dataInizio)} – {formattaData(pagamento.dataScadenza)}</td>
        {#if pagamento.pregresso}
          <td class="pregresso">—</td>
          <td class="importo pregresso" title="Pagato prima di usare il programma">—</td>
        {:else}
          <td>{ETICHETTE_METODO[pagamento.metodo]}</td>
          <td class="importo">{formattaEuro(pagamento.importoCent)}</td>
        {/if}
        <td class="azioni">
          <button type="button" onclick={() => onElimina(pagamento)} title="Elimina pagamento">
            Elimina
          </button>
        </td>
      </tr>
    {:else}
      <tr><td colspan={mostraIscritto ? 7 : 6} class="vuoto">{messaggioVuoto}</td></tr>
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
  .pregresso {
    color: var(--colore-testo-secondario);
  }

  .eliminato {
    color: var(--colore-testo-secondario);
    font-style: italic;
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
