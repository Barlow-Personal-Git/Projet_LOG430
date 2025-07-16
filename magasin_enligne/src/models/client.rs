use crate::schema::clients;
use diesel::{Insertable, Queryable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
