use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("L'environnement n'est pas configuré");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Erreur de connection : {}", database_url))
}
