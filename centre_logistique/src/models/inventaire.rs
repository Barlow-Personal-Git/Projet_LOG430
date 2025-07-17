use crate::models::magasin::Magasin;
use crate::models::produit::Produit;
use crate::schema::inventaires;
use diesel::{Associations, Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize, Clone, JsonSchema)]
#[diesel(belongs_to(Produit, foreign_key = id_produit))]
#[diesel(belongs_to(Magasin, foreign_key = id_magasin))]
#[serde(crate = "rocket::serde")]
pub struct Inventaire {
    #[schemars(example = "crate::examples::exemple_id_inventaire")]
    pub id_inventaire: i32,
    #[schemars(example = "crate::examples::exemple_id_produit")]
    pub id_produit: i32,
    #[schemars(example = "crate::examples::exemple_id_magasin")]
    pub id_magasin: i32,
    #[schemars(example = "crate::examples::exemple_category")]
    pub category: String,
    #[schemars(example = "crate::examples::exemple_nbr")]
    pub nbr: i32,
}

#[derive(Insertable)]
#[diesel(table_name = inventaires)]
pub struct NouveauInventaire<'a> {
    pub id_produit: i32,
    pub id_magasin: i32,
    pub category: &'a str,
    pub nbr: i32,
}
