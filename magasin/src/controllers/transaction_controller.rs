use rocket::serde::json::Json;
use crate::models::transaction::Transaction;
use crate::db::get_conn;
use diesel::prelude::*;
use rocket::get;
use crate::schema::transactions::dsl::*;

#[get("/transactions")]
pub async fn get_transactions() -> Result<Json<Vec<Transaction>>, String> {
    let mut conn = get_conn();

    transactions
        .load::<Transaction>(&mut conn)
        .map(Json)
        .map_err(|e| format!("Erreur DB: {}", e))
}
