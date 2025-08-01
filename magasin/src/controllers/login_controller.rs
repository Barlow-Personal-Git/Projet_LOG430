use crate::db::get_conn;
use crate::models::client::{Client, NouveauClient};
use crate::session::client_session::CLIENT_SESSION;
use crate::views::login_view;

use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;

pub async fn login() {
    login_view::afficher_bienvenue_magasin();

    let mut conn = get_conn();

    loop {
        let nom_utilisateur = login_view::demander_nom();

        if nom_utilisateur.trim().is_empty() {
            login_view::afficher_nom_invalide();
            continue;
        }

        use crate::schema::clients::dsl::*;
        let client_opt = clients
            .filter(nom.eq(&nom_utilisateur))
            .first::<Client>(&mut conn)
            .optional()
            .expect("Erreur DB");

        let client = match client_opt {
            Some(c) => {
                login_view::afficher_bienvenue(&nom_utilisateur);
                c
            }
            None => {
                let nouveau_client = NouveauClient {
                    nom: &nom_utilisateur,
                    role: "user",
                };
                diesel::insert_into(clients)
                    .values(&nouveau_client)
                    .execute(&mut conn)
                    .expect("Erreur insertion client");

                login_view::afficher_bienvenue(&format!("{nom_utilisateur} (Nouveau)"));

                clients
                    .select(Client::as_select())
                    .filter(nom.eq(&nom_utilisateur))
                    .first::<Client>(&mut conn)
                    .expect("Erreur récupération nouveau client")
            }
        };

        {
            let mut session = CLIENT_SESSION.lock().unwrap();
            session.set_client(client);
        }
        break;
    }
}
