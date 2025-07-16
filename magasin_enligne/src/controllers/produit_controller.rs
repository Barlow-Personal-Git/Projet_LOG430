use crate::db::get_conn;
use crate::models::produit::Produit;
use crate::schema::inventaires::dsl::{
    category as i_category, id_produit as i_id_produit, inventaires,
};
use crate::schema::produits::dsl::{id_produit as p_id_produit, nom as p_nom, produits};
use diesel::prelude::*;
use rocket::get;
use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct AffichageProduit {
    id_produit: i32,
    nom: String,
    description: Option<String>,
    prix_decimal: String,
}

#[derive(Serialize)]
struct ProduitContext<'a> {
    message: Option<&'a str>,
    resultats: Option<Vec<AffichageProduit>>,
}

#[get("/produit?<id_produit>&<nom>&<categorie>")]
pub fn get_produit(
    id_produit: Option<i32>,
    nom: Option<String>,
    categorie: Option<String>,
) -> Template {
    let mut conn = get_conn();

    let resultats = if let Some(id_val) = id_produit {
        produits
            .filter(p_id_produit.eq(id_val))
            .load::<Produit>(&mut conn)
            .ok()
    } else if let Some(nom_val) = nom {
        produits
            .filter(p_nom.ilike(format!("%{nom_val}%")))
            .load::<Produit>(&mut conn)
            .ok()
    } else if let Some(category_val) = categorie {
        inventaires
            .inner_join(produits.on(p_id_produit.eq(i_id_produit)))
            .filter(i_category.ilike(format!("%{category_val}%")))
            .select(produits::all_columns())
            .load::<Produit>(&mut conn)
            .ok()
    } else {
        produits.load::<Produit>(&mut conn).ok()
    };

    let context = if let Some(prods) = resultats {
        if prods.is_empty() {
            ProduitContext {
                message: Some("Aucun produit trouvé"),
                resultats: None,
            }
        } else {
            let p_affichage = prods
                .into_iter()
                .map(|p| AffichageProduit {
                    id_produit: p.id_produit,
                    nom: p.nom,
                    description: Some(p.description),
                    prix_decimal: format!("{:.2}", p.prix),
                })
                .collect();

            ProduitContext {
                message: None,
                resultats: Some(p_affichage),
            }
        }
    } else {
        ProduitContext {
            message: Some("Erreur lors de la recherche"),
            resultats: None,
        }
    };

    Template::render("produit", &context)
}
