mod controllers;
mod db;
mod dto;
mod models;
mod routes;
mod schema;
mod seeds;
mod session;

use db::get_conn;
use dotenvy::dotenv;
use rocket_dyn_templates::Template;
use routes::routes;
use seeds::{seed_clients, seed_inventaires, seed_produits};
use std::env;

#[rocket::main]
async fn main() -> Result<(), Box<rocket::Error>> {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "seed" {
        println!("<-Exécution des seeds->");

        let mut conn = get_conn();

        seed_clients(&mut conn).expect("Erreur lors du seed clients");
        seed_produits(&mut conn).expect("Erreur lors du seed produits");
        seed_inventaires(&mut conn).expect("Erreur lors du seed inventaires");
        println!("Seeds terminés !");
        return Ok(());
    }

    rocket::build()
        .mount("/", routes())
        .attach(Template::fairing())
        .launch()
        .await?;
    Ok(())
}
