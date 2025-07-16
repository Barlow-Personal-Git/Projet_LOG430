use crate::models::inventaire::Inventaire;
use crate::models::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InventaireDTO<'a> {
    pub magasin: &'a str,
    pub inventaires: Vec<Inventaire>,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionDTO<'a> {
    pub magasin: &'a str,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TransactionProduitsDTO {
    pub magasin: String,
    pub transaction_produits: Vec<NouvelleTransactionProduitsDTO>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NouvelleTransactionProduitsDTO {
    pub id_transaction: i32,
    pub produits: Vec<NouvelleProduitsDTO>,
    pub total: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NouvelleProduitsDTO {
    pub nom: String,
    pub prix: f32,
    pub nbr: i32,
}
