-- Iscrizione annuale separata dagli abbonamenti e anagrafica completa.

-- Anagrafica: data e luogo di nascita (obbligatori per i nuovi iscritti, verificati
-- dal programma; nel database restano facoltativi per gli iscritti già presenti) ed email.
ALTER TABLE iscritti ADD COLUMN data_nascita TEXT
    CHECK (data_nascita IS NULL OR date(data_nascita) IS data_nascita);
ALTER TABLE iscritti ADD COLUMN luogo_nascita TEXT;
ALTER TABLE iscritti ADD COLUMN email TEXT;

-- La data di iscrizione non sta più nell'anagrafica: è il periodo dell'iscrizione annuale.
ALTER TABLE iscritti DROP COLUMN data_iscrizione;

-- Iscrizioni annuali (anno sportivo) con la quota pagata
CREATE TABLE iscrizioni (
    id             INTEGER PRIMARY KEY,
    iscritto_id    INTEGER NOT NULL REFERENCES iscritti (id) ON DELETE RESTRICT,
    importo_cent   INTEGER NOT NULL CHECK (importo_cent >= 0),
    data_pagamento TEXT    NOT NULL CHECK (date(data_pagamento) IS data_pagamento),
    metodo         TEXT    NOT NULL CHECK (metodo IN ('contanti', 'carta', 'bonifico', 'altro')),
    data_inizio    TEXT    NOT NULL CHECK (date(data_inizio) IS data_inizio),
    data_fine      TEXT    NOT NULL CHECK (date(data_fine) IS data_fine AND data_fine >= data_inizio),
    note           TEXT,
    creato_il      TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_iscrizioni_iscritto_fine ON iscrizioni (iscritto_id, data_fine);
CREATE INDEX idx_iscrizioni_data_pagamento ON iscrizioni (data_pagamento);

INSERT INTO impostazioni (chiave, valore) VALUES
    ('mese_inizio_anno_sportivo', '9'),
    ('quota_iscrizione_cent', '3000');

-- Ultima data di fine iscrizione di ogni iscritto; lo stato è calcolato in Rust
CREATE VIEW v_ultima_iscrizione_iscritti AS
SELECT i.id               AS iscritto_id,
       MAX(s.data_fine)   AS ultima_data_fine_iscrizione
FROM iscritti i
LEFT JOIN iscrizioni s ON s.iscritto_id = i.id
GROUP BY i.id;
