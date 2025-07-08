use rocket::serde::json::Json;
use rocket::get;
use rocket::post;
use diesel::dsl::sum;
use diesel::prelude::*;
use diesel::upsert::excluded;
use crate::db::get_conn;
use crate::models::{Transaction, Magasin};
use crate::dto::TransactionDTO;
use crate::models::transaction::{NouvelleTransaction, SommeTransactionParMagasin};
use crate::schema::transactions::dsl::{
    transactions,
    id_magasin as trans_id_magasin,
    total as trans_total,
    created_date as trans_created_date
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

    diesel::insert_into(transactions)
        .values(&new_tr)
        .on_conflict((trans_id_magasin, trans_created_date))
        .do_update()
        .set(
            trans_total.eq(excluded(trans_total))
        )
        .execute(&mut conn)
        .map_err(|e| format!("Erreur insertion transaction_produits: {}", e))?;

    Ok("Transaction insérée".to_string())
}

#[get("/sommes")]
pub async fn get_sommes() -> Result<Json<Vec<SommeTransactionParMagasin>>, String> {
  let mut conn = get_conn();
  
  let resultats = transactions
    .group_by(trans_id_magasin)
    .select((trans_id_magasin, sum(trans_total)))
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