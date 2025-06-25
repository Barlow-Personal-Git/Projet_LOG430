use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::models::inventaire::NouveauInventaire;
use crate::models::produit::Produit;
use crate::models::magasin::Magasin;
use crate::schema::inventaires;
use crate::schema::magasins::dsl::{magasins, nom as nom_magasin};
use crate::schema::produits::dsl::produits;
use diesel::associations::HasTable;

pub fn seed_inventaires(conn: &mut PgConnection) -> QueryResult<()> {
    let produits_list: Vec<Produit> = produits.load(conn)?;

    let central_magasin: Magasin = magasins.filter(nom_magasin.eq("CENTRAL")).first(conn)?;

    let inventaires = vec![
        NouveauInventaire { id_produit: produits_list[0].id_produit, id_magasin: central_magasin.id_magasin, category: "Breuvage", nbr: 200 },
        NouveauInventaire { id_produit: produits_list[1].id_produit, id_magasin: central_magasin.id_magasin, category: "Breuvage", nbr: 200 },
        NouveauInventaire { id_produit: produits_list[2].id_produit, id_magasin: central_magasin.id_magasin, category: "Breuvage", nbr: 200 },
        NouveauInventaire { id_produit: produits_list[3].id_produit, id_magasin: central_magasin.id_magasin, category: "Fruit", nbr: 200 },
        NouveauInventaire { id_produit: produits_list[4].id_produit, id_magasin: central_magasin.id_magasin, category: "Fruit", nbr: 200 },
        NouveauInventaire { id_produit: produits_list[5].id_produit, id_magasin: central_magasin.id_magasin, category: "Fruit", nbr: 200 },
        NouveauInventaire { id_produit: produits_list[6].id_produit, id_magasin: central_magasin.id_magasin, category: "Collation", nbr: 200 },
        NouveauInventaire { id_produit: produits_list[7].id_produit, id_magasin: central_magasin.id_magasin, category: "Collation", nbr: 200 },
        NouveauInventaire { id_produit: produits_list[8].id_produit, id_magasin: central_magasin.id_magasin, category: "Collation", nbr: 200 },
    ];

    diesel::insert_into(inventaires::table)
        .values(&inventaires)
        .execute(conn)?;

    println!("Inventaires insérés");

    Ok(())
}
