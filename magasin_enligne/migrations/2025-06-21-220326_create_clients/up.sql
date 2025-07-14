-- Your SQL goes here
CREATE TABLE clients (
    id_client SERIAL PRIMARY KEY,
    nom VARCHAR NOT NULL,
    role VARCHAR NOT NULL DEFAULT 'user',
    CHECK (role IN ('admin', 'user'))
);
