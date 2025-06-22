use crate::views::menu_view;
use crate::controllers::recherche_controller;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::establish_connection;
// use crate::controllers::{recherche_controller, achat_controller, retour_controller, consulter_controller};

pub fn menu_principal() {
    loop {
        menu_view::afficher_choix();
        menu_view::afficher_recherche();
        menu_view::afficher_achat();
        menu_view::afficher_gerer_retour();
        menu_view::afficher_consulter_stock();
        menu_view::afficher_quitter();

        let choix = menu_view::demander_choix();

        match choix.as_str() {
            "1" => {
                let mut conn = establish_connection();
                recherche_controller::menu_recherche(&mut conn);
            },
            // "2" => achat_controller::menu_achat(),
            // "3" => retour_controller::menu_retour(),
            // "4" => consulter_controller::consulter_liste_produit(),
            "5" => break,
            _ => println!("Choix invalide."),
        }
    }
}
