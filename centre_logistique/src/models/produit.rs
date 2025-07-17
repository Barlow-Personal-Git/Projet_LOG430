use crate::schema::produits;
use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Queryable, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct Produit {
    #[schemars(example = "crate::examples::exemple_id_inventaire")]
    pub id_produit: i32,
    #[schemars(example = "crate::examples::exemple_nom")]
    pub nom: String,
    #[schemars(example = "crate::examples::exemple_prix")]
    pub prix: f32,
    #[schemars(example = "crate::examples::exemple_description")]
    pub description: String,
}

#[derive(Insertable, JsonSchema)]
#[diesel(table_name = produits)]
pub struct NouveauProduit<'a> {
    #[schemars(example = "crate::examples::exemple_nom")]
    pub nom: &'a str,
    #[schemars(example = "crate::examples::exemple_prix")]
    pub prix: f32,
    #[schemars(example = "crate::examples::exemple_description")]
    pub description: &'a str,
}
