use genpdf::{elements, Document};
use reqwest::Client;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use serde::Deserialize;
use std::env;
use std::io::Cursor;

#[derive(Deserialize)]
struct VenteParMagasin {
    magasin: String,
    total: f64,
}

#[derive(Deserialize)]
struct ProduitVendu {
    nom_produit: String,
    nbr_vendue: i32,
}

#[derive(Deserialize)]
struct InventaireRestants {
    nom_produit: String,
    nbr_inventaire: i32,
}

pub struct PdfResponse {
    pub content: Vec<u8>,
}

impl<'r> Responder<'r, 'static> for PdfResponse {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        Response::build()
            .header(ContentType::PDF)
            .sized_body(self.content.len(), Cursor::new(self.content))
            .ok()
    }
}

async fn fetch_ventes(base_url: &str) -> Result<Vec<VenteParMagasin>, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/ventes_magasin", base_url);
    let resp = client.get(&url).send().await?;
    resp.json::<Vec<VenteParMagasin>>().await
}

async fn fetch_produits_vendus(base_url: &str) -> Result<Vec<ProduitVendu>, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/produits_vendus", base_url);
    let resp = client.get(&url).send().await?;
    resp.json::<Vec<ProduitVendu>>().await
}

async fn fetch_inventaires_restants(
    base_url: &str,
) -> Result<Vec<InventaireRestants>, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/inventaires_restants", base_url);
    let resp = client.get(&url).send().await?;
    resp.json::<Vec<InventaireRestants>>().await
}

#[rocket::get("/rapport")]
pub async fn rapport() -> Result<PdfResponse, Status> {
    let base_url = match env::var("CENTRAL_URL") {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Erreur récupération CENTRAL_URL: {}", e);
            return Err(rocket::http::Status::InternalServerError);
        }
    };

    let ventes = fetch_ventes(&base_url).await.map_err(|e| {
        eprintln!("Erreur fetch ventes: {}", e);
        Status::InternalServerError
    })?;

    let produits_vendus = fetch_produits_vendus(&base_url).await.map_err(|e| {
        eprintln!("Erreur fetch produits vendus: {}", e);
        Status::InternalServerError
    })?;

    let inventaires_restants = fetch_inventaires_restants(&base_url).await.map_err(|e| {
        eprintln!("Erreur fetch inventaires restants: {}", e);
        Status::InternalServerError
    })?;

    let font_family =
        genpdf::fonts::from_files("./fonts", "LiberationSans", None).map_err(|e| {
            eprintln!("Erreur chargement police: {}", e);
            Status::InternalServerError
        })?;

    let mut doc = Document::new(font_family);
    doc.set_title("Rapport ventes par magasin");

    doc.push(elements::Paragraph::new("Ventes par magasin :"));
    for vente in ventes {
        doc.push(elements::Paragraph::new(format!(
            "{} : {:.2} $",
            vente.magasin, vente.total
        )));
    }

    doc.push(elements::Break::new(1));

    doc.push(elements::Paragraph::new("Produits vendus :"));
    for produit in produits_vendus {
        doc.push(elements::Paragraph::new(format!(
            "{} : {}",
            produit.nom_produit, produit.nbr_vendue
        )));
    }

    doc.push(elements::Break::new(1));

    doc.push(elements::Paragraph::new("Inventaires restants :"));
    for inventaire in inventaires_restants {
        doc.push(elements::Paragraph::new(format!(
            "{} : {}",
            inventaire.nom_produit, inventaire.nbr_inventaire
        )));
    }

    let mut buffer = Vec::new();
    doc.render(&mut buffer).map_err(|e| {
        eprintln!("Erreur génération PDF: {}", e);
        Status::InternalServerError
    })?;

    Ok(PdfResponse { content: buffer })
}
