-- Eliminazione su richiesta (GDPR): l'anagrafica viene cancellata, i pagamenti restano
-- anonimi perché i totali degli incassi non devono cambiare. La data segna la riga come eliminata.
ALTER TABLE iscritti ADD COLUMN data_eliminazione TEXT
    CHECK (data_eliminazione IS NULL OR date(data_eliminazione) IS data_eliminazione);
