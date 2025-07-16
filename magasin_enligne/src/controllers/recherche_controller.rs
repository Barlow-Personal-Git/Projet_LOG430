use rocket::get;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/recherche")]
pub fn get_recherche() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("recherche", &context)
}
