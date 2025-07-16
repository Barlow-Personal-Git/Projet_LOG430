use crate::schema::clients;
use diesel::{Insertable, Queryable, Selectable};

#[derive(Debug, Queryable, Selectable, Clone)]
pub struct Client {
    pub id_client: i32,
    #[allow(dead_code)]
    pub nom: String,
    #[allow(dead_code)]
    pub role: String,
}

#[derive(Insertable)]
#[diesel(table_name = clients)]
pub struct NouveauClient<'a> {
    pub nom: &'a str,
    pub role: &'a str,
}
