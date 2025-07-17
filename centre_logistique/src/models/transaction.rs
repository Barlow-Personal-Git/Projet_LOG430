use crate::models::magasin::Magasin;
use crate::schema::transactions;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::{Float, Nullable, Text};
use diesel::{Associations, Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize, Clone, JsonSchema)]
#[diesel(belongs_to(Magasin, foreign_key = id_magasin))]
#[serde(crate = "rocket::serde")]
pub struct Transaction {
    #[schemars(example = "crate::examples::exemple_id_transaction")]
    pub id_transaction: i32,
    #[schemars(example = "crate::examples::exemple_id_magasin")]
    pub id_magasin: i32,
    #[schemars(example = "crate::examples::exemple_total")]
    pub total: f32,
    #[schemars(example = "crate::examples::exemple_created_date")]
    pub created_date: NaiveDateTime,
    #[schemars(example = "crate::examples::exemple_updated_date")]
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
