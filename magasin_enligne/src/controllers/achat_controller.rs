use crate::db::get_conn;
use crate::dto::TransactionDTO;
use crate::models::produit::Produit;
use crate::models::transaction::NouvelleTransaction;
use crate::schema::produits::dsl::{id_produit, produits};
use crate::session::client_session::CLIENT_SESSION;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::{delete, get, post};
use rocket_dyn_templates::Template;
use rocket_okapi::openapi;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct PanierItem {
    nom: String,
    prix: String,
    nbr: u32,
    total: String,
}

#[derive(Serialize)]
struct PanierContext {
    produits: Vec<PanierItem>,
    message: Option<String>,
}

#[get("/panier")]
pub fn get_panier() -> Template {
    let session = CLIENT_SESSION.lock().unwrap();
    let produits_vente = session.get_produits();

    let produits_affichage: Vec<PanierItem> = produits_vente
        .into_iter()
        .map(|(produit, nbr)| PanierItem {
            nom: produit.nom,
            prix: format!("{:.2}", produit.prix),
            nbr,
            total: format!("{:.2}", produit.prix * nbr as f32),
        })
        .collect();

    let message = if produits_affichage.is_empty() {
        Some("Votre panier est vide.".to_string())
    } else {
        None
    };

    let context = PanierContext {
        produits: produits_affichage,
        message,
    };

    Template::render("panier", &context)
}

#[openapi]
#[post("/panier/vider")]
pub fn delete_panier() -> Redirect {
    let mut session = CLIENT_SESSION.lock().unwrap();
    session.clear_vente();
    Redirect::to("/panier")
}

#[openapi]
#[post("/panier/ajouter", format = "json", data = "<data>")]
pub fn post_panier(data: Json<TransactionDTO>) -> Status {
    let mut conn = get_conn();
    let produit_id = data.id_produit;
    let nbr = data.nbr;

    let produit_result = produits
        .filter(id_produit.eq(produit_id))
        .first::<Produit>(&mut conn);

    match produit_result {
        Ok(produit) => {
            let mut session = CLIENT_SESSION.lock().unwrap();
            session.add_produit(produit, nbr);
            Status::Ok
        }
        Err(_) => Status::NotFound,
    }
}

#[get("/panier/ajouter")]
pub fn get_ajouter_panier() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("ajouter_panier", &context)
}

#[post("/panier/achat/confirmer")]
pub fn confirmer_achat() -> Result<Json<String>, Status> {
    let mut session = CLIENT_SESSION.lock().unwrap();

    let client = session.get_client().ok_or(Status::Unauthorized)?;
    let produits_vente = session.get_produits();

    if produits_vente.is_empty() {
        return Err(Status::BadRequest);
    }

    let mut conn = get_conn();

    let total_montant: f32 = produits_vente
        .iter()
        .map(|(produit, nbr)| produit.prix * (*nbr as f32))
        .sum();

    let now = chrono::Utc::now().naive_utc();

    let nouvelle_transaction = NouvelleTransaction {
        id_client: client.id_client,
        total: total_montant,
        created_date: now,
        updated_date: now,
    };

    use crate::schema::transactions::dsl::*;

    diesel::insert_into(transactions)
        .values(&nouvelle_transaction)
        .execute(&mut conn)
        .map_err(|_| Status::InternalServerError)?;

    session.clear_vente();

    Ok(Json("Commande validée avec succès.".to_string()))
}
