use crate::models::{
    produit::Produit,
    inventaire::Inventaire,
    transaction::{Transaction, NouvelleTransaction},
    transaction_produit::NouveauTransactionProduit,
};
use diesel::prelude::*;
use crate::schema::produits::dsl::*;
use crate::schema::inventaires::dsl::{inventaires, id_produit as inv_id_produit, nbr as inv_nbr};
use crate::schema::transactions::dsl::{transactions, id_transaction as trans_id_transaction, total as trans_total};
use crate::schema::transaction_produits::dsl::{transaction_produits};
use crate::views::achat_view;
use crate::session::client_session::CLIENT_SESSION;
use crate::db::get_conn;
use chrono::Utc;

pub fn menu_achat() {
    let mut conn = get_conn();
    loop {
        achat_view::afficher_choix();
        achat_view::afficher_achat();
        achat_view::afficher_produits();
        achat_view::afficher_quitter();

        let choix = achat_view::demander_choix();

        match choix.as_str() {
            "1" => ajouter_produit(&mut conn),
            "2" => consulter_produit(&mut conn),
            "3" => break,
            _ => println!("Choix invalide, veuillez réessayer."),
        }
    }
}

fn ajouter_produit(conn: &mut PgConnection) {
    loop {
        achat_view::afficher_ajouter_produit();
        let produit_id_str = achat_view::demander_choix_ajouter();

        if produit_id_str.to_lowercase() == "back" {
            break;
        }

        let produit_id: i32 = match produit_id_str.trim().parse() {
            Ok(id) => id,
            Err(_) => {
                println!("Identifiant invalide.");
                continue;
            }
        };

        let produit_opt = produits.find(produit_id).first::<Produit>(conn).optional();
        let produit = match produit_opt {
            Ok(Some(p)) => p,
            Ok(None) => {
                println!("Produit non trouvé.");
                continue;
            }
            Err(e) => {
                println!("Erreur DB : {}", e);
                continue;
            }
        };

        let nbr_str = achat_view::demande_quantite();
        let quantite: u32 = match nbr_str.trim().parse() {
            Ok(n) if n > 0 => n,
            _ => {
                achat_view::afficher_zero();
                continue;
            }
        };

        let inventaire_opt = inventaires
            .filter(inv_id_produit.eq(produit.id_produit))
            .first::<Inventaire>(conn)
            .optional();

        let inventaire = match inventaire_opt {
            Ok(Some(inv)) => inv,
            Ok(None) => {
                achat_view::afficher_inventaire_pas_enregistrer();
                continue;
            }
            Err(e) => {
                println!("Erreur DB : {}", e);
                continue;
            }
        };

        if inventaire.nbr < quantite as i32 {
            achat_view::afficher_insuffisant();
            continue;
        }

        let mut session = CLIENT_SESSION.lock().unwrap();
        session.add_produit(produit.clone(), quantite);
        achat_view::afficher_produit_ajouter(&produit, quantite);
    }
}

fn consulter_produit(conn: &mut PgConnection) {
    let session = CLIENT_SESSION.lock().unwrap();
    let produits_liste = session.get_produits();
    drop(session);

    if produits_liste.is_empty() {
        achat_view::afficher_aucun_produit();
        return;
    }

    achat_view::afficher_ventes();

    let mut total_transaction: f32 = 0.0;

    for (produit, quantite) in &produits_liste {
        let produit_total = *quantite as f32 * produit.prix as f32;
        total_transaction += produit_total;
        achat_view::afficher_produit_total(produit, *quantite, produit_total.into());
    }

    achat_view::afficher_total(total_transaction.into());

    loop {
        achat_view::afficher_choix();
        achat_view::afficher_confirmer();
        achat_view::afficher_effacer();
        achat_view::afficher_quitter();

        let choix = achat_view::demander_choix();
        match choix.as_str() {
            "1" => {
                confirmer_vente(conn);
                break;
            }
            "2" => {
                restart_vente();
                break;
            }
            "3" => break,
            _ => println!("Choix invalide."),
        }
    }
}

fn restart_vente() {
    let mut session = CLIENT_SESSION.lock().unwrap();
    session.clear_vente();
}

fn confirmer_vente(conn: &mut PgConnection) {
    let res = conn.transaction::<(), diesel::result::Error, _>(|conn| {
        let session = CLIENT_SESSION.lock().unwrap();
        let client = match session.get_client() {
            Some(c) => c,
            None => {
                println!("Aucun client connecté.");
                return Ok(());
            }
        };

        let produits_liste = session.get_produits();
        drop(session);

        let now = Utc::now().naive_utc();

        let new_transaction = NouvelleTransaction {
            id_client: client.id_client,
            total: 0.0,
            created_date: now,
            updated_date: now,
        };

        diesel::insert_into(transactions)
            .values(&new_transaction)
            .execute(conn)?;

        let transaction: Transaction = transactions
            .order(trans_id_transaction.desc())
            .first(conn)?;

        let mut total_transaction: f32 = 0.0;

        for (produit, quantite) in produits_liste {
            let total_produit = produit.prix as f32 * quantite as f32;

            let inventaire_opt = inventaires
                .filter(inv_id_produit.eq(produit.id_produit))
                .first::<Inventaire>(conn)
                .optional()?;

            let mut inventaire = match inventaire_opt {
                Some(inv) => inv,
                None => {
                    achat_view::afficher_insuffisant();
                    return Err(diesel::result::Error::RollbackTransaction);
                }
            };

            if inventaire.nbr < quantite as i32 {
                achat_view::afficher_insuffisant();
                return Err(diesel::result::Error::RollbackTransaction);
            }

            inventaire.nbr -= quantite as i32;

            diesel::update(inventaires.find(inventaire.id_inventaire))
                .set(inv_nbr.eq(inventaire.nbr))
                .execute(conn)?;

            let nouveau_tp = NouveauTransactionProduit {
                id_transaction: transaction.id_transaction,
                id_produit: produit.id_produit,
                nbr: quantite as i32,
                total: total_produit,
            };

            diesel::insert_into(transaction_produits)
                .values(&nouveau_tp)
                .execute(conn)?;

            total_transaction += total_produit;
        }

        diesel::update(transactions.find(transaction.id_transaction))
            .set(trans_total.eq(total_transaction))
            .execute(conn)?;

        let mut session = CLIENT_SESSION.lock().unwrap();
        session.clear_vente();

        achat_view::afficher_vente_confirmer();

        Ok(())
    });

    if let Err(e) = res {
        println!("Erreur lors de la transaction : {:?}", e);
    }
}
