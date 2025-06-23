use diesel::prelude::*;
use diesel::ExpressionMethods;
use crate::db::get_conn;
use crate::models::inventaire::Inventaire;
use crate::models::produit::Produit;
use crate::schema::{inventaires, produits};
use crate::views::consulter_view;

pub fn consulter_liste_produit() {
    let mut conn = get_conn();

    let inventaires_result = inventaires::table
        .inner_join(produits::table)
        .order(inventaires::category.asc())
        .load::<(Inventaire, Produit)>(&mut conn);

    consulter_view::afficher_titre();

    match inventaires_result {
        Ok(inventaires_produits) => {
            for (inventaire, produit) in inventaires_produits {
                consulter_view::afficher_inventaire(&produit, &inventaire);
            }
        }
        Err(e) => {
            eprintln!("Erreur lors de la récupération des inventaires : {}", e);
        }
    }
}
