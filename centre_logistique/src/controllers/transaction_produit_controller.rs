use rocket::serde::json::Json;
use rocket::get;
use rocket::post;
use diesel::prelude::*;
use diesel::dsl::sum;
use diesel::upsert::excluded;
use crate::db::get_conn;
use crate::models::{TransactionProduit, Magasin};
use crate::dto::TransactionProduitsDTO;
use crate::models::transaction_produit::{NouveauTransactionProduit, SommeTransactionProduitParMagasin};
use crate::schema::transaction_produits::dsl::{
    transaction_produits,
    id_transaction as trp_magasin_id_transaction,
    id_magasin as trp_magasin_id,
    produits as trp_produits,
    total as trp_total,
};
use crate::schema::magasins::dsl::{magasins, nom, id_magasin};

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
        .on_conflict((trp_magasin_id, trp_magasin_id_transaction))
        .do_update()
        .set((
            trp_produits.eq(excluded(trp_produits)),
            trp_total.eq(excluded(trp_total))
        ))
        .execute(&mut conn)
        .map_err(|e| format!("Erreur insertion: {}", e))?;

    Ok("Inventaire insérée".to_string())
}

#[get("/ventes_magasin")]
pub async fn get_ventes_magasin() -> Result<Json<Vec<SommeTransactionProduitParMagasin>>, String> {
    let mut conn = get_conn();

    let resultats = transaction_produits
        .inner_join(magasins.on(id_magasin.eq(trp_magasin_id)))
        .group_by(nom)
        .select((nom, sum(trp_total)))
        .load::<(String, Option<f32>)>(&mut conn)
        .map_err(|e| format!("Erreur DB : {}", e))?;

    let sommes: Vec<SommeTransactionProduitParMagasin> = resultats
        .into_iter()
        .map(|(magasin_nom, magasin_total)| SommeTransactionProduitParMagasin {
            magasin : magasin_nom,
            total: magasin_total.unwrap_or(0.0),
    })
    .collect();

    Ok(Json(sommes))
}
