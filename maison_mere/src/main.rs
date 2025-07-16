mod controllers;
mod routes;

use dotenvy::dotenv;
use rocket_dyn_templates::Template;
use routes::routes;

#[rocket::main]
async fn main() -> Result<(), Box<rocket::Error>> {
    dotenv().ok();

    rocket::build()
        .mount("/", routes())
        .attach(Template::fairing())
        .launch()
        .await?;
    Ok(())
}
