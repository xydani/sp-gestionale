<script lang="ts">
  import { onMount } from "svelte";
  import { messaggioErrore, trovaIscritto } from "$lib/api";
  import { mostraAvviso } from "$lib/avvisi.svelte";
  import { aggiornaConteggioScadenze } from "$lib/promemoria.svelte";
  import type { TipoRinnovabile } from "$lib/rinnovo";
  import type { Iscritto } from "$lib/types";
  import FinestraSovrapposta from "./FinestraSovrapposta.svelte";
  import FormIscrizione from "./FormIscrizione.svelte";
  import FormPagamento from "./FormPagamento.svelte";

  /** Rinnovo al volo da un elenco, senza passare dalla scheda dell'iscritto. */
  let {
    iscrittoId,
    tipo,
    onChiusa,
  }: {
    iscrittoId: number;
    tipo: TipoRinnovabile;
    /** `registrato` è vero se il rinnovo è stato salvato: l'elenco va ricaricato. */
    onChiusa: (registrato: boolean) => void;
  } = $props();

  const TITOLI: Record<TipoRinnovabile, string> = {
    abbonamento: "Rinnova abbonamento",
    iscrizione: "Rinnova iscrizione annuale",
  };

  let iscritto = $state<Iscritto | null>(null);

  onMount(async () => {
    try {
      iscritto = await trovaIscritto(iscrittoId);
    } catch (errore) {
      mostraAvviso("errore", messaggioErrore(errore));
      onChiusa(false);
    }
  });

  async function registrato() {
    mostraAvviso("successo", `${TITOLI[tipo]}: fatto per ${iscritto?.nome ?? "l'iscritto"}`);
    await aggiornaConteggioScadenze();
    onChiusa(true);
  }
</script>

<FinestraSovrapposta
  titolo={TITOLI[tipo]}
  sottotitolo={iscritto ? `${iscritto.cognome} ${iscritto.nome}` : null}
  onChiusa={() => onChiusa(false)}
>
  {#if iscritto && tipo === "abbonamento"}
    <FormPagamento
      iscrittoIniziale={iscritto}
      onRegistrato={registrato}
      onAnnulla={() => onChiusa(false)}
    />
  {:else if iscritto}
    <FormIscrizione
      iscrittoId={iscritto.id}
      onRegistrata={registrato}
      onAnnulla={() => onChiusa(false)}
    />
  {/if}
</FinestraSovrapposta>
