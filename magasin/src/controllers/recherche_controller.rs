use diesel::prelude::*;
use crate::models::{produit::Produit, inventaire::Inventaire};
use crate::views::recherche_view;
use crate::schema::{produits::dsl as p_dsl, inventaires::dsl as i_dsl};

pub fn menu_recherche(conn: &mut PgConnection) {
    loop {
        recherche_view::afficher_choix();
        recherche_view::afficher_identifiant();
        recherche_view::afficher_nom();
        recherche_view::afficher_categorie();
        recherche_view::afficher_quitter();

        let choix = recherche_view::demander_choix();

        match choix.as_str() {
            "1" => menu_recherche_id(conn),
            "2" => menu_recherche_nom(conn),
            "3" => menu_recherche_categorie(conn),
            "4" => break,
            _ => println!("Choix invalide, veuillez réessayer."),
        }
    }
}

fn menu_recherche_nom(conn: &mut PgConnection) {
    loop {
        recherche_view::afficher_recherche_choix("nom");
        let recherche = recherche_view::demander_recherche_choix("nom");

        if recherche.to_lowercase() == "back" {
            break;
        }

        if !recherche.trim().is_empty() {
            match p_dsl::produits
                .filter(p_dsl::nom.ilike(format!("%{}%", recherche.trim())))
                .load::<Produit>(conn)
            {
                Ok(produits_vec) if !produits_vec.is_empty() => {
                    recherche_view::afficher_titre_liste();
                    for p in produits_vec {
                        recherche_view::afficher_produit(&p);
                    }
                }
                Ok(_) => println!("Aucun produit trouvé."),
                Err(e) => println!("Erreur lors de la recherche : {}", e),
            }
        } else {
            println!("Veuillez réessayer de nouveau !");
        }
    }
}

fn menu_recherche_id(conn: &mut PgConnection) {
    loop {
        recherche_view::afficher_recherche_choix("id");
        let recherche = recherche_view::demander_recherche_choix("id");

        if recherche.to_lowercase() == "back" {
            break;
        }

        if let Ok(id) = recherche.trim().parse::<i32>() {
            match p_dsl::produits.find(id).first::<Produit>(conn) {
                Ok(produit) => {
                    recherche_view::afficher_titre_produit();
                    recherche_view::afficher_produit(&produit);
                }
                Err(diesel::result::Error::NotFound) => {
                    recherche_view::afficher_indisponible();
                }
                Err(e) => {
                    println!("Erreur lors de la recherche : {}", e);
                }
            }
        } else {
            recherche_view::afficher_erreur();
        }
    }
}

fn menu_recherche_categorie(conn: &mut PgConnection) {
    loop {
        recherche_view::afficher_recherche_choix("categorie");
        let categorie = recherche_view::demander_recherche_choix("categorie");

        if categorie.to_lowercase() == "back" {
            break;
        }

        if !categorie.trim().is_empty() {
            let resultats = i_dsl::inventaires
                .inner_join(p_dsl::produits.on(p_dsl::id_produit.eq(i_dsl::id_produit)))
                .filter(i_dsl::category.eq(categorie.trim()))
                .select((i_dsl::inventaires::all_columns(), p_dsl::produits::all_columns()))
                .load::<(Inventaire, Produit)>(conn);

            match resultats {
                Ok(paires) if !paires.is_empty() => {
                    recherche_view::afficher_titre_liste();
                    for (_inv, prod) in paires {
                        recherche_view::afficher_produit(&prod);
                    }
                }
                Ok(_) => recherche_view::afficher_indisponible(),
                Err(e) => println!("Erreur lors de la recherche : {}", e),
            }
        } else {
            recherche_view::afficher_erreur();
        }
    }
}
