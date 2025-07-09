-- Your SQL goes here
CREATE TABLE reapprovisionnements  (
    id_reapprovisionnement SERIAL PRIMARY KEY,
    id_produit INTEGER NOT NULL REFERENCES produits(id_produit),
    id_magasin INTEGER NOT NULL REFERENCES magasins(id_magasin),
    nbr INTEGER NOT NULL CHECK (nbr > 0),
    status VARCHAR NOT NULL DEFAULT 'en_attente',
    created_date TIMESTAMP NOT NULL DEFAULT NOW()
);