-- Your SQL goes here
CREATE TABLE transaction_produits (
    id_transaction_produit SERIAL PRIMARY KEY,
    id_transaction INTEGER NOT NULL REFERENCES transactions(id_transaction),
    id_produit INTEGER NOT NULL REFERENCES produits(id_produit),
    nbr INTEGER NOT NULL,
    total REAL NOT NULL
);