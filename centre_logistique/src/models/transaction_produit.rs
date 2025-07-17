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
    #[schemars(example = "crate::examples::exemple_id_transaction_produit")]
    pub id_transaction_produit: i32,
    #[schemars(example = "crate::examples::exemple_id_transaction")]
    pub id_transaction: i32,
    #[schemars(example = "crate::examples::exemple_id_magasin")]
    pub id_magasin: i32,
    #[schemars(example = "crate::examples::exemple_produits")]
    pub produits: Value,
    #[schemars(example = "crate::examples::exemple_total")]
    pub total: f32,
}

#[derive(Insertable, JsonSchema)]
#[diesel(table_name = transaction_produits)]
pub struct NouveauTransactionProduit {
    #[schemars(example = "crate::examples::exemple_id_transaction")]
    pub id_transaction: i32,
    #[schemars(example = "crate::examples::exemple_id_magasin")]
    pub id_magasin: i32,
    #[schemars(example = "crate::examples::exemple_produits")]
    pub produits: Value,
    #[schemars(example = "crate::examples::exemple_total")]
    pub total: f32,
}

#[derive(Queryable, Debug, Serialize, JsonSchema)]
pub struct SommeTransactionProduitParMagasin {
    pub magasin: String,
    #[schemars(example = "crate::examples::exemple_total")]
    pub total: f32,
}
