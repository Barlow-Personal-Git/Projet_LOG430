use rocket::serde::json::Json;
use crate::models::inventaire::Inventaire;
use crate::db::get_conn;
use diesel::prelude::*;
use rocket::get;
use crate::schema::inventaires::dsl::*;

#[get("/inventaires")]
pub fn get_inventaires() -> Result<Json<Vec<Inventaire>>, String> {
    let mut conn = get_conn();
    
    inventaires
        .load::<Inventaire>(&mut conn)
        .map(Json)
        .map_err(|e| format!("Erreur DB: {}", e))
}
