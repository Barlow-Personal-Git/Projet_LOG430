use rocket::serde::json::Json;
use rocket::{get, post, put};
use rocket_okapi::openapi;
use diesel::prelude::*;
use diesel::upsert::excluded;
use diesel::sql_types::Integer;
use diesel::dsl::{sum, sql};
use crate::db::get_conn;
use crate::models::{Inventaire, Magasin};
use crate::dto::{InventaireDTO, InventairesFaibleDTO, InventairesSurplusDTO, InventaireRestantDTO, InventaireUpdateDTO};
use crate::models::inventaire::NouveauInventaire;
use crate::schema::inventaires::dsl::{
    inventaires,
    id_produit as inv_id_produit,
    id_magasin as inv_id_magasin,
    nbr as inv_nbr,
    id_inventaire,
};
use crate::schema::magasins::dsl::{magasins, nom, id_magasin};
use crate::schema::produits::dsl::{produits, nom as produit_nom, id_produit};
use crate::metrics::{HTTP_REQUESTS_TOTAL, HTTP_REQ_DURATION_SECONDS};
use prometheus::HistogramTimer;
use crate::cache::{INVENTAIRES_CACHE,INVENTAIRES_STRING_CACHE};
use cached::Cached;
use rocket::http::Status;
use tracing::error;

#[openapi]
#[get("/inventaires")]
pub async fn get_inventaires() -> Result<Json<Vec<Inventaire>>, String> {
    let mut conn = get_conn();

    inventaires
        .load::<Inventaire>(&mut conn)
        .map(|inv| Json(inv))
        .map_err(|e| format!("Erreur DB : {}", e))
}

#[openapi]
#[get("/inventaires/<id>")]
pub async fn get_inventaires_id(id: i32) -> Result<Json<Inventaire>, Status> {
    
    let mut cache = INVENTAIRES_CACHE.lock().unwrap();

    if let Some(cached_inv) = cache.cache_get(&id) {
        return Ok(Json(cached_inv.clone()));
    }

    let mut conn = get_conn();

    match inventaires.filter(id_inventaire.eq(id)).first::<Inventaire>(&mut conn) {
        Ok(inventaire) => {
            cache.cache_set(id, inventaire.clone());
            HTTP_REQUESTS_TOTAL.with_label_values(&["200"]).inc();
            Ok(Json(inventaire))
        }
        Err(diesel::result::Error::NotFound) => {
            HTTP_REQUESTS_TOTAL.with_label_values(&["404"]).inc();
            Err(Status::NotFound)
        }
        Err(_) => {
            HTTP_REQUESTS_TOTAL.with_label_values(&["500"]).inc();
            Err(Status::InternalServerError)
        }
    }
}

#[openapi]
#[get("/inventaires?<id_magasins>")]
pub async fn get_inventaires_par_magasins(id_magasins: Option<&str>) -> Result<Json<Vec<Inventaire>>, Status> {
    
    let mut cache = INVENTAIRES_STRING_CACHE.lock().unwrap();
    let timer: HistogramTimer = HTTP_REQ_DURATION_SECONDS.start_timer();

    let mut conn = get_conn();

    let cache_key = id_magasins.unwrap_or("").to_string();

    if let Some(cached_inventaires) = cache.cache_get(&cache_key) {
        HTTP_REQUESTS_TOTAL.with_label_values(&["200"]).inc();
        return Ok(Json(cached_inventaires.clone()));
    }

    let magasin_ids: Vec<i32> = if cache_key.is_empty() {
        vec![]
    } else {
        cache_key
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect()
    };

    let query = if magasin_ids.is_empty() {
        inventaires.into_boxed()
    } else {
        inventaires.into_boxed().filter(inv_id_magasin.eq_any(magasin_ids))
    };

    let resultats = match query.load::<Inventaire>(&mut conn) {
        Ok(res) => {
            HTTP_REQUESTS_TOTAL.with_label_values(&["200"]).inc();
            res
        }
        Err(e) => {
            HTTP_REQUESTS_TOTAL.with_label_values(&["500"]).inc();
            timer.observe_duration();
            error!("Erreur DB : {}", e);
            return Err(Status::InternalServerError);
        }
    };

    cache.cache_set(cache_key, resultats.clone());
    timer.observe_duration();

    Ok(Json(resultats))
}

#[openapi]
#[put("/inventaires", data = "<data>")]
pub async fn put_inventaire(data: Json<InventaireUpdateDTO>) -> Result<String, String> {
    let mut conn = get_conn();
    let update_data = data.into_inner();

    diesel::update(
        inventaires
            .filter(inv_id_produit.eq(update_data.id_produit))
            .filter(inv_id_magasin.eq(update_data.id_magasin))
    )
    .set(inv_nbr.eq(sql::<Integer>(&format!("nbr + {}", update_data.nbr))))
    .execute(&mut conn)
    .map_err(|e| format!("Erreur lors de la mise à jour : {}", e))?;

    Ok("Quantité mise à jour.".to_string())
}

#[openapi]
#[post("/inventaires", data = "<data>")]
pub async fn post_inventaires(data: Json<InventaireDTO<'_>>) -> Result<String, String> {
    let mut conn = get_conn();

    let magasin_record = magasins
        .filter(nom.eq(&data.magasin))
        .first::<Magasin>(&mut conn)
        .map_err(|e| format!("Magasin inconnu : {}", e))?;
    
    let new_inv: Vec<NouveauInventaire> = data.inventaires.iter()
        .map(|inv| NouveauInventaire {
            id_produit: inv.id_produit,
            id_magasin: magasin_record.id_magasin,
            category: &inv.category,
            nbr: inv.nbr,
    }).collect();

    diesel::insert_into(inventaires)
        .values(&new_inv)
        .on_conflict((inv_id_produit, inv_id_magasin))
        .do_update()
        .set((
            inv_nbr.eq(excluded(inv_nbr)),
        ))
        .execute(&mut conn)
        .map_err(|e| format!("Erreur insertion: {}", e))?;

    Ok("Inventaire insérée".to_string())
}

#[openapi]
#[get("/inventaires_faible")]
pub async fn get_inventaires_faible() -> Result<Json<Vec<InventairesFaibleDTO>>, String> {
    let mut conn = get_conn();
    let faible_stock = 10;

    let resultats = inventaires
        .inner_join(magasins.on(id_magasin.eq(inv_id_magasin)))
        .inner_join(produits.on(id_produit.eq(inv_id_produit)))
        .filter(inv_nbr.le(faible_stock))
        .select((nom, produit_nom, inv_nbr))
        .order(nom.asc())
        .load::<(String, String, i32)>(&mut conn)
        .map_err(|e| format!("Erreur DB : {}", e))?;
    
    let inv_faible: Vec<InventairesFaibleDTO> = resultats
        .into_iter()
        .map(|(nom_value, produit_nom_value, inv_nbr_value)| InventairesFaibleDTO {
            nom: nom_value,
            produit_nom: produit_nom_value,
            inv_nbr: inv_nbr_value
    })
    .collect();

    Ok(Json(inv_faible))
}

#[openapi]
#[get("/inventaires_surplus")]
pub async fn get_inventaires_surplus() -> Result<Json<Vec<InventairesSurplusDTO>>, String> {
    let mut conn = get_conn();
    let surplus_stock = 30;

    let resultats = inventaires
        .inner_join(magasins.on(id_magasin.eq(inv_id_magasin)))
        .inner_join(produits.on(id_produit.eq(inv_id_produit)))
        .filter(inv_nbr.ge(surplus_stock))
        .select((nom, produit_nom, inv_nbr))
        .order(nom.asc())
        .load::<(String, String, i32)>(&mut conn)
        .map_err(|e| format!("Erreur DB : {}", e))?;
    
    let inv_surplus: Vec<InventairesSurplusDTO> = resultats
        .into_iter()
        .map(|(nom_value, produit_nom_value, inv_nbr_value)| InventairesSurplusDTO {
            nom: nom_value,
            produit_nom: produit_nom_value,
            inv_nbr: inv_nbr_value
        })
        .collect();

    Ok(Json(inv_surplus))
}

#[openapi]
#[get("/inventaires_restants")]
pub async fn get_inventaires_restants() -> Result<Json<Vec<InventaireRestantDTO>>, String> {
    let mut conn = get_conn();

    let resultats = inventaires
        .inner_join(produits.on(id_produit.eq(inv_id_produit)))
        .group_by(produit_nom)
        .select((produit_nom, sum(inv_nbr)))
        .load::<(String, Option<i64>)>(&mut conn)
        .map_err(|e| format!("Erreur DB : {}", e))?;
    
    let inv_restant: Vec<InventaireRestantDTO> = resultats
        .into_iter()
        .map(|(pro_nom, somme)| InventaireRestantDTO {
            nom_produit: pro_nom,
            nbr_inventaire: somme.unwrap_or(0) as i32,
        })
        .collect();

    Ok(Json(inv_restant))
}
