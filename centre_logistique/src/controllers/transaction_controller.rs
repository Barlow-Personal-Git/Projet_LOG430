use rocket::serde::json::Json;
use rocket::{get, post};
use rocket_okapi::openapi;
use diesel::sql_query;
use diesel::prelude::*;
use diesel::upsert::excluded;
use crate::db::get_conn;
use crate::models::{Transaction, Magasin};
use crate::dto::{TransactionDTO, TendancesHebdoDTO};
use crate::models::transaction::{NouvelleTransaction, TendancesHebdoSQL};
use crate::schema::transactions::dsl::{
    transactions,
    id_magasin as trans_id_magasin,
    total as trans_total,
    created_date as trans_created_date
};
use crate::schema::magasins::dsl::{magasins, nom};
use crate::metrics::{HTTP_REQUESTS_TOTAL, HTTP_REQ_DURATION_SECONDS};
use prometheus::HistogramTimer;
use tracing::{info, error};
use rocket::http::Status;

#[openapi]
#[get("/transactions")]
pub async fn get_transactions() -> Result<Json<Vec<Transaction>>, String> {
    let mut conn = get_conn();

    transactions
        .load::<Transaction>(&mut conn)
        .map(|inv| Json(inv))
        .map_err(|e| format!("Erreur DB : {}", e))
}

#[openapi]
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

#[openapi]
#[get("/tendances_hebdomadaires")]
pub async fn get_tendances_hebdomadaires() -> Result<Json<Vec<TendancesHebdoDTO>>, Status> {
    
    let timer: HistogramTimer = HTTP_REQ_DURATION_SECONDS.start_timer();

    let mut conn = get_conn();

    let query = "
        SELECT magasins.nom, TO_CHAR(date_trunc('week', created_date), 'YYYY-MM-DD') AS semaine, SUM(transactions.total) AS total
        FROM transactions
        JOIN magasins ON transactions.id_magasin = magasins.id_magasin
        GROUP BY magasins.nom, semaine
        ORDER BY magasins.nom, semaine
    ";
    let resultats = sql_query(query)
        .load::<TendancesHebdoSQL>(&mut conn)
        .map_err(|e| {
            error!("Erreur DB lors de la récupération des magasins : {}", e);
            HTTP_REQUESTS_TOTAL.with_label_values(&["500"]).inc();
            Status::InternalServerError
        })?;

    let tendances: Vec<TendancesHebdoDTO> = resultats
        .into_iter()
        .map(|res_query| TendancesHebdoDTO {
            magasin: res_query.nom,
            semaine: res_query.semaine,
            total: res_query.total.unwrap_or(0.0),
    })
    .collect();
    info!("Générer la tendance hebdomadaires {:?}", tendances);

    HTTP_REQUESTS_TOTAL.with_label_values(&["200"]).inc();
    timer.observe_duration();
    Ok(Json(tendances))
}
