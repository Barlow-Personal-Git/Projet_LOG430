use chrono::NaiveDateTime;
use genpdf::{elements, Document};
use reqwest::Client;
use rocket::get;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::Cursor;

#[derive(Deserialize, Serialize)]
struct VenteParMagasin {
    magasin: String,
    total: f64,
}

#[derive(Deserialize, Serialize)]
struct AlerteReapprovisionnement {
    magasin: String,
    produit: String,
    nbr: i32,
    status: String,
    date_creation: NaiveDateTime,
}

#[derive(Deserialize, Serialize)]
struct InventairesSurplus {
    nom: String,
    produit_nom: String,
    inv_nbr: i32,
}

#[derive(Deserialize, Serialize)]
struct TendancesHebdo {
    magasin: String,
    semaine: String,
    total: f32,
}

#[derive(Serialize)]
pub struct Context {
    ventes: Vec<VenteParMagasin>,
    alertes: Vec<AlerteReapprovisionnement>,
    surplus: Vec<InventairesSurplus>,
    tendances: Vec<TendancesHebdo>,
}

async fn fetch_ventes(base_url: &str) -> Result<Vec<VenteParMagasin>, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/ventes_magasin", base_url);
    let resp = client.get(&url).send().await?;
    resp.json::<Vec<VenteParMagasin>>().await
}

async fn fetch_alerte_reapprovisionnements(
    base_url: &str,
) -> Result<Vec<AlerteReapprovisionnement>, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/alerte_reapprovisionnements", base_url);
    let resp = client.get(&url).send().await?;
    resp.json::<Vec<AlerteReapprovisionnement>>().await
}

async fn fetch_inventaires_surplus(
    base_url: &str,
) -> Result<Vec<InventairesSurplus>, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/inventaires_surplus", base_url);
    let resp = client.get(&url).send().await?;
    resp.json::<Vec<InventairesSurplus>>().await
}

async fn fetch_tendances_hebdomadaires(
    base_url: &str,
) -> Result<Vec<TendancesHebdo>, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/tendances_hebdomadaires", base_url);
    let resp = client.get(&url).send().await?;
    resp.json::<Vec<TendancesHebdo>>().await
}

#[get("/performances")]
pub async fn performances() -> Result<Template, Status> {
    let base_url = match env::var("CENTRAL_URL") {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Erreur récupération CENTRAL_URL: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    let ventes = fetch_ventes(&base_url).await.map_err(|e| {
        eprintln!("Erreur fetch ventes: {}", e);
        Status::InternalServerError
    })?;

    let alertes = fetch_alerte_reapprovisionnements(&base_url)
        .await
        .map_err(|e| {
            eprintln!("Erreur fetch alertes: {}", e);
            Status::InternalServerError
        })?;

    let surplus = fetch_inventaires_surplus(&base_url).await.map_err(|e| {
        eprintln!("Erreur fetch surplus: {}", e);
        Status::InternalServerError
    })?;

    let tendances = fetch_tendances_hebdomadaires(&base_url)
        .await
        .map_err(|e| {
            eprintln!("Erreur fetch tendances: {}", e);
            Status::InternalServerError
        })?;

    let context = Context {
        ventes,
        alertes,
        surplus,
        tendances,
    };

    Ok(Template::render("performances", &context))
}
