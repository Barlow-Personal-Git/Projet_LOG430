use rocket::form::Form;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::uri;
use rocket::{get, post};
use rocket_dyn_templates::Template;
use rocket_okapi::openapi;
use std::collections::HashMap;

use crate::db::get_conn;
use crate::dto::LoginDTO;
use crate::models::client::{Client, NouveauClient};
use crate::schema::clients;
use crate::schema::clients::dsl::*;
use crate::session::client_session::CLIENT_SESSION;

use diesel::prelude::*;

#[get("/login")]
pub async fn get_login() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("login", &context)
}

#[openapi]
#[post("/login", data = "<login_data>")]
pub async fn post_login(login_data: Form<LoginDTO>) -> Result<Redirect, Status> {
    let login_data = login_data.into_inner();
    let mut conn = get_conn();

    let client_opt = clients
        .filter(nom.eq(&login_data.nom))
        .first::<Client>(&mut conn)
        .optional()
        .map_err(|_| Status::InternalServerError)?;

    let client = match client_opt {
        Some(c) => c,
        None => {
            let nouveau_client = NouveauClient {
                nom: &login_data.nom,
                role: "user",
            };
            diesel::insert_into(clients)
                .values(&nouveau_client)
                .execute(&mut conn)
                .map_err(|_| Status::InternalServerError)?;

            clients
                .filter(nom.eq(&login_data.nom))
                .first::<Client>(&mut conn)
                .map_err(|_| Status::InternalServerError)?
        }
    };

    {
        let mut session = CLIENT_SESSION.lock().unwrap();
        session.set_client(client);
    }
    Ok(Redirect::to("/menu"))
}
