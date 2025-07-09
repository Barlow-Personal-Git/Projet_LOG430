use rocket::serde::{Deserialize, Serialize };
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NouveauInventaireDTO<'a> {
    pub id_produit: i32,
    pub category: &'a str,
    pub nbr: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct InventaireDTO<'a> {
    pub magasin: &'a str,
    pub inventaires: Vec<NouveauInventaireDTO<'a>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NouvelleTransactionDTO {
    pub total: f32,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TransactionDTO<'a> {
    pub magasin: &'a str,
    pub transactions: Vec<NouvelleTransactionDTO>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NouveauMessageDTO<'a> {
    pub id_produit: i32,
    pub message: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MessageDTO<'a> {
    pub magasin: &'a str,
    pub messages: Vec<NouveauMessageDTO<'a>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TransactionProduitsDTO {
    pub magasin: String,
    pub transaction_produits: Vec<NouvelleTransactionProduitsDTO>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NouvelleTransactionProduitsDTO {
    pub id_transaction: i32,
    pub produits: Vec<NouvelleProduitsDTO>,
    pub total: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NouvelleProduitsDTO {
    pub nom: String,
    pub prix: f32,
    pub nbr: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InventairesFaibleDTO {
    pub nom: String,
    pub produit_nom: String,
    pub inv_nbr: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InventairesSurplusDTO {
    pub nom: String,
    pub produit_nom: String,
    pub inv_nbr: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TendancesHebdoDTO {
    pub magasin: String,
    pub semaine: String,
    pub total: f32,
}

#[derive(Serialize, Deserialize)]
pub struct ProduitVenduDTO {
    pub nom_produit: String,
    pub nbr_vendue: i32,
}

#[derive(Serialize, Deserialize)]
pub struct InventaireRestantDTO {
    pub nom_produit: String,
    pub nbr_inventaire: i32,
}

#[derive(Serialize, Deserialize)]
pub struct AlerteReapprovisionnementDTO {
    pub magasin: String,
    pub produit: String,
    pub nbr: i32,
    pub status: String,
    pub date_creation: NaiveDateTime,
}
