-- Certificato medico consegnato senza una data di scadenza indicata: vale come presentato.
ALTER TABLE iscritti ADD COLUMN certificato_senza_scadenza INTEGER NOT NULL DEFAULT 0
    CHECK (certificato_senza_scadenza IN (0, 1));

-- La vista dà a promemoria, conteggi e filtri la scadenza "effettiva": per un certificato
-- presentato senza data una scadenza lontanissima, così risulta valido e non viene segnalato.
DROP VIEW v_certificato_iscritti;
CREATE VIEW v_certificato_iscritti AS
SELECT id AS iscritto_id,
       CASE WHEN data_scadenza_certificato IS NULL AND certificato_senza_scadenza = 1
            THEN '9999-12-31'
            ELSE data_scadenza_certificato
       END AS data_scadenza_certificato
FROM iscritti;
