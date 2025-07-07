use serde::{Serialize, Deserialize};
use crate::models::inventaire::Inventaire;
use crate::models::transaction::Transaction;
use crate::models::transaction_produit::TransactionProduit;

#[derive(Serialize, Deserialize)]
pub struct InventaireDTO<'a> {
    pub magasin: &'a str,
    pub inventaires: Vec<Inventaire>,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionDTO<'a> {
    pub magasin: &'a str,
    pub transactions: Vec<Transaction>,
    pub transaction_produits: Vec<TransactionProduit>,
}
