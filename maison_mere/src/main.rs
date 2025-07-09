mod controllers;
mod routes;

use dotenvy::dotenv;
use routes::routes;
use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};
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
