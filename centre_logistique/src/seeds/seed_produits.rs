use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::models::produit::NouveauProduit;
use crate::schema::produits;

pub fn seed_produits(conn: &mut PgConnection) -> QueryResult<()> {
    let produits = vec![
        NouveauProduit { nom: "Eau", prix: 1.00, description: "eau QC" },
        NouveauProduit { nom: "Café", prix: 1.50, description: "eau brun noir" },
        NouveauProduit { nom: "Thé", prix: 1.25, description: "eau brun" },
        NouveauProduit { nom: "Pomme", prix: 0.30, description: "fruit QC" },
        NouveauProduit { nom: "Orange", prix: 0.10, description: "fruit USA" },
        NouveauProduit { nom: "Fraise", prix: 0.10, description: "fruit CA"  },
        NouveauProduit { nom: "Biscuits", prix: 3.00, description: "Produit QC" },
        NouveauProduit { nom: "Chocolats", prix: 2.50, description: "Produit QC" },
        NouveauProduit { nom: "Chips", prix: 2.25, description: "Produit QC" },
    ];

    diesel::insert_into(produits::table)
        .values(&produits)
        .execute(conn)?;

    println!("Produits insérés");

    Ok(())
}
