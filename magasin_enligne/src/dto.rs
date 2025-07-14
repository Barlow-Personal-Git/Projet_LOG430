use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::NaiveDateTime;
use diesel::Queryable;
use rocket::form::FromForm;

#[derive(Queryable, Debug, Clone, Serialize, Deserialize, JsonSchema, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct LoginDTO {
    pub nom: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct TransactionDTO {
    pub id_produit: i32,
    pub nbr: u32,
}
