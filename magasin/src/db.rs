use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use once_cell::sync::Lazy;
use std::env;

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub static DB_POOL: Lazy<PgPool> = Lazy::new(|| {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Erreur lors de la création du pool de connexion")
});

pub fn get_conn() -> diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>> {
    DB_POOL
        .get()
        .expect("Impossible d'obtenir une connexion du pool")
}
