-- Your SQL goes here
ALTER TABLE transaction_produits
ADD CONSTRAINT transaction_produits_unique UNIQUE (id_magasin, id_transaction);
ALTER TABLE transactions
ADD CONSTRAINT transaction_unique UNIQUE (id_magasin, created_date);
