use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Associations};
use rocket::serde::{Serialize, Deserialize};
use crate::models::magasin::Magasin;
use crate::schema::transactions;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize, Clone)]
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
