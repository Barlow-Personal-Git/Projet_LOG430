use std::io::{self, Write};

pub fn afficher_bienvenue_magasin() {
    println!("Bienvenue au magasin Barlow Supermarket!");
}

pub fn demander_nom() -> String {
    print!("\nVeuillez inscrire votre nom : ");
    io::stdout().flush().expect("Erreur lors du flush");
    let mut nom = String::new();
    io::stdin()
        .read_line(&mut nom)
        .expect("Erreur lors de la lecture");
    nom.trim().to_string()
}

pub fn afficher_nom_invalide() {
    println!("Nom invalide. Veuillez réessayer.");
}

pub fn afficher_bienvenue(nom: &str) {
    println!("Bienvenue {}", nom);
}
