use diesel::{Queryable, Insertable};
use crate::schema::magasins;

#[derive(Debug, Queryable, Clone)]
pub struct Magasin {
    pub id_magasin: i32,
    pub nom: String,
}

#[derive(Insertable)]
#[diesel(table_name = magasins)]
pub struct NouveauMagasin<'a> {
    pub nom: &'a str,
}