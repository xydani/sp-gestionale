-- Promemoria delle scadenze già notificati allo staff, per abbonamenti e iscrizioni.
-- La chiave è iscritto + data di scadenza: se l'iscritto rinnova, la nuova scadenza
-- verrà notificata a sua volta; se un pagamento viene eliminato non resta nulla da pulire.

DROP TABLE notifiche_inviate;

CREATE TABLE promemoria_inviati (
    tipo          TEXT    NOT NULL CHECK (tipo IN ('abbonamento', 'iscrizione')),
    iscritto_id   INTEGER NOT NULL REFERENCES iscritti (id) ON DELETE CASCADE,
    data_scadenza TEXT    NOT NULL CHECK (date(data_scadenza) IS data_scadenza),
    inviato_il    TEXT    NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (tipo, iscritto_id, data_scadenza)
);
