use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::models::magasin::NouveauMagasin;
use crate::schema::magasins;

pub fn seed_magasins(conn: &mut PgConnection) -> QueryResult<()> {
    let magasins = vec![
        NouveauMagasin { nom: "CENTRAL"},
        NouveauMagasin { nom: "MAGASIN_1"},
        NouveauMagasin { nom: "MAGASIN_2"},
        NouveauMagasin { nom: "MAGASIN_3"},
        NouveauMagasin { nom: "MAGASIN_4"},
        NouveauMagasin { nom: "MAGASIN_5"},
    ];

    diesel::insert_into(magasins::table)
        .values(&magasins)
        .execute(conn)?;

    println!("Magasins insérés");

    Ok(())
}
