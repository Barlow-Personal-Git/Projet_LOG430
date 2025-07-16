pub fn afficher_titre() {
    println!("---Inventaires---");
}

pub fn afficher_inventaire(
    produit: &crate::models::produit::Produit,
    inventaire: &crate::models::inventaire::Inventaire,
) {
    println!(
        "- Nom : {} | Prix : {:.2} | Catégorie : {} | Quantité : {}",
        produit.nom, produit.prix, inventaire.category, inventaire.nbr
    );
}
