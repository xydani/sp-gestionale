-- I database esistenti sono già in uso: la configurazione guidata serve solo ai nuovi,
-- che alla creazione impostano questo valore a '0'.
INSERT INTO impostazioni (chiave, valore) VALUES ('configurazione_iniziale_completata', '1');
