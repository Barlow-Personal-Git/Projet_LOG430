use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Associations};
use rocket::serde::{Serialize, Deserialize};
use crate::models::client::Client;
use crate::schema::transactions;

#[derive(Debug, Clone, Queryable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Client, foreign_key = id_client))]
#[serde(crate = "rocket::serde")]
pub struct Transaction {
    pub id_transaction: i32,
    pub id_client: i32,
    pub total: f32,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct NouvelleTransaction {
    pub id_client: i32,
    pub total: f32,
    
    #[diesel(sql_type = Timestamp)]
    pub created_date: NaiveDateTime,

    #[diesel(sql_type = Timestamp)]
    pub updated_date: NaiveDateTime,
}
