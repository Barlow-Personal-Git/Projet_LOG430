use rocket::{routes, Route};

use crate::controllers::{
    login_controller::*,
    home_controller::*,
    menu_controller::*,
    recherche_controller::*,
    produit_controller::*,
    achat_controller::*,
};

pub fn routes() -> Vec<Route> {
    routes![
        home,
        get_login,
        post_login,
        menu,
        get_recherche,
        get_produit,
        get_panier,
        delete_panier,
        post_panier,
        get_ajouter_panier,
        confirmer_achat,
    ]
}