use diesel::{Queryable, Insertable, Associations};
use rocket::serde::{Serialize, Deserialize};
use crate::models::produit::Produit;
use crate::models::magasin::Magasin;
use crate::schema::inventaires;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize, Clone)]
#[diesel(belongs_to(Produit, foreign_key = id_produit))]
#[diesel(belongs_to(Magasin, foreign_key = id_magasin))]
#[serde(crate = "rocket::serde")]
pub struct Inventaire {
    pub id_inventaire: i32,
    pub id_produit: i32,
    pub id_magasin: i32,
    pub category: String,  
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