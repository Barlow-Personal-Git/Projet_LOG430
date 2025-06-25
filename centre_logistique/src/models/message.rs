use diesel::{Queryable, Insertable, Associations};
use rocket::serde::{Serialize, Deserialize};
use crate::models::produit::Produit;
use crate::models::magasin::Magasin;
use crate::schema::messages;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize, Clone)]
#[diesel(belongs_to(Produit, foreign_key = id_produit))]
#[diesel(belongs_to(Magasin, foreign_key = id_magasin))]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub id_message: i32,
    pub id_produit: i32,
    pub id_magasin: i32,
    pub message: String,  
}

#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NouveauMessage<'a> {
    pub id_produit: i32,
    pub id_magasin: i32,
    pub message: &'a str,
}