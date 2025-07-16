use crate::schema::magasins;
use diesel::{Insertable, Queryable};

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
