use rocket::serde::json::Json;
use rocket::{get, post, put, http::Status};
use diesel::prelude::*;
use crate::db::get_conn;
use crate::models::reapprovisionnement::{Reapprovisionnement, NouveauReapprovisionnement};
use crate::schema::reapprovisionnements::dsl::*;

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
