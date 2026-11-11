-- Abbonamenti e quote già pagati prima di usare il programma: contano per stato e scadenze,
-- ma non sono incassi registrati qui, quindi restano fuori da riepilogo ed export.
ALTER TABLE pagamenti ADD COLUMN pregresso INTEGER NOT NULL DEFAULT 0 CHECK (pregresso IN (0, 1));
ALTER TABLE iscrizioni ADD COLUMN pregresso INTEGER NOT NULL DEFAULT 0 CHECK (pregresso IN (0, 1));
