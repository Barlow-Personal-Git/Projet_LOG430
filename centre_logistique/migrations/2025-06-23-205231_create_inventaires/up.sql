-- Your SQL goes here
CREATE TABLE inventaires (
    id_inventaire SERIAL PRIMARY KEY,
    id_produit INTEGER NOT NULL REFERENCES produits(id_produit),
    id_magasin INTEGER NOT NULL REFERENCES magasins(id_magasin),
    category VARCHAR NOT NULL,
    nbr INTEGER NOT NULL,
    UNIQUE(id_produit, id_magasin)
);
