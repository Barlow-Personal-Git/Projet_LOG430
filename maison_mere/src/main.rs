mod controllers;
mod routes;

use dotenvy::dotenv;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use routes::routes;
use std::env;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    rocket::build()
        .mount("/", routes())
        .attach(Template::fairing())
        .launch()
        .await?;
    Ok(())
}
