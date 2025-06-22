mod controllers;
mod db;
mod views;
mod models;
mod schema;
mod session;
mod seeds;

use controllers::login_controller;
use seeds::{seed_clients, seed_inventaires, seed_produits};
use std::env;
use db::establish_connection;
use diesel::pg::PgConnection;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "seed" {
        println!("🔧 Exécution des seeds...");

        let mut conn: PgConnection = establish_connection();

        seed_clients(&mut conn).expect("Erreur lors du seed clients");
        seed_produits(&mut conn).expect("Erreur lors du seed produits");
        seed_inventaires(&mut conn).expect("Erreur lors du seed inventaires");
        println!("✅ Seeds terminés !");
        return;
    }
    
    login_controller::login();
}
