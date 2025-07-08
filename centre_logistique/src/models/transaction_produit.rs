use diesel::{Queryable, Insertable, Associations};
use rocket::serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::models::magasin::Magasin;
use crate::schema::transaction_produits;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize, Clone)]
#[diesel(belongs_to(Magasin, foreign_key = id_magasin))]
#[serde(crate = "rocket::serde")]
pub struct TransactionProduit {
    pub id_transaction_produit: i32,
    pub id_transaction: i32,
    pub id_magasin: i32,
    pub produits: Value,
    pub total: f32,
}

#[derive(Insertable)]
#[diesel(table_name = transaction_produits)]
pub struct NouveauTransactionProduit {
    pub id_transaction: i32,
    pub id_magasin: i32,
    pub produits: Value,
    pub total: f32,
}
