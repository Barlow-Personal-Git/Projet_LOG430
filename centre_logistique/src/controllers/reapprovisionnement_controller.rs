use rocket::serde::json::Json;
use rocket::{get, post, put, http::Status};
use diesel::prelude::*;
use crate::db::get_conn;
use crate::dto::AlerteReapprovisionnementDTO;
use crate::models::produit::Produit;
use crate::models::magasin::Magasin;
use crate::models::reapprovisionnement::{Reapprovisionnement, NouveauReapprovisionnement};
use crate::schema::reapprovisionnements::dsl::{
    reapprovisionnements,
    id_reapprovisionnement,
    id_produit as ra_produit_id,
    id_magasin as ra_magasin_id,
    nbr as ra_nbr,
    status,
    created_date as ra_created_date,
};
use crate::schema::magasins::dsl::{magasins, nom, id_magasin};
use crate::schema::produits::dsl::{produits, id_produit, nom as nom_produit};

#[get("/reapprovisionnements")]
pub async fn get_reapprovisionnements() -> Result<Json<Vec<Reapprovisionnement>>, String> {
    let mut conn = get_conn();

    reapprovisionnements
        .load::<Reapprovisionnement>(&mut conn)
        .map(|inv| Json(inv))
        .map_err(|e| format!("Erreur DB : {}", e))
}

#[post("/reapprovisionnements", data = "<data>")]
pub async fn post_reapprovisionnements(data: Json<NouveauReapprovisionnement>) -> Result<(Status, Json<Reapprovisionnement>), String> {
    let mut conn = get_conn();

    let mut nouveau_data = data.into_inner();
    nouveau_data.status = "en_attente".to_string();

    diesel::insert_into(reapprovisionnements)
        .values(&nouveau_data)
        .get_result::<Reapprovisionnement>(&mut conn)
        .map(|reappro| (Status::Created, Json(reappro)))
        .map_err(|e| format!("Erreur insertion: {}", e))
}

#[put("/reapprovisionnements/<id>", data = "<data>")]
pub async fn put_reapprovisionnement(id: i32, data: Json<NouveauReapprovisionnement>) -> Result<Json<Reapprovisionnement>, String> {
    let mut conn = get_conn();
    let update_data = data.into_inner();

    diesel::update(reapprovisionnements.filter(id_reapprovisionnement.eq(id)))
        .set((
            status.eq(update_data.status),
        ))
        .get_result::<Reapprovisionnement>(&mut conn)
        .map(Json)
        .map_err(|e| format!("Erreur lors de la mise à jour : {}", e))
}


#[get("/alerte_reapprovisionnements")]
pub async fn get_alerte_reapprovisionnements() -> Result<Json<Vec<AlerteReapprovisionnementDTO>>, String> {
    let mut conn = get_conn();

    let resultats = reapprovisionnements
        .inner_join(magasins.on(id_magasin.eq(ra_magasin_id)))
        .inner_join(produits.on(id_produit.eq(ra_produit_id)))
        .select((
            nom, 
            nom_produit, 
            ra_nbr, 
            status, 
            ra_created_date
        ))
        .load::<(String, String, i32, String, chrono::NaiveDateTime)>(&mut conn)
        .map_err(|e| format!("Erreur DB : {}", e))?;
    
    let alerte_res: Vec<AlerteReapprovisionnementDTO> = resultats
        .into_iter()
        .map(|(nom_mag, nom_pro, nombre, ra_status, ra_date_creation)| AlerteReapprovisionnementDTO {
            magasin: nom_mag,
            produit: nom_pro,
            nbr: nombre,
            status: ra_status,
            date_creation: ra_date_creation,
        })
        .collect();
    Ok(Json(alerte_res))
}