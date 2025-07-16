use std::io::{self, Write};

pub fn afficher_choix() {
    println!("\nVeuillez sélectionner une des choix");
}

pub fn afficher_retour() {
    println!("1. Retourner une vente");
}

pub fn afficher_consulter_liste_vente() {
    println!("2. Consulter les ventes");
}

pub fn afficher_quitter() {
    println!("3. Quitter");
}

pub fn demander_choix() -> String {
    print!("Choix : ");
    io::stdout().flush().unwrap();

    let mut choix = String::new();
    io::stdin().read_line(&mut choix).unwrap();
    choix.trim().to_string()
}

pub fn afficher_vente_retour() {
    println!("\nVeuillez inscrire l'id de la vente ou taper 'Back' pour retourner en arrière");
}

pub fn demander_vente_id() -> String {
    print!("Vente ID : ");
    io::stdout().flush().unwrap();

    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    id.trim().to_string()
}

pub fn afficher_vente_introuvable() {
    println!("Vente introuvable.");
}

pub fn afficher_transaction(transaction_id: i32) {
    println!("Vente {transaction_id} annulée.");
}

pub fn afficher_vente_disponible(transaction: &crate::models::transaction::Transaction) {
    println!(
        "ID : {} | Total: {:.2}$",
        transaction.id_transaction, transaction.total
    );
}
