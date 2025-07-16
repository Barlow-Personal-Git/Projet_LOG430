use crate::db::get_conn;
use crate::dto::MessageDTO;
use crate::models::message::NouveauMessage;
use crate::models::{Magasin, Message};
use crate::schema::magasins::dsl::{magasins, nom};
use crate::schema::messages::dsl::messages;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{get, post};
use rocket_okapi::openapi;

#[openapi]
#[get("/messages")]
pub async fn get_messages() -> Result<Json<Vec<Message>>, String> {
    let mut conn = get_conn();

    messages
        .load::<Message>(&mut conn)
        .map(Json)
        .map_err(|e| format!("Erreur DB : {e}"))
}

#[openapi]
#[post("/messages", data = "<data>")]
pub async fn post_message(data: Json<MessageDTO<'_>>) -> Result<String, String> {
    let mut conn = get_conn();

    let magasin_record = magasins
        .filter(nom.eq(&data.magasin))
        .first::<Magasin>(&mut conn)
        .map_err(|e| format!("Magasin inconnu : {e}"))?;

    let new_messages: Vec<NouveauMessage> = data
        .messages
        .iter()
        .map(|msg| NouveauMessage {
            id_produit: msg.id_produit,
            id_magasin: magasin_record.id_magasin,
            message: msg.message,
        })
        .collect();

    diesel::insert_into(messages)
        .values(&new_messages)
        .execute(&mut conn)
        .map_err(|e| format!("Erreur insertion: {e}"))?;

    Ok("Message insérée".to_string())
}
