-- Schema iniziale. Le date sono testo ISO YYYY-MM-DD, gli importi centesimi interi.
-- I controlli sulle date usano IS: date() restituisce NULL per testo non valido
-- e un CHECK che vale NULL verrebbe considerato superato.

CREATE TABLE iscritti (
    id              INTEGER PRIMARY KEY,
    nome            TEXT    NOT NULL CHECK (trim(nome) <> ''),
    cognome         TEXT    NOT NULL CHECK (trim(cognome) <> ''),
    telefono        TEXT,
    data_iscrizione TEXT    NOT NULL CHECK (date(data_iscrizione) IS data_iscrizione),
    note            TEXT,
    archiviato      INTEGER NOT NULL DEFAULT 0 CHECK (archiviato IN (0, 1)),
    creato_il       TEXT    NOT NULL DEFAULT (datetime('now')),
    modificato_il   TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_iscritti_cognome_nome
    ON iscritti (cognome COLLATE NOCASE, nome COLLATE NOCASE);

CREATE TABLE tipologie_abbonamento (
    id            INTEGER PRIMARY KEY,
    nome          TEXT    NOT NULL UNIQUE COLLATE NOCASE CHECK (trim(nome) <> ''),
    durata_valore INTEGER NOT NULL CHECK (durata_valore > 0),
    durata_unita  TEXT    NOT NULL CHECK (durata_unita IN ('giorni', 'mesi')),
    prezzo_cent   INTEGER NOT NULL CHECK (prezzo_cent >= 0),
    attiva        INTEGER NOT NULL DEFAULT 1 CHECK (attiva IN (0, 1))
);

INSERT INTO tipologie_abbonamento (nome, durata_valore, durata_unita, prezzo_cent) VALUES
    ('Mensile',      1, 'mesi',  5000),
    ('Trimestrale',  3, 'mesi', 13500),
    ('Semestrale',   6, 'mesi', 25000),
    ('Annuale',     12, 'mesi', 45000);

CREATE TABLE pagamenti (
    id             INTEGER PRIMARY KEY,
    iscritto_id    INTEGER NOT NULL REFERENCES iscritti (id) ON DELETE RESTRICT,
    -- Il nome della tipologia è copiato: lo storico non cambia se il listino viene modificato
    tipologia_id   INTEGER REFERENCES tipologie_abbonamento (id) ON DELETE SET NULL,
    tipologia_nome TEXT    NOT NULL,
    importo_cent   INTEGER NOT NULL CHECK (importo_cent >= 0),
    data_pagamento TEXT    NOT NULL CHECK (date(data_pagamento) IS data_pagamento),
    metodo         TEXT    NOT NULL CHECK (metodo IN ('contanti', 'carta', 'bonifico', 'altro')),
    data_inizio    TEXT    NOT NULL CHECK (date(data_inizio) IS data_inizio),
    data_scadenza  TEXT    NOT NULL CHECK (date(data_scadenza) IS data_scadenza
                                           AND data_scadenza >= data_inizio),
    note           TEXT,
    rinnovo_di_id  INTEGER REFERENCES pagamenti (id) ON DELETE SET NULL,
    creato_il      TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_pagamenti_iscritto_scadenza ON pagamenti (iscritto_id, data_scadenza);
CREATE INDEX idx_pagamenti_data_pagamento ON pagamenti (data_pagamento);

CREATE TABLE impostazioni (
    chiave TEXT PRIMARY KEY,
    valore TEXT NOT NULL
);

INSERT INTO impostazioni (chiave, valore) VALUES
    ('giorni_preavviso', '7'),
    ('nome_palestra', ''),
    ('backup_conservati', '30');

-- Evita di notificare più volte la stessa scadenza
CREATE TABLE notifiche_inviate (
    pagamento_id INTEGER NOT NULL REFERENCES pagamenti (id) ON DELETE CASCADE,
    tipo         TEXT    NOT NULL CHECK (tipo IN ('in_scadenza', 'scaduto')),
    inviata_il   TEXT    NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (pagamento_id, tipo)
);

-- Ultima scadenza di ogni iscritto; lo stato è calcolato in Rust (domain::stato)
CREATE VIEW v_ultima_scadenza_iscritti AS
SELECT i.id                  AS iscritto_id,
       MAX(p.data_scadenza)  AS ultima_data_scadenza
FROM iscritti i
LEFT JOIN pagamenti p ON p.iscritto_id = i.id
GROUP BY i.id;
