use crate::db::get_conn;
use crate::dto::AlerteReapprovisionnementDTO;
use crate::models::reapprovisionnement::{NouveauReapprovisionnement, Reapprovisionnement};
use crate::schema::magasins::dsl::{id_magasin, magasins, nom};
use crate::schema::produits::dsl::{id_produit, nom as nom_produit, produits};
use crate::schema::reapprovisionnements::dsl::{
    created_date as ra_created_date, id_magasin as ra_magasin_id, id_produit as ra_produit_id,
    id_reapprovisionnement, nbr as ra_nbr, reapprovisionnements, status,
};
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{get, http::Status, post, put};
use rocket_okapi::openapi;

#[openapi]
#[get("/reapprovisionnements")]
pub async fn get_reapprovisionnements() -> Result<Json<Vec<Reapprovisionnement>>, String> {
    let mut conn = get_conn();

    reapprovisionnements
        .load::<Reapprovisionnement>(&mut conn)
        .map(|inv| Json(inv))
        .map_err(|e| format!("Erreur DB : {}", e))
}

#[openapi]
#[post("/reapprovisionnements", data = "<data>")]
pub async fn post_reapprovisionnements(
    data: Json<NouveauReapprovisionnement>,
) -> Result<(Status, Json<Reapprovisionnement>), String> {
    let mut conn = get_conn();

    let mut nouveau_data = data.into_inner();
    nouveau_data.status = "en_attente".to_string();

    diesel::insert_into(reapprovisionnements)
        .values(&nouveau_data)
        .get_result::<Reapprovisionnement>(&mut conn)
        .map(|reappro| (Status::Created, Json(reappro)))
        .map_err(|e| format!("Erreur insertion: {}", e))
}

#[openapi]
#[put("/reapprovisionnements/<id>", data = "<data>")]
pub async fn put_reapprovisionnement(
    id: i32,
    data: Json<NouveauReapprovisionnement>,
) -> Result<Json<Reapprovisionnement>, String> {
    let mut conn = get_conn();
    let update_data = data.into_inner();

    diesel::update(reapprovisionnements.filter(id_reapprovisionnement.eq(id)))
        .set((status.eq(update_data.status),))
        .get_result::<Reapprovisionnement>(&mut conn)
        .map(Json)
        .map_err(|e| format!("Erreur lors de la mise à jour : {}", e))
}

#[openapi]
#[get("/alerte_reapprovisionnements")]
pub async fn get_alerte_reapprovisionnements(
) -> Result<Json<Vec<AlerteReapprovisionnementDTO>>, String> {
    let mut conn = get_conn();

    let resultats = reapprovisionnements
        .inner_join(magasins.on(id_magasin.eq(ra_magasin_id)))
        .inner_join(produits.on(id_produit.eq(ra_produit_id)))
        .select((nom, nom_produit, ra_nbr, status, ra_created_date))
        .load::<(String, String, i32, String, chrono::NaiveDateTime)>(&mut conn)
        .map_err(|e| format!("Erreur DB : {}", e))?;

    let alerte_res: Vec<AlerteReapprovisionnementDTO> = resultats
        .into_iter()
        .map(|(nom_mag, nom_pro, nombre, ra_status, ra_date_creation)| {
            AlerteReapprovisionnementDTO {
                magasin: nom_mag,
                produit: nom_pro,
                nbr: nombre,
                status: ra_status,
                date_creation: ra_date_creation,
            }
        })
        .collect();
    Ok(Json(alerte_res))
}
