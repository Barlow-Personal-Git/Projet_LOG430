use crate::models::magasin::Magasin;
use crate::schema::transaction_produits;
use diesel::{Associations, Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use serde_json::Value;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize, Clone, JsonSchema)]
#[diesel(belongs_to(Magasin, foreign_key = id_magasin))]
#[serde(crate = "rocket::serde")]
pub struct TransactionProduit {
    pub id_transaction_produit: i32,
    pub id_transaction: i32,
    pub id_magasin: i32,
    pub produits: Value,
    pub total: f32,
}

#[derive(Insertable, JsonSchema)]
#[diesel(table_name = transaction_produits)]
pub struct NouveauTransactionProduit {
    pub id_transaction: i32,
    pub id_magasin: i32,
    pub produits: Value,
    pub total: f32,
}

#[derive(Queryable, Debug, Serialize, JsonSchema)]
pub struct SommeTransactionProduitParMagasin {
    pub magasin: String,
    pub total: f32,
}
