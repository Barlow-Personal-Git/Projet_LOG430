use rocket_dyn_templates::Template;
use rocket::get;
use serde::Serialize;

#[derive(Serialize)]
struct Context {
    liens: Vec<Lien>,
}

#[derive(Serialize)]
struct Lien {
   title: String,
   url: String,
}

#[get("/menu")]
pub fn menu() -> Template {
    let context = Context {
        liens: vec![
            Lien { title: "Menu recherche".to_string(), url: "/recherche".to_string()},
            Lien { title: "Menu achat".to_string(), url: "/panier".to_string()},
            Lien { title: "Menu consulter la liste des produits".to_string(), url: "/produit".to_string()}
        ]
    };
    Template::render("menu", &context)
}