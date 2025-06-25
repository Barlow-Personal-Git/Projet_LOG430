use rocket::{routes, Route};
use crate::controllers::{
    inventaire_controller,
    transaction_controller,
    transaction_produit_controller,
};

pub fn routes_api() -> Vec<Route> {
    routes![
        inventaire_controller::get_inventaires,
        transaction_controller::get_transactions,
        transaction_produit_controller::get_transaction_produits,
    ]
}
