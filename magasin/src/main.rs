mod controllers;
mod db;
mod dto;
mod views;
mod models;
mod schema;
mod session;
mod seeds;
mod routes;

use crate::routes::api::routes_api;
use controllers::login_controller;
use seeds::{seed_clients, seed_inventaires, seed_produits};
use std::env;
use db::get_conn;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "seed" => {
                let mut conn = get_conn();
                seed_clients(&mut conn).unwrap();
                seed_produits(&mut conn).unwrap();
                seed_inventaires(&mut conn).unwrap();
                println!("Données insérées !");
                Ok(())
            }
            "login" => {
                controllers::login_controller::login();
                Ok(())
            }
            _ => {
                println!("Commande inconnue. Utilise `seed` ou `login`.");
                Ok(())
            }
        }
    } else {
        let _rocket = rocket::build()
            .mount("/api", routes_api())
            .launch()
            .await?;

        Ok(())
    }
}
