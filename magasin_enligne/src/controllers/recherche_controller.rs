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

use diesel::prelude::*;

#[get("/recherche")]
pub fn get_recherche() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("recherche", &context)
}
