mod controllers;
mod db;
mod dto;
mod models;
mod schema;
mod routes;
mod seeds;

use routes::routes;
use seeds::{seed_inventaires, seed_magasins, seed_produits};
use std::env;
use db::get_conn;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "seed" {
        println!("Exécution des seeds...");

        let mut conn = get_conn();

        seed_magasins(&mut conn).expect("Erreur lors du seed magasins");
        seed_produits(&mut conn).expect("Erreur lors du seed produits");
        seed_inventaires(&mut conn).expect("Erreur lors du seed inventaires");
        println!("Seeds terminés !");
        return Ok(());
    }
    
    rocket::build()
        .mount("/", routes())
        .launch()
        .await?;
    Ok(())
}
