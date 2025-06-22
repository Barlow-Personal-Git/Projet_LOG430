use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::models::client::{Client, NouveauClient};
use crate::schema::clients;

pub fn seed_clients(conn: &mut PgConnection) -> QueryResult<()> {
    let clients = vec![
        NouveauClient { role: "admin", nom: "admin2" },
        NouveauClient { role: "admin", nom: "admin" },
    ];

    diesel::insert_into(clients::table)
        .values(&clients)
        .execute(conn)?;

    println!("Clients insérés");

    Ok(())
}
