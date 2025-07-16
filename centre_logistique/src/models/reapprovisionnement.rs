use crate::models::magasin::Magasin;
use crate::models::produit::Produit;
use crate::schema::reapprovisionnements;
use chrono::NaiveDateTime;
use diesel::{Associations, Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize, Clone, JsonSchema)]
#[diesel(belongs_to(Magasin, foreign_key = id_magasin))]
#[diesel(belongs_to(Produit, foreign_key = id_produit))]
#[serde(crate = "rocket::serde")]
pub struct Reapprovisionnement {
    pub id_reapprovisionnement: i32,
    pub id_produit: i32,
    pub id_magasin: i32,
    pub nbr: i32,
    pub status: String,
    pub created_date: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = reapprovisionnements)]
pub struct NouveauReapprovisionnement {
    pub id_produit: i32,
    pub id_magasin: i32,
    pub nbr: i32,
    pub status: String,
    pub created_date: NaiveDateTime,
}
