use rocket::serde::{Deserialize, Serialize};
use crate::models::{Transaction, TransactionProduit, Message};

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NouveauInventaireDTO<'a> {
    pub id_produit: i32,
    pub category: &'a str,
    pub nbr: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct InventaireDTO<'a> {
    pub magasin: &'a str,
    pub inventaires: Vec<NouveauInventaireDTO<'a>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NouvelleTransactionDTO {
    pub total: f32,
    pub created_date: chrono::NaiveDateTime,
    pub updated_date: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TransactionDTO<'a> {
    pub magasin: &'a str,
    pub transactions: Vec<NouvelleTransactionDTO>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NouveauTransactionProduitDTO {
    pub id_transaction: i32,
    pub id_produit: i32,
    pub nbr: i32,
    pub total: f32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TransactionProduitDTO<'a> {
    pub magasin: &'a str,
    pub transaction_produits: Vec<NouveauTransactionProduitDTO>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NouveauMessageDTO<'a> {
    pub id_produit: i32,
    pub message: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MessageDTO<'a> {
    pub magasin: &'a str,
    pub messages: Vec<NouveauMessageDTO<'a>>,
}
