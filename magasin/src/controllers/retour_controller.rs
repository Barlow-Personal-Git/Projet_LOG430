use crate::db::get_conn;
use crate::models::{
    inventaire::Inventaire, transaction::Transaction, transaction_produit::TransactionProduit,
};
use crate::schema::produits;
use crate::session::client_session::CLIENT_SESSION;
use crate::views::retour_view;
use diesel::prelude::*;

pub fn menu_retour() -> Result<(), diesel::result::Error> {
    let mut conn = get_conn();

    loop {
        retour_view::afficher_choix();
        retour_view::afficher_retour();
        retour_view::afficher_consulter_liste_vente();
        retour_view::afficher_quitter();

        let choix = retour_view::demander_choix();

        match choix.as_str() {
            "1" => retourner_transaction(&mut conn)?,
            "2" => consulter_vente(&mut conn)?,
            "3" => break,
            _ => println!("Choix invalide, veuillez réessayer."),
        }
    }
    Ok(())
}

fn retourner_transaction(conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
    retour_view::afficher_vente_retour();
    let transaction_id_str = retour_view::demander_vente_id();

    if transaction_id_str.to_lowercase() == "back" {
        return Ok(());
    }

    let transaction_id: i32 = match transaction_id_str.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Identifiant invalide.");
            return Ok(());
        }
    };

    let session = CLIENT_SESSION.lock().unwrap();
    let client = match session.get_client() {
        Some(c) => c,
        None => {
            println!("Aucun client connecté.");
            return Ok(());
        }
    };
    drop(session);

    use crate::schema::{inventaires, transaction_produits, transactions};

    let transaction_opt = transactions::table
        .filter(transactions::id_transaction.eq(transaction_id))
        .filter(transactions::id_client.eq(client.id_client))
        .first::<Transaction>(conn)
        .optional()?;

    let transaction = match transaction_opt {
        Some(t) => t,
        None => {
            retour_view::afficher_vente_introuvable();
            return Ok(());
        }
    };

    let produits_transaction = transaction_produits::table
        .filter(transaction_produits::id_transaction.eq(transaction.id_transaction))
        .inner_join(produits::table.on(produits::id_produit.eq(transaction_produits::id_produit)))
        .load::<(TransactionProduit, crate::models::produit::Produit)>(conn)?;

    conn.transaction::<(), diesel::result::Error, _>(|conn| {
        for (tp, _produit) in &produits_transaction {
            let mut inventaire_opt = inventaires::table
                .filter(inventaires::id_produit.eq(tp.id_produit))
                .first::<Inventaire>(conn)
                .optional()?;

            if let Some(mut inventaire) = inventaire_opt.take() {
                inventaire.nbr += tp.nbr;

                diesel::update(inventaires::table.find(inventaire.id_inventaire))
                    .set(inventaires::nbr.eq(inventaire.nbr))
                    .execute(conn)?;
            }
        }

        for (tp, _produit) in &produits_transaction {
            diesel::delete(transaction_produits::table.find(tp.id_transaction_produit))
                .execute(conn)?;
        }

        diesel::delete(transactions::table.find(transaction.id_transaction)).execute(conn)?;

        Ok(())
    })?;

    retour_view::afficher_transaction(transaction_id);
    Ok(())
}

fn consulter_vente(conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
    let session = CLIENT_SESSION.lock().unwrap();
    let client = match session.get_client() {
        Some(c) => c,
        None => {
            println!("Aucun client connecté.");
            return Ok(());
        }
    };
    drop(session);

    use crate::schema::transactions;

    let transactions = transactions::table
        .filter(transactions::id_client.eq(client.id_client))
        .load::<Transaction>(conn)?;

    if transactions.is_empty() {
        retour_view::afficher_vente_introuvable();
        return Ok(());
    }

    for transaction in transactions {
        retour_view::afficher_vente_disponible(&transaction);
    }

    Ok(())
}
