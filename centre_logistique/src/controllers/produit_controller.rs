use crate::cache::PRODUITS_CACHE;
use crate::db::get_conn;
use crate::dto::ProduitUpdateDTO;
use crate::metrics::{HTTP_REQUESTS_TOTAL, HTTP_REQ_DURATION_SECONDS};
use crate::models::produit::Produit;
use crate::schema::produits::dsl::{
    description as p_description, nom as p_nom, prix as p_prix, produits,
};
use cached::Cached;
use diesel::prelude::*;
use prometheus::HistogramTimer;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, put};
use rocket_okapi::openapi;
use tracing::{error, info};

#[openapi]
#[get("/produits")]
pub async fn get_produits() -> Result<Json<Vec<Produit>>, String> {
    let mut conn = get_conn();

    produits
        .load::<Produit>(&mut conn)
        .map(|inv| Json(inv))
        .map_err(|e| format!("Erreur DB : {}", e))
}

#[openapi]
#[get("/produits/<id>")]
pub async fn get_produit(id: i32) -> Result<Json<Produit>, String> {
    let mut cache = PRODUITS_CACHE.lock().unwrap();
    if let Some(cached_produit) = cache.cache_get(&id) {
        return Ok(Json(cached_produit.clone()));
    }

    let mut conn = get_conn();
    let produit = produits
        .find(id)
        .first::<Produit>(&mut conn)
        .map_err(|e| format!("Erreur lecture produit : {}", e))?;

    cache.cache_set(id, produit.clone());

    Ok(Json(produit))
}

#[openapi]
#[put("/produits/<id>", data = "<data_update>")]
pub async fn put_produit(
    id: i32,
    data_update: Json<ProduitUpdateDTO>,
) -> Result<Json<Produit>, Status> {
    let timer: HistogramTimer = HTTP_REQ_DURATION_SECONDS.start_timer();

    let mut conn = get_conn();

    let resultat = diesel::update(produits.find(id))
        .set((
            p_nom.eq(&data_update.nom),
            p_prix.eq(data_update.prix),
            p_description.eq(&data_update.description),
        ))
        .get_result::<Produit>(&mut conn);

    match resultat {
        Ok(produit) => {
            let mut cache = PRODUITS_CACHE.lock().unwrap();
            cache.cache_remove(&id);

            HTTP_REQUESTS_TOTAL.with_label_values(&["200"]).inc();
            timer.observe_duration();

            info!("Mise à jour produit id : {}", produit.id_produit);
            Ok(Json(produit))
        }
        Err(e) => {
            error!("Erreur Mise à jour : {}", e);
            HTTP_REQUESTS_TOTAL.with_label_values(&["500"]).inc();
            timer.observe_duration();

            Err(Status::InternalServerError)
        }
    }
}
