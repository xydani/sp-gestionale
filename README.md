# SP Gestionale

Gestionale desktop per gli abbonamenti di una palestra: iscritti, pagamenti, scadenze, incassi.

Realizzato con [Tauri 2](https://v2.tauri.app) (Rust) e SvelteKit (TypeScript). Funziona su Windows, Linux e macOS.

## Installazione

Scaricare l'installer dall'[ultima release](https://github.com/xydani/sp-gestionale/releases/latest):

- **Windows**: `SP.Gestionale_x.y.z_x64-setup.exe`. Gli installer non hanno firma Authenticode:
  al primo avvio Windows SmartScreen può mostrare un avviso ("Ulteriori informazioni" › "Esegui comunque").
- **Linux**: `.deb`, `.rpm` oppure `.AppImage`.
- **macOS**: `.dmg` (Apple Silicon e Intel). L'app non è notarizzata: al primo avvio aprirla con
  clic destro › Apri.

## Aggiornamenti

Il programma controlla all'avvio e ogni 24 ore se c'è una nuova versione. Con **Aggiorna ora**
salva un backup dei dati, scarica l'aggiornamento, ne verifica la firma, lo installa e si riavvia.
Da **Impostazioni › Aggiornamenti** si vede la versione installata e si può verificare a mano.

## Dati

- I dati sono salvati in un database SQLite **criptato** (SQLCipher) nella cartella dati utente
  (`%APPDATA%\it.sp.gestionale\` su Windows), mai nella cartella di installazione.
- La password viene scelta al primo avvio. **Se si perde, i dati non sono recuperabili.**

## Sviluppo

Prerequisiti: Node 22+, Rust stable e le [dipendenze di sistema di Tauri](https://v2.tauri.app/start/prerequisites/).

```bash
npm install
npm run tauri dev     # avvia l'app in sviluppo
```

## Controlli

```bash
npm test              # test frontend (Vitest)
npm run check         # controllo tipi Svelte/TypeScript
npm run lint          # Prettier + ESLint
npm run test:rust     # test backend
npm run lint:rust     # rustfmt + clippy
```

## Licenza

[MIT](LICENSE)
