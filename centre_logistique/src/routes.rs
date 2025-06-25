use rocket::{get, post, routes, Route};
use rocket::serde::json::Json;
use diesel::prelude::*;
use diesel::upsert::excluded;
use crate::db::get_conn;
use crate::models::{Message, Inventaire, Transaction, TransactionProduit, Magasin};
use crate::dto::{InventaireDTO, TransactionDTO, TransactionProduitDTO, MessageDTO};
use crate::schema::messages::dsl::*;
use crate::schema::transactions::dsl::*;
use crate::schema::transactions::dsl::{id_magasin as trans_id_magasin};
use crate::schema::inventaires::dsl::{
    inventaires, id_produit as inv_id_produit, id_magasin as inv_id_magasin, category as inv_category, nbr as inv_nbr
};
use crate::schema::transaction_produits::dsl::{
    transaction_produits, id_transaction as tp_id_transaction, id_produit as tp_id_produit, nbr as tp_nbr, total as tp_total
};
use crate::schema::magasins::dsl::*;
use crate::models::inventaire::NouveauInventaire;
use crate::models::transaction::NouvelleTransaction;
use crate::models::transaction_produit::NouveauTransactionProduit;
use crate::models::message::NouveauMessage;


#[get("/inventaires")]
async fn get_inventaires() -> Result<Json<Vec<Inventaire>>, String> {
    let mut conn = get_conn();

    inventaires
        .load::<Inventaire>(&mut conn)
        .map(|inv| Json(inv))
        .map_err(|e| format!("Erreur DB : {}", e))
}

#[post("/inventaires", data = "<data>")]
async fn post_inventaires(data: Json<InventaireDTO<'_>>) -> Result<String, String> {
    let mut conn = get_conn();

    let magasin_record = magasins
        .filter(nom.eq(&data.magasin))
        .first::<Magasin>(&mut conn)
        .map_err(|e| format!("Magasin inconnu : {}", e))?;
    
    let new_inv: Vec<NouveauInventaire> = data.inventaires.iter()
        .map(|inv| NouveauInventaire {
            id_produit: inv.id_produit,
            id_magasin: magasin_record.id_magasin,
            category: &inv.category,
            nbr: inv.nbr,
    }).collect();

    diesel::insert_into(inventaires)
        .values(&new_inv)
        .on_conflict((inv_id_produit, inv_id_magasin))
        .do_update()
        .set((
            inv_nbr.eq(excluded(inv_nbr)),
        ))
        .execute(&mut conn)
        .map_err(|e| format!("Erreur insertion: {}", e))?;

    Ok("Inventaire insérée".to_string())
}

#[get("/transactions")]
async fn get_transactions() -> Result<Json<Vec<Transaction>>, String> {
    let mut conn = get_conn();

    transactions
        .load::<Transaction>(&mut conn)
        .map(|inv| Json(inv))
        .map_err(|e| format!("Erreur DB : {}", e))
}

#[post("/transactions", data = "<data>")]
async fn post_transaction(data: Json<TransactionDTO<'_>>) -> Result<String, String> {
    let mut conn = get_conn();

    let magasin_record = magasins
        .filter(nom.eq(&data.magasin))
        .first::<Magasin>(&mut conn)
        .map_err(|e| format!("Magasin inconnu : {}", e))?;
    
    let new_tr: Vec<NouvelleTransaction> = data.transactions.iter()
        .map(|tr| NouvelleTransaction {
            id_magasin: magasin_record.id_magasin,
            total: tr.total,
            created_date: tr.created_date,
            updated_date: tr.updated_date
    }).collect();

    diesel::insert_into(transactions)
        .values(&new_tr)
        .on_conflict(trans_id_magasin)
        .do_update()
        .set((
            total.eq(excluded(total)),
            updated_date.eq(excluded(updated_date)),
        ))
        .execute(&mut conn)
        .map_err(|e| format!("Erreur insertion: {}", e))?;

    Ok("Transaction insérée".to_string())
}

#[get("/transaction_produits")]
async fn get_transaction_produits() -> Result<Json<Vec<TransactionProduit>>, String> {
    let mut conn = get_conn();

    transaction_produits
        .load::<TransactionProduit>(&mut conn)
        .map(|inv| Json(inv))
        .map_err(|e| format!("Erreur DB : {}", e))
}

#[post("/transaction_produits", data = "<data>")]
async fn post_transaction_produits(data: Json<TransactionProduitDTO<'_>>) -> Result<String, String> {
    let mut conn = get_conn();

    let new_trp: Vec<NouveauTransactionProduit> = data.transaction_produits.iter()
        .map(|trp| NouveauTransactionProduit {
            id_transaction: trp.id_transaction,
            id_produit: trp.id_produit,
            nbr: trp.nbr,
            total: trp.total
    }).collect();

    diesel::insert_into(transaction_produits)
        .values(&new_trp)
        .execute(&mut conn)
        .map_err(|e| format!("Erreur insertion: {}", e))?;

    Ok("Transaction Produits insérée".to_string())
}

#[get("/messages")]
async fn get_messages() -> Result<Json<Vec<Message>>, String> {
    let mut conn = get_conn();

    messages
        .load::<Message>(&mut conn)
        .map(|inv| Json(inv))
        .map_err(|e| format!("Erreur DB : {}", e))
}

#[post("/messages", data = "<data>")]
async fn post_message(data: Json<MessageDTO<'_>>) -> Result<String, String> {
    let mut conn = get_conn();

    let magasin_record = magasins
        .filter(nom.eq(&data.magasin))
        .first::<Magasin>(&mut conn)
        .map_err(|e| format!("Magasin inconnu : {}", e))?;
    
    let new_messages: Vec<NouveauMessage> = data.messages.iter().map(|msg| NouveauMessage {
        id_produit: msg.id_produit,
        id_magasin: magasin_record.id_magasin,
        message: &msg.message,
    }).collect();

    diesel::insert_into(messages)
        .values(&new_messages)
        .execute(&mut conn)
        .map_err(|e| format!("Erreur insertion: {}", e))?;

    Ok("Message insérée".to_string())
}

pub fn routes() -> Vec<Route> {
    routes![
        get_inventaires,
        post_inventaires,
        get_transactions,
        post_transaction,
        get_transaction_produits,
        post_transaction_produits,
        get_messages,
        post_message 
    ]
}