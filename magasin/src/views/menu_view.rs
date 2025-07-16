use std::io::{self, Write};

pub fn afficher_choix() {
    println!("\nVeuillez sélectionner une des choix");
}

pub fn afficher_recherche() {
    println!("1. Rechercher un produit");
}

pub fn afficher_achat() {
    println!("2. Enregistrer une vente");
}

pub fn afficher_gerer_retour() {
    println!("3. Gérer les retours");
}

pub fn afficher_consulter_stock() {
    println!("4. Consulter l'état du stock des produits");
}

pub fn afficher_quitter() {
    println!("5. Quitter");
}

pub fn demander_choix() -> String {
    print!("Choix : ");
    io::stdout().flush().expect("Erreur lors du flush");
    let mut choix = String::new();
    io::stdin()
        .read_line(&mut choix)
        .expect("Erreur lors de la lecture");
    choix.trim().to_string()
}
