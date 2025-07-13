use rocket::serde::json::{Json, Value};
use rocket_okapi::openapi;
use rocket::get;
use rocket::post;
use std::collections::HashMap;
use diesel::prelude::*;
use diesel::dsl::sum;
use diesel::upsert::excluded;
use crate::db::get_conn;
use crate::models::{TransactionProduit, Magasin};
use crate::dto::{TransactionProduitsDTO, ProduitVenduDTO};
use crate::models::transaction_produit::{NouveauTransactionProduit, SommeTransactionProduitParMagasin};
use crate::schema::transaction_produits::dsl::{
    transaction_produits,
    id_transaction as trp_magasin_id_transaction,
    id_magasin as trp_magasin_id,
    produits as trp_produits,
    total as trp_total,
};
use crate::schema::magasins::dsl::{magasins, nom, id_magasin};
use tracing::{info, error};

#[openapi]
#[get("/transaction_produits")]
pub async fn get_transaction_produits() -> Result<Json<Vec<TransactionProduit>>, String> {
    let mut conn = get_conn();

    transaction_produits
        .load::<TransactionProduit>(&mut conn)
        .map(|inv| {
            info!("Récupération réussie, {} transactions", inv.len());
            Json(inv)    
        })
        .map_err(|e| {
            error!("Erreur DB lors de la récupération des transactions produits : {}", e);
            format!("Erreur DB : {}", e)
        })
}

#[openapi]
#[post("/transaction_produits", data = "<data>")]
pub async fn post_transaction_produits(data: Json<TransactionProduitsDTO>) -> Result<String, String> {
    let mut conn = get_conn();

    let magasin_record = magasins
        .filter(nom.eq(&data.magasin))
        .first::<Magasin>(&mut conn)
        .map_err(|e| {
            error!("Erreur DB lors de la récupération des magasins : {}", e);
            format!("Erreur DB : {}", e)
        })?;
    
    info!("Insertion des produits pour id_magasin {}", magasin_record.id_magasin);

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
        .map_err(|e| {
            error!("Erreur Insertion des transaction_produits : {}", e);
            format!("Erreur Insertion : {}", e)
        })?;

    info!("Insertion pour le magasin {}", magasin_record.id_magasin);

    Ok("Inventaire insérée".to_string())
}

#[openapi]
#[get("/ventes_magasin")]
pub async fn get_ventes_magasin() -> Result<Json<Vec<SommeTransactionProduitParMagasin>>, String> {
    let mut conn = get_conn();

    let resultats = transaction_produits
        .inner_join(magasins.on(id_magasin.eq(trp_magasin_id)))
        .group_by(nom)
        .select((nom, sum(trp_total)))
        .load::<(String, Option<f32>)>(&mut conn)
        .map_err(|e| {
            error!("Erreur DB lors de la récupération des ventes magasins : {}", e);
            format!("Erreur DB : {}", e)
        })?;

    let sommes: Vec<SommeTransactionProduitParMagasin> = resultats
        .into_iter()
        .map(|(magasin_nom, magasin_total)| SommeTransactionProduitParMagasin {
            magasin : magasin_nom,
            total: magasin_total.unwrap_or(0.0),
    })
    .collect();
    info!("Récupération réussie, {} transactions produits", sommes.len());

    Ok(Json(sommes))
}

#[openapi]
#[get("/produits_vendus")]
pub async fn get_produits_vendus() -> Result<Json<Vec<ProduitVenduDTO>>, String> {
    let mut conn = get_conn();

    let resultats = transaction_produits
        .select(trp_produits)
        .load::<Value>(&mut conn)
        .map_err(|e| {
            error!("Erreur DB lors de la récupération des produits vendus : {}", e);
            format!("Erreur DB : {}", e)
        })?;
    
    let mut compteur: HashMap<String, i32> = HashMap::new();

    for produit_json in resultats.iter() {
        if let Value::Array(items) = produit_json {
            for item in items {
                if let Some(nom_val) = item.get("nom") {
                    if let Some(nbr_val) = item.get("nbr") {
                        if let (Some(nom_str), Some(nbr_i64)) = (nom_val.as_str(), nbr_val.as_i64()) {
                            *compteur.entry(nom_str.to_string()).or_insert(0) += nbr_i64 as i32;
                        }
                    }
                }
            }
        }
    }

    let mut produits_vendus: Vec<ProduitVenduDTO> = compteur.into_iter()
        .map(|(nom_produit, nbr_vendue)| ProduitVenduDTO { nom_produit, nbr_vendue })
        .collect();

    produits_vendus.sort_by(|a, b| b.nbr_vendue.cmp(&a.nbr_vendue));
    info!("Récupération réussie, {} produits vendus", produits_vendus.len());

    Ok(Json(produits_vendus))
}
