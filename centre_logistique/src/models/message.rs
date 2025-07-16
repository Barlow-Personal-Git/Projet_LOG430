use crate::models::magasin::Magasin;
use crate::models::produit::Produit;
use crate::schema::messages;
use diesel::{Associations, Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize, Clone, JsonSchema)]
#[diesel(belongs_to(Produit, foreign_key = id_produit))]
#[diesel(belongs_to(Magasin, foreign_key = id_magasin))]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub id_message: i32,
    pub id_produit: i32,
    pub id_magasin: i32,
    pub message: String,
}

#[derive(Insertable, JsonSchema)]
#[diesel(table_name = messages)]
pub struct NouveauMessage<'a> {
    pub id_produit: i32,
    pub id_magasin: i32,
    pub message: &'a str,
}
