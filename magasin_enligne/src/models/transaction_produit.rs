use diesel::{Queryable, Insertable, Associations};
use rocket::serde::{Serialize, Deserialize};
use crate::models::produit::Produit;
use crate::models::transaction::Transaction;
use crate::schema::transaction_produits;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Transaction, foreign_key = id_transaction))]
#[diesel(belongs_to(Produit, foreign_key = id_produit))]
#[serde(crate = "rocket::serde")]
pub struct TransactionProduit {
    pub id_transaction_produit: i32,
    pub id_transaction: i32,
    pub id_produit: i32,
    pub nbr: i32,
    pub total: f32,
}

#[derive(Insertable)]
#[diesel(table_name = transaction_produits)]
pub struct NouveauTransactionProduit {
    pub id_transaction: i32,
    pub id_produit: i32,
    pub nbr: i32,
    pub total: f32,
}
