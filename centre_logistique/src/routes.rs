use rocket::{routes, Route};
use crate::controllers::{
    inventaire_controller::*,
    transaction_controller::*,
    magasin_controller::*,
    message_controller::*,
};

pub fn routes() -> Vec<Route> {
    routes![
        get_inventaires,
        post_inventaires,
        get_transactions,
        post_transaction,
        get_transaction_produits,
        get_messages,
        post_message,
        get_sommes
    ]
}