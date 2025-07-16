use crate::dto::{NouvelleProduitsDTO, NouvelleTransactionProduitsDTO, TransactionProduitsDTO};
use crate::models::produit::Produit;
use crate::models::transaction::Transaction;
use crate::models::transaction_produit::TransactionProduit;
use std::collections::HashMap;

pub fn map_transaction_produits(
    magasin: &str,
    transactions: Vec<Transaction>,
    transaction_produits: Vec<TransactionProduit>,
    produits: Vec<Produit>,
) -> TransactionProduitsDTO {
    let produit_map: HashMap<i32, &Produit> = produits.iter().map(|p| (p.id_produit, p)).collect();

    let mut grouped: HashMap<i32, Vec<&TransactionProduit>> = HashMap::new();
    for tp in &transaction_produits {
        grouped
            .entry(tp.id_transaction)
            .or_insert(Vec::new())
            .push(tp);
    }

    let transactions_dto = transactions
        .iter()
        .filter_map(|t| {
            grouped.get(&t.id_transaction).map(|tps| {
                let produits_dto = tps
                    .iter()
                    .filter_map(|tp| {
                        produit_map
                            .get(&tp.id_produit)
                            .map(|p| NouvelleProduitsDTO {
                                nom: p.nom.clone(),
                                prix: p.prix,
                                nbr: tp.nbr,
                            })
                    })
                    .collect();

                NouvelleTransactionProduitsDTO {
                    id_transaction: t.id_transaction,
                    produits: produits_dto,
                    total: t.total,
                }
            })
        })
        .collect();

    TransactionProduitsDTO {
        magasin: magasin.to_string(),
        transaction_produits: transactions_dto,
    }
}
