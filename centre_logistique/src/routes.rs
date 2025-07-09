use rocket::{routes, Route};
use crate::controllers::{
    inventaire_controller::*,
    transaction_controller::*,
    transaction_produit_controller::*,
    message_controller::*,
};

pub fn routes() -> Vec<Route> {
    routes![
        get_inventaires,
        post_inventaires,
        get_transactions,
        post_transaction,
        get_transaction_produits,
        post_transaction_produits,
        get_messages,
        post_message,
        get_produits_vendus,
        get_ventes_magasin,
        get_inventaires_faible,
        get_inventaires_surplus,
        get_tendances_hebdomadaires
    ]
}