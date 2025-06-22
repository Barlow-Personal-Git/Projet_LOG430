use diesel::{Queryable, Insertable};
use crate::schema::produits;

#[derive(Debug, Queryable, Clone)]
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
