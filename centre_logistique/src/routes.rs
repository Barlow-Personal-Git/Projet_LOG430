use crate::controllers::{
    inventaire_controller::*, message_controller::*, produit_controller::*,
    reapprovisionnement_controller::*, transaction_controller::*,
    transaction_produit_controller::*,
};
use crate::metrics::metrics;
use rocket::{routes, Route};
use rocket_okapi::openapi_get_routes;

pub fn routes() -> Vec<Route> {
    let mut routes = openapi_get_routes![
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
        get_tendances_hebdomadaires,
        get_reapprovisionnements,
        post_reapprovisionnements,
        put_reapprovisionnement,
        get_inventaires_restants,
        get_alerte_reapprovisionnements,
        get_inventaires_id,
        put_inventaire,
        get_inventaires_par_magasins,
        get_produits,
        get_produit,
        put_produit,
    ];
    routes.extend(routes![metrics]);

    routes
}
