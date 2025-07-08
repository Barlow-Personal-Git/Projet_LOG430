-- Your SQL goes here
CREATE TABLE transaction_produits (
    id_transaction_produit SERIAL PRIMARY KEY,
    id_transaction INTEGER NOT NULL,
    id_magasin INTEGER NOT NULL REFERENCES magasins(id_magasin),
    produits JSONB NOT NULL,
    total REAL NOT NULL
);