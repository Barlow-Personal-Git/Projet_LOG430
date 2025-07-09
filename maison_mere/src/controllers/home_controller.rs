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

#[get("/")]
pub fn home() -> Template {
    let context = Context {
        liens: vec![
            Lien { title: "Réapprovisionnement des magasins au centre logistique".to_string(), url: "/reapprovisionnements".to_string()},
            Lien { title: "Générer un rapport consolidé des ventes".to_string(), url: "/rapport".to_string()},
            Lien { title: "Visualiser les performances des magasins".to_string(), url: "/performances".to_string()}
        ]
    };
    Template::render("home", &context)
}