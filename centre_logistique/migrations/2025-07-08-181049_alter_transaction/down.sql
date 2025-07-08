-- This file should undo anything in `up.sql`
ALTER TABLE transaction_produits
DROP CONSTRAINT transaction_produits_unique;

ALTER TABLE transactions
DROP CONSTRAINT transaction_unique;