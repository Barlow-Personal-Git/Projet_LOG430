use crate::controllers::{
    achat_controller, consulter_controller, recherche_controller, retour_controller,
};
use crate::views::menu_view;

pub async fn menu_principal() {
    loop {
        menu_view::afficher_choix();
        menu_view::afficher_recherche();
        menu_view::afficher_achat();
        menu_view::afficher_gerer_retour();
        menu_view::afficher_consulter_stock();
        menu_view::afficher_quitter();

        let choix = menu_view::demander_choix();

        match choix.as_str() {
            "1" => recherche_controller::menu_recherche(),
            "2" => achat_controller::menu_achat(),
            "3" => retour_controller::menu_retour()
                .expect("Erreur de connecter sur la table de transaction"),
            "4" => consulter_controller::consulter_liste_produit(),
            "5" => break,
            _ => println!("Choix invalide."),
        }
    }
}
