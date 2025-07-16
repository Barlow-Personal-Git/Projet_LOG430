use crate::db::get_conn;
use crate::dto::{InventaireDTO, TransactionDTO};
use crate::mappers::map_transaction_produits;
use crate::models::inventaire::Inventaire;
use crate::models::produit::Produit;
use crate::models::transaction::Transaction;
use crate::models::transaction_produit::TransactionProduit;
use crate::schema::inventaires::dsl::inventaires;
use crate::schema::produits::dsl::produits;
use crate::schema::transaction_produits::dsl::transaction_produits;
use crate::schema::transactions::dsl::transactions;
use diesel::prelude::*;
use reqwest::Client;
use std::env;

pub async fn sync_data() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("CENTRAL_URL")?;
    let nom_magasin = env::var("MAGASIN")?;

    let client = Client::new();
    let mut conn = get_conn();

    // Inventaire
    let inv: Vec<Inventaire> = inventaires.load(&mut conn)?;

    let inv_dto = InventaireDTO {
        magasin: nom_magasin.as_str(),
        inventaires: inv,
    };
    let url = format!("{base_url}/inventaires");
    client.post(&url).json(&inv_dto).send().await?;

    // Transaction
    let tr: Vec<Transaction> = transactions.load(&mut conn)?;

    let tr_dto = TransactionDTO {
        magasin: nom_magasin.as_str(),
        transactions: tr.clone(),
    };
    let url = format!("{base_url}/transactions");
    client.post(&url).json(&tr_dto).send().await?;

    // Transaction_Produits
    let prods: Vec<Produit> = produits.load(&mut conn)?;
    let trp: Vec<TransactionProduit> = transaction_produits.load(&mut conn)?;

    let trp_dto = map_transaction_produits(&nom_magasin, tr, trp, prods);

    let url = format!("{base_url}/transaction_produits");
    client.post(&url).json(&trp_dto).send().await?;

    Ok(())
}
