-- Your SQL goes here
CREATE TABLE messages (
    id_message SERIAL PRIMARY KEY,
    id_produit INTEGER NOT NULL REFERENCES produits(id_produit),
    id_magasin INTEGER NOT NULL REFERENCES magasins(id_magasin),
    message TEXT NOT NULL
);