use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::models::client::Client;
use crate::models::produit::Produit;

#[derive(Debug)]
pub struct ClientSession {
    client: Option<Client>,
    produit_ventes: HashMap<i32, (Produit, u32)>,
    total: f64,
}

pub static CLIENT_SESSION: Lazy<Mutex<ClientSession>> = Lazy::new(|| {
    Mutex::new(ClientSession {
        client: None,
        produit_ventes: HashMap::new(),
        total: 0.0,
    })
});

impl ClientSession {
    pub fn set_client(&mut self, client: Client) {
        self.client = Some(client);
    }

    pub fn get_client(&self) -> Option<Client> {
        self.client.clone()
    }

    pub fn add_produit(&mut self, produit: Produit, nbr: u32) {
        self.produit_ventes
            .entry(produit.id_produit)
            .and_modify(|e| e.1 += nbr)
            .or_insert((produit, nbr));
    }

    pub fn get_produits(&self) -> Vec<(Produit, u32)> {
        self.produit_ventes.values().cloned().collect()
    }

    pub fn clear_vente(&mut self) {
        self.produit_ventes.clear();
        self.total = 0.0;
    }
}
