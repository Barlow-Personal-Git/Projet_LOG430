use rocket::serde::json::Json;
use rocket::get;
use rocket::post;
use diesel::dsl::sum;
use diesel::prelude::*;
use diesel::upsert::excluded;
use crate::db::get_conn;
use crate::models::{Transaction, TransactionProduit, Magasin};
use crate::dto::TransactionDTO;
use crate::models::transaction::{NouvelleTransaction, SommeTransactionParMagasin};
use crate::models::transaction_produit::NouveauTransactionProduit;
use crate::schema::transactions::dsl::{
    transactions,
    id_magasin as trans_id_magasin,
    total,
    updated_date,
};
use crate::schema::transaction_produits::dsl::{
    transaction_produits,
};
use crate::schema::magasins::dsl::{magasins, nom};


#[get("/transactions")]
pub async fn get_transactions() -> Result<Json<Vec<Transaction>>, String> {
    let mut conn = get_conn();

    transactions
        .load::<Transaction>(&mut conn)
        .map(|inv| Json(inv))
        .map_err(|e| format!("Erreur DB : {}", e))
}

#[post("/transactions", data = "<data>")]
pub async fn post_transaction(data: Json<TransactionDTO<'_>>) -> Result<String, String> {
    let mut conn = get_conn();

    let magasin_record = magasins
        .filter(nom.eq(&data.magasin))
        .first::<Magasin>(&mut conn)
        .map_err(|e| format!("Magasin inconnu : {}", e))?;
    
    let new_tr: Vec<NouvelleTransaction> = data.transactions.iter()
        .map(|tr| NouvelleTransaction {
            id_magasin: magasin_record.id_magasin,
            total: tr.total,
            created_date: tr.created_date,
            updated_date: tr.updated_date
    }).collect();

    let inserted_transactions: Vec<Transaction> = diesel::insert_into(transactions)
        .values(&new_tr)
        .on_conflict(trans_id_magasin)
        .do_update()
        .set((
            total.eq(excluded(total)),
            updated_date.eq(excluded(updated_date)),
        ))
        .get_results(&mut conn)
        .map_err(|e| format!("Erreur insertion: {}", e))?;
    
    let mut new_trp = Vec::new();
    for (idx, trp) in data.transaction_produits.iter().enumerate() {
        new_trp.push(NouveauTransactionProduit {
            id_transaction: inserted_transactions[idx].id_transaction,
            id_produit: trp.id_produit,
            nbr: trp.nbr,
            total: trp.total,
        });
    }

    diesel::insert_into(transaction_produits)
        .values(&new_trp)
        .execute(&mut conn)
        .map_err(|e| format!("Erreur insertion transaction_produits: {}", e))?;

    Ok("Transaction insérée".to_string())
}

#[get("/transaction_produits")]
pub async fn get_transaction_produits() -> Result<Json<Vec<TransactionProduit>>, String> {
    let mut conn = get_conn();

    transaction_produits
        .load::<TransactionProduit>(&mut conn)
        .map(|inv| Json(inv))
        .map_err(|e| format!("Erreur DB : {}", e))
}

#[get("/sommes")]
pub async fn get_sommes() -> Result<Json<Vec<SommeTransactionParMagasin>>, String> {
  let mut conn = get_conn();
  
  let resultats = transactions
    .group_by(trans_id_magasin)
    .select((trans_id_magasin, sum(total)))
    .load::<(i32, Option<f32>)>(&mut conn)
    .map_err(|e| format!("Erreur DB : {}", e))?;

  let sommes: Vec<SommeTransactionParMagasin> = resultats
    .into_iter()
    .map(|(id_magasin, total_opt)| SommeTransactionParMagasin {
        id_magasin,
        total: total_opt.unwrap_or(0.0),
    })
    .collect();

    Ok(Json(sommes))
}