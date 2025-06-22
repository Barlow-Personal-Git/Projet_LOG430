use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Associations};
use crate::models::client::Client;
use crate::schema::transactions;

#[derive(Debug, Queryable, Associations)]
#[diesel(belongs_to(Client, foreign_key = id_client))]
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
}
