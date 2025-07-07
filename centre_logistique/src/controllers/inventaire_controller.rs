use rocket::serde::json::Json;
use rocket::get;
use rocket::post;
use diesel::prelude::*;
use diesel::upsert::excluded;
use crate::db::get_conn;
use crate::models::{Inventaire, Magasin};
use crate::dto::InventaireDTO;
use crate::models::inventaire::NouveauInventaire;
use crate::schema::inventaires::dsl::{
    inventaires,
    id_produit as inv_id_produit,
    id_magasin as inv_id_magasin,
    category as inv_category,
    nbr as inv_nbr,
};
use crate::schema::magasins::dsl::{magasins, nom};


#[get("/inventaires")]
pub async fn get_inventaires() -> Result<Json<Vec<Inventaire>>, String> {
    let mut conn = get_conn();

    inventaires
        .load::<Inventaire>(&mut conn)
        .map(|inv| Json(inv))
        .map_err(|e| format!("Erreur DB : {}", e))
}

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