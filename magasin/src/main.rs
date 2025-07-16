mod controllers;
mod db;
mod dto;
mod mappers;
mod models;
mod schema;
mod seeds;
mod session;
mod views;

use controllers::synchroniser_controller::sync_data;
use db::get_conn;
use seeds::{seed_clients, seed_inventaires, seed_produits};
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "seed" => {
                let mut conn = get_conn();
                seed_clients(&mut conn).unwrap();
                seed_produits(&mut conn).unwrap();
                seed_inventaires(&mut conn).unwrap();
                println!("Données insérées !");
            }
            "login" => {
                controllers::login_controller::login().await;

                tokio::spawn(async {
                    let mut interval = tokio::time::interval(Duration::from_secs(30));
                    loop {
                        interval.tick().await;
                        if let Err(e) = sync_data().await {
                            eprintln!("Erreur sync: {e}");
                        }
                    }
                });
                controllers::menu_controller::menu_principal().await;
            }
            _ => {
                println!("Commande inconnue. Utilise `seed` ou `login`.");
            }
        }
    } else {
        println!("Commande inconnue. Utilise `seed` ou `login`.");
    }
}
