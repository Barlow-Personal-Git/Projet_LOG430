mod controllers;
mod db;
mod dto;
mod views;
mod mappers;
mod models;
mod schema;
mod session;
mod seeds;

use seeds::{seed_clients, seed_inventaires, seed_produits};
use std::env;
use db::get_conn;

fn main(){

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
                controllers::login_controller::login();
            }
            _ => {
                println!("Commande inconnue. Utilise `seed` ou `login`.");
            }
        }
    } else {
        println!("Commande inconnue. Utilise `seed` ou `login`.");
    }
}
