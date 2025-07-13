mod controllers;
mod db;
mod dto;
mod models;
mod schema;
mod routes;
mod seeds;
mod metrics;
mod cache;

use routes::routes;
use rocket::Config;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use seeds::{seed_inventaires, seed_magasins, seed_produits};
use std::env;
use db::get_conn;
use std::fs::File;
use tracing_subscriber::EnvFilter;
use tracing_appender::{non_blocking, non_blocking::WorkerGuard};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port_str = env::var("SERVER_PORT").unwrap_or_else(|_| "8000".to_string());
    let port: u16 = port_str.parse().expect("SERVER_PORT doit être un entier valide");

    let file = File::create("logs.json").expect("Impossible de créer logs");
    let (file_writer, _guard_file): (_, WorkerGuard) = non_blocking(file);

    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .with_writer(file_writer)
        .init();
    
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

    metrics::init_metrics();
    
    let config = Config {
        address: host.parse().expect("IP invalide"),
        port,
        ..Config::default()
    };

    let mut rocket = rocket::custom(config)
    .mount("/api", routes());

    if cfg!(debug_assertions) {
        rocket = rocket.mount(
            "/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/api/openapi.json".to_string(),
                ..Default::default()
            }),
        );
    }
    println!("Server run on http://{}:{}", host, port);
    rocket.launch().await?;
    Ok(())
}
