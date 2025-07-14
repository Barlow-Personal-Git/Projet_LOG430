use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::models::inventaire::NouveauInventaire;
use crate::models::produit::Produit;
use crate::schema::produits;
use crate::schema::inventaires;

pub fn seed_inventaires(conn: &mut PgConnection) -> QueryResult<()> {
    let produits: Vec<Produit> = produits::table.load(conn)?;

    let inventaires = vec![
        NouveauInventaire { id_produit: produits[0].id_produit, category: "Breuvage", nbr: 25 },
        NouveauInventaire { id_produit: produits[1].id_produit, category: "Breuvage", nbr: 25 },
        NouveauInventaire { id_produit: produits[2].id_produit, category: "Breuvage", nbr: 25 },
        NouveauInventaire { id_produit: produits[3].id_produit, category: "Fruit", nbr: 35 },
        NouveauInventaire { id_produit: produits[4].id_produit, category: "Fruit", nbr: 30 },
        NouveauInventaire { id_produit: produits[5].id_produit, category: "Fruit", nbr: 40 },
        NouveauInventaire { id_produit: produits[6].id_produit, category: "Collation", nbr: 40 },
        NouveauInventaire { id_produit: produits[7].id_produit, category: "Collation", nbr: 40 },
        NouveauInventaire { id_produit: produits[8].id_produit, category: "Collation", nbr: 30 },
    ];

    diesel::insert_into(inventaires::table)
        .values(&inventaires)
        .execute(conn)?;

    println!("Inventaires insérés");

    Ok(())
}
