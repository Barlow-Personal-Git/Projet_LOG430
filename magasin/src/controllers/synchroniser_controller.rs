    use serde_json::json;
    use std::env;
    use diesel::prelude::*;
    use reqwest::Client;
    use crate::db::get_conn;
    use crate::models::inventaire::Inventaire;
    use crate::models::transaction::Transaction;
    use crate::models::transaction_produit::TransactionProduit;
    use crate::dto::{InventaireDTO, TransactionDTO, TransactionProduitDTO};

    pub async fn sync_data() -> Result<(), Box<dyn std::error::Error>> {

        let base_url = env::var("CENTRAL_URL")?;
        let nom_magasin = env::var("MAGASIN")?;

        let client = Client::new();
        let mut conn = get_conn();

        let inv: Vec<Inventaire> = crate::schema::inventaires::dsl::inventaires
            .load(&mut conn)?;
        
        let inv_dto = InventaireDTO {
            magasin: nom_magasin.as_str(),
            inventaires: inv,
        };
        let url = format!("{}/inventaires", base_url);
        client.post(&url).json(&inv_dto).send().await?;

        let tr: Vec<Transaction> = crate::schema::transactions::dsl::transactions
            .load(&mut conn)?;

        let tr_dto = TransactionDTO {
            magasin: nom_magasin.as_str(),
            transactions: tr,
        };
        let url = format!("{}/transactions", base_url);
        client.post(&url).json(&tr_dto).send().await?;

        let trp: Vec<TransactionProduit> = crate::schema::transaction_produits::dsl::transaction_produits
            .load(&mut conn)?;

        let trp_dto = TransactionProduitDTO {
            magasin: nom_magasin.as_str(),
            transaction_produits: trp,
        };
        let url = format!("{}/transaction_produits", base_url);
        client.post(&url).json(&trp_dto).send().await?;

        println!("Synchronisation terminée !");
        Ok(())
    }
