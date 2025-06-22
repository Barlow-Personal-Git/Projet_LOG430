-- Your SQL goes here
CREATE TABLE produits (
    id_produit SERIAL PRIMARY KEY,
    nom VARCHAR NOT NULL,
    prix REAL NOT NULL,
    description VARCHAR NOT NULL
);
