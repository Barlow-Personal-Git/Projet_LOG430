-- Your SQL goes here
CREATE TABLE transactions (
    id_transaction SERIAL PRIMARY KEY,
    id_magasin INTEGER NOT NULL REFERENCES magasins(id_magasin),
    total REAL NOT NULL,
    created_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(id_magasin, created_date, updated_date)
);

-- La mettre à jour automatique updated_date
CREATE OR REPLACE FUNCTION update_updated_date_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_date = CURRENT_TIMESTAMP;
   RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Appel du function trigger avant chaque Update
CREATE TRIGGER update_transaction_updated_date
BEFORE UPDATE ON transactions
FOR EACH ROW
EXECUTE PROCEDURE update_updated_date_column();
