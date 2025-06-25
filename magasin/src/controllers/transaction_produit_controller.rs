use rocket::serde::json::Json;
use crate::models::transaction_produit::TransactionProduit;
use crate::db::get_conn;
use diesel::prelude::*;
use rocket::get;
use crate::schema::transaction_produits::dsl::*;

#[get("/transaction_produits")]
pub async fn get_transaction_produits() -> Result<Json<Vec<TransactionProduit>>, String> {
    let mut conn = get_conn();
    
    transaction_produits
        .load::<TransactionProduit>(&mut conn)
        .map(Json)
        .map_err(|e| format!("Erreur DB: {}", e))
}
