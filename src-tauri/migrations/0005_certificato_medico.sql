-- Certificato medico: la data di scadenza; NULL significa che non è stato presentato.
ALTER TABLE iscritti ADD COLUMN data_scadenza_certificato TEXT
    CHECK (data_scadenza_certificato IS NULL
           OR date(data_scadenza_certificato) IS data_scadenza_certificato);

-- Stessa forma delle viste di abbonamento e iscrizione: promemoria e conteggi le trattano allo stesso modo
CREATE VIEW v_certificato_iscritti AS
SELECT id AS iscritto_id, data_scadenza_certificato
FROM iscritti;

-- Anche i certificati in scadenza vanno notificati: il vincolo sul tipo non si può modificare,
-- quindi la tabella viene ricreata con i dati già presenti.
CREATE TABLE promemoria_inviati_nuova (
    tipo          TEXT    NOT NULL CHECK (tipo IN ('abbonamento', 'iscrizione', 'certificato')),
    iscritto_id   INTEGER NOT NULL REFERENCES iscritti (id) ON DELETE CASCADE,
    data_scadenza TEXT    NOT NULL CHECK (date(data_scadenza) IS data_scadenza),
    inviato_il    TEXT    NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (tipo, iscritto_id, data_scadenza)
);
INSERT INTO promemoria_inviati_nuova SELECT tipo, iscritto_id, data_scadenza, inviato_il
FROM promemoria_inviati;
DROP TABLE promemoria_inviati;
ALTER TABLE promemoria_inviati_nuova RENAME TO promemoria_inviati;
