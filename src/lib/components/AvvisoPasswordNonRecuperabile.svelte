<script lang="ts">
  import { AVVISO_PASSWORD_NON_RECUPERABILE } from "$lib/password";

  /** Avviso e conferma obbligatoria: senza la spunta il modulo della nuova password non parte. */
  let { conservata = $bindable(false) }: { conservata?: boolean } = $props();
</script>

<div class="avviso-password">
  <p><strong>Attenzione:</strong> {AVVISO_PASSWORD_NON_RECUPERABILE}</p>
  <label>
    <input type="checkbox" bind:checked={conservata} required />
    Ho scritto la password e la conservo in un posto sicuro
  </label>
</div>

<style>
  .avviso-password {
    display: grid;
    gap: 0.5rem;
    padding: 0.7rem 0.85rem;
    border-radius: 10px;
    background: var(--colore-avviso-tenue);
    color: var(--colore-avviso);
    text-align: left;
    font-size: 0.9rem;
  }

  .avviso-password p {
    margin: 0;
    color: inherit;
  }

  /* Più specifico degli stili generici della finestra di accesso, che vale per i campi di testo */
  .avviso-password label {
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
    color: var(--colore-testo);
    font-weight: 500;
  }

  .avviso-password input {
    width: 1rem;
    height: 1rem;
    margin: 0.15rem 0 0;
    padding: 0;
    flex-shrink: 0;
  }
</style>
