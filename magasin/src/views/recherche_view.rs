use std::io::{self, Write};

pub fn afficher_choix() {
    println!("\nVeuillez sélectionner une des choix");
}

pub fn afficher_identifiant() {
    println!("1. Par identifiant");
}

pub fn afficher_nom() {
    println!("2. Par nom");
}

pub fn afficher_categorie() {
    println!("3. Par catégorie");
}

pub fn afficher_quitter() {
    println!("4. Quitter");
}

/// Demande le choix à l'utilisateur et retourne la chaîne saisie
pub fn demander_choix() -> String {
    print!("Choix : ");
    io::stdout().flush().unwrap();

    let mut choix = String::new();
    io::stdin()
        .read_line(&mut choix)
        .expect("Erreur de lecture");
    choix.trim().to_string()
}

pub fn afficher_recherche_choix(recherche: &str) {
    match recherche {
        "nom" => {
            println!("\nInscrivez le nom du produit ou tapez 'Back' pour retourner en arrière")
        }
        "id" => println!("\nInscrivez l'id du produit ou tapez 'Back' pour retourner en arrière"),
        "categorie" => println!(
            "\nInscrivez la catégorie du produit ou tapez 'Back' pour retourner en arrière"
        ),
        _ => (),
    }
}

/// Demande une valeur selon l'option et retourne la chaîne saisie
pub fn demander_recherche_choix(recherche: &str) -> String {
    match recherche {
        "nom" => {
            print!("Nom du produit : ");
        }
        "id" => {
            print!("Id du produit : ");
        }
        "categorie" => {
            print!("Catégorie du produit : ");
        }
        _ => (),
    }
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");
    input.trim().to_string()
}

pub fn afficher_titre_liste() {
    println!("---Liste de produits---");
}

pub fn afficher_titre_produit() {
    println!("---Produit---");
}

/// Affiche un produit, structure attendue avec au moins ces champs : id_produit, nom, prix
pub fn afficher_produit(p: &crate::models::produit::Produit) {
    println!(
        "- ID : {} | Nom : {} | Prix : ({} $)",
        p.id_produit, p.nom, p.prix
    );
}

pub fn afficher_indisponible() {
    println!("Aucun produit trouvé");
}

pub fn afficher_erreur() {
    println!("Veuillez réessayer de nouveau!");
}
