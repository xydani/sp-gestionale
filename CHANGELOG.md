# Changelog

## [Non rilasciato]

## [0.1.4] - 2026-11-11

- Inserimento rapido di chi era già iscritto prima del programma: nome, cognome e scadenza dell'abbonamento in corso; abbonamento e quota pregressi contano per le scadenze ma non come incassi
- Gli iscritti con anagrafica incompleta risultano «da completare» e si possono filtrare
- Pulsante Blocca ridisegnato nel piè di menu con la versione installata; si blocca anche con Ctrl+L
- «Rinnova» direttamente da Scadenze e dashboard; il rinnovo propone tipologia e metodo dell'ultimo pagamento
- Ricerca rapida degli iscritti da qualsiasi pagina con Ctrl+K
- Cassa di oggi in dashboard: totale, divisione per metodo e movimenti del giorno
- Nella scheda dell'iscritto una striscia riassume cosa sistemare e porta al punto giusto
- Eliminare un pagamento o un'iscrizione non chiede più conferma: per 10 secondi si può premere «Annulla»
- I filtri di Iscritti e Pagamenti restano quando si cambia pagina
- Dimensione del testo regolabile da Impostazioni
- Le pagine ancora vuote spiegano da dove iniziare

## [0.1.3] - 2026-11-10

- Corretto: un importo scritto con il punto (es. 30.50) veniva letto cento volte più grande
- Corretto: dopo un tentativo di salvataggio senza scelta, una tendina obbligatoria bloccava il modulo anche dopo aver scelto
- Corretto: un backup di una versione più recente rendeva l'archivio inapribile; ora viene rifiutato prima
- Corretto: dopo un ripristino la pagina Impostazioni mostrava ancora i valori precedenti
- La finestra non si blocca più durante sblocco, backup ed esportazioni
- Il CSV esportato non contiene testi che Excel eseguirebbe come formule
- Ricerca, paginazione e rinnovo anticipato degli abbonamenti pluriennali più affidabili
- Un abbonamento disattivato nel listino non si può più registrare come nuovo pagamento

## [0.1.2] - 2026-11-10

- Controlli sui dati inseriti: nomi e luoghi senza numeri, date di nascita e di pagamento non nel futuro, telefono ed email validi, importi e durate plausibili; l'errore compare subito sul campo
- Il logo della palestra è incluso nel programma: si vede subito anche su Windows e macOS

## [0.1.1] - 2026-11-10

- Il nuovo iscritto si registra insieme alla quota annuale e al primo abbonamento
- Certificato medico: scadenza nella scheda, avvisi se manca o sta per scadere, scheda dedicata nelle Scadenze e riquadro in dashboard
- La configurazione iniziale non si può più saltare: il programma si sblocca solo a guida completata
- Logo della palestra scelto da Impostazioni; senza logo si vede lo stemma
- Avvisi e conferme in basso a destra, verdi se positivi, che si chiudono da soli
- Blocco automatico dopo alcuni minuti di inattività (impostabile)
- Indicatore di robustezza della password; le nuove password devono avere almeno 12 caratteri
- Avviso prima di esportare il CSV, che non è criptato
- Eliminazione definitiva dei dati di un iscritto archiviato, su sua richiesta (GDPR)
- Aggiornamento in-app anche su macOS

## [0.1.0] - 2026-11-10

Prima versione.

- Archivio criptato con password a ogni avvio, backup giornaliero, manuale e ripristino
- Iscritti con anagrafica, ricerca per nome e cognome, archiviazione
- Iscrizione annuale per anno sportivo e abbonamenti dal listino, con rinnovo e storico
- Scadenze e promemoria allo staff, con notifica desktop
- Dashboard con stati, incassi del mese e prossime scadenze
- Riepilogo incassi per giorno, mese e anno, per metodo, con esportazione CSV
- Impostazioni, cambio password, avvio automatico e configurazione guidata con il listino
- Calendario per scegliere le date
- Stile grafico minimal con menu blu notte e icona con lo stemma della palestra
- Aggiornamento in-app con backup automatico prima dell'installazione
