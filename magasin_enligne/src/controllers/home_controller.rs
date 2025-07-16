use rocket::get;
use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct Context {
    lien: Lien,
}

#[derive(Serialize)]
struct Lien {
    title: String,
    url: String,
}

#[get("/")]
pub fn home() -> Template {
    let context = Context {
        lien: Lien {
            title: "Login".to_string(),
            url: "/login".to_string(),
        },
    };
    Template::render("home", &context)
}
