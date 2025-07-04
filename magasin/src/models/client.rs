use diesel::{Queryable, Insertable};
use crate::schema::clients;

#[derive(Debug, Queryable, Clone)]
pub struct Client {
    pub id_client: i32,
    pub nom: String,
    pub role: String,
}

#[derive(Insertable)]
#[diesel(table_name = clients)]
pub struct NouveauClient<'a> {
    pub nom: &'a str,
    pub role: &'a str,
}