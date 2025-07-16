use crate::schema::produits;
use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Produit {
    pub id_produit: i32,
    pub nom: String,
    pub prix: f32,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = produits)]
pub struct NouveauProduit<'a> {
    pub nom: &'a str,
    pub prix: f32,
    pub description: &'a str,
}
