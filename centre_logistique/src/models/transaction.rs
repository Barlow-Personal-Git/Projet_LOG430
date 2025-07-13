use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::{Text, Nullable, Float};
use diesel::{Queryable, Insertable, Associations};
use rocket::serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use crate::models::magasin::Magasin;
use crate::schema::transactions;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize, Clone, JsonSchema)]
#[diesel(belongs_to(Magasin, foreign_key = id_magasin))]
#[serde(crate = "rocket::serde")]
pub struct Transaction {
    pub id_transaction: i32,
    pub id_magasin: i32,
    pub total: f32,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct NouvelleTransaction {
    pub id_magasin: i32,
    pub total: f32,
    
    #[diesel(sql_type = Timestamp)]
    pub created_date: NaiveDateTime,

    #[diesel(sql_type = Timestamp)]
    pub updated_date: NaiveDateTime,
}

#[derive(QueryableByName, Debug)]
pub struct TendancesHebdoSQL {
    #[diesel(sql_type = Text)]
    pub nom: String,

    #[diesel(sql_type = Text)]
    pub semaine: String,

    #[diesel(sql_type = Nullable<Float>)]
    pub total: Option<f32>,
}
