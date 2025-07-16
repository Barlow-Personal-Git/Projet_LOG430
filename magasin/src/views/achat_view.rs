use std::io::{self, Write};

pub fn afficher_choix() {
    println!("\nVeuillez sélectionner une des choix");
}

pub fn afficher_achat() {
    println!("1. Ajouter des produits");
}

pub fn afficher_produits() {
    println!("2. Consulter la vente");
}

pub fn afficher_quitter() {
    println!("3. Retour");
}

pub fn demander_choix() -> String {
    print!("Choix : ");
    io::stdout().flush().unwrap();
    let mut choix = String::new();
    io::stdin().read_line(&mut choix).unwrap();
    choix.trim().to_string()
}

pub fn afficher_ajouter_produit() {
    println!(
        "\nVeuillez inscrire l'identifiant du produit ou taper 'Back' pour retourner en arrière"
    );
}

pub fn demander_choix_ajouter() -> String {
    print!("Produit ID : ");
    io::stdout().flush().unwrap();
    let mut produit_id = String::new();
    io::stdin().read_line(&mut produit_id).unwrap();
    produit_id.trim().to_string()
}

pub fn demande_quantite() -> String {
    print!("Quantites : ");
    io::stdout().flush().unwrap();
    let mut quantite = String::new();
    io::stdin().read_line(&mut quantite).unwrap();
    quantite.trim().to_string()
}

pub fn afficher_zero() {
    println!("\nVeuillez recommencer et ajouter une quantite supérieure ou égale à 1");
}

pub fn afficher_insuffisant() {
    println!("Il n'a pas suffisament de produit disponible");
}

pub fn afficher_inventaire_pas_enregistrer() {
    println!("Aucun inventaire trouvé pour ce produit.");
}

pub fn afficher_aucun_produit() {
    println!("Aucun produit");
}

pub fn afficher_ventes() {
    println!("---- Ventes ----");
}

pub fn afficher_produit_total(
    produit: &crate::models::produit::Produit,
    nbr: u32,
    produit_total: f64,
) {
    println!("- {} x{} = {:.2}$", produit.nom, nbr, produit_total);
}

pub fn afficher_total(total: f64) {
    println!("Total : {total:.2}$");
}

pub fn afficher_confirmer() {
    println!("1. Confirmer votre vente");
}

pub fn afficher_effacer() {
    println!("2. Recommencer la vente");
}

pub fn afficher_vente_confirmer() {
    println!("Vente confirmer");
}

pub fn afficher_produit_ajouter(produit: &crate::models::produit::Produit, nbr: u32) {
    println!("Ajouter à la vente : {} x{}", produit.nom, nbr);
}
