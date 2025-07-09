use rocket::serde::json::Json;
use rocket::get;
use rocket::post;
use diesel::prelude::*;
use diesel::upsert::excluded;
use crate::db::get_conn;
use crate::models::{Inventaire, Magasin};
use crate::dto::{InventaireDTO, InventairesFaibleDTO, InventairesSurplusDTO};
use crate::models::inventaire::NouveauInventaire;
use crate::schema::inventaires::dsl::{
    inventaires,
    id_produit as inv_id_produit,
    id_magasin as inv_id_magasin,
    nbr as inv_nbr,
};
use crate::schema::magasins::dsl::{magasins, nom, id_magasin};
use crate::schema::produits::dsl::{produits, nom as produit_nom, id_produit};


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
