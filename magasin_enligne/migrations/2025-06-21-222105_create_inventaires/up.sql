-- Your SQL goes here
CREATE TABLE inventaires (
    id_inventaire SERIAL PRIMARY KEY,
    id_produit INTEGER NOT NULL UNIQUE REFERENCES produits(id_produit),
    category VARCHAR NOT NULL,
    nbr INTEGER NOT NULL
);
