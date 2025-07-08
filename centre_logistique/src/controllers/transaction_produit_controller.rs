use rocket::serde::json::Json;
use rocket::get;
use rocket::post;
use diesel::prelude::*;
use diesel::upsert::excluded;
use crate::db::get_conn;
use crate::models::{TransactionProduit, Magasin};
use crate::dto::TransactionProduitsDTO;
use crate::models::transaction_produit::NouveauTransactionProduit;
use crate::schema::transaction_produits::dsl::{
    transaction_produits,
    id_transaction as trp_magasin_id_transaction,
    id_magasin as trp_magasin,
    produits as trp_produits,
    total as trp_total,
};
use crate::schema::magasins::dsl::{magasins, nom};

#[get("/transaction_produits")]
pub async fn get_transaction_produits() -> Result<Json<Vec<TransactionProduit>>, String> {
    let mut conn = get_conn();

    transaction_produits
        .load::<TransactionProduit>(&mut conn)
        .map(|inv| Json(inv))
        .map_err(|e| format!("Erreur DB : {}", e))
}

#[post("/transaction_produits", data = "<data>")]
pub async fn post_transaction_produits(data: Json<TransactionProduitsDTO>) -> Result<String, String> {
    let mut conn = get_conn();

    let magasin_record = magasins
        .filter(nom.eq(&data.magasin))
        .first::<Magasin>(&mut conn)
        .map_err(|e| format!("Magasin inconnu : {}", e))?;
    
    let new_trp: Vec<NouveauTransactionProduit> = data.transaction_produits.iter()
        .map(|trans_produits| NouveauTransactionProduit {
            id_magasin: magasin_record.id_magasin,
            id_transaction: trans_produits.id_transaction,
            produits: serde_json::to_value(&trans_produits.produits).unwrap(),
            total: trans_produits.total
    }).collect();

    diesel::insert_into(transaction_produits)
        .values(&new_trp)
        .on_conflict((trp_magasin, trp_magasin_id_transaction))
        .do_update()
        .set((
            trp_produits.eq(excluded(trp_produits)),
            trp_total.eq(excluded(trp_total))
        ))
        .execute(&mut conn)
        .map_err(|e| format!("Erreur insertion: {}", e))?;

    Ok("Inventaire insérée".to_string())
}