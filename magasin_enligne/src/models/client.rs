use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use crate::schema::clients;
use schemars::JsonSchema;

#[derive(Debug, Queryable, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Client {
    pub id_client: i32,
    pub nom: String,
    pub role: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = clients)]
pub struct NouveauClient<'a> {
    pub nom: &'a str,
    pub role: &'a str,
}