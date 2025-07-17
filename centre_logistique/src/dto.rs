use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct NouveauInventaireDTO<'a> {
    #[schemars(example = "crate::examples::exemple_id_produit")]
    pub id_produit: i32,
    #[schemars(example = "crate::examples::exemple_category")]
    pub category: &'a str,
    #[schemars(example = "crate::examples::exemple_nbr")]
    pub nbr: i32,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct InventaireDTO<'a> {
    #[schemars(example = "crate::examples::exemple_magasin")]
    pub magasin: &'a str,
    pub inventaires: Vec<NouveauInventaireDTO<'a>>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct NouvelleTransactionDTO {
    #[schemars(example = "crate::examples::exemple_total")]
    pub total: f32,
    #[schemars(example = "crate::examples::exemple_created_date")]
    pub created_date: NaiveDateTime,
    #[schemars(example = "crate::examples::exemple_updated_date")]
    pub updated_date: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct TransactionDTO<'a> {
    pub magasin: &'a str,
    pub transactions: Vec<NouvelleTransactionDTO>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct NouveauMessageDTO<'a> {
    #[schemars(example = "crate::examples::exemple_magasin")]
    pub id_produit: i32,
    #[schemars(example = "crate::examples::exemple_message")]
    pub message: &'a str,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct MessageDTO<'a> {
    #[schemars(example = "crate::examples::exemple_magasin")]
    pub magasin: &'a str,
    pub messages: Vec<NouveauMessageDTO<'a>>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct TransactionProduitsDTO {
    #[schemars(example = "crate::examples::exemple_magasin")]
    pub magasin: String,
    pub transaction_produits: Vec<NouvelleTransactionProduitsDTO>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct NouvelleTransactionProduitsDTO {
    #[schemars(example = "crate::examples::exemple_id_transaction")]
    pub id_transaction: i32,
    #[schemars(example = "crate::examples::exemple_produits")]
    pub produits: Vec<NouvelleProduitsDTO>,
    #[schemars(example = "crate::examples::exemple_total")]
    pub total: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct NouvelleProduitsDTO {
    #[schemars(example = "crate::examples::exemple_nom")]
    pub nom: String,
    #[schemars(example = "crate::examples::exemple_prix")]
    pub prix: f32,
    #[schemars(example = "crate::examples::exemple_nbr")]
    pub nbr: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct InventairesFaibleDTO {
    #[schemars(example = "crate::examples::exemple_magasin")]
    pub nom: String,
    #[schemars(example = "crate::examples::exemple_nom")]
    pub produit_nom: String,
    #[schemars(example = "crate::examples::exemple_nbr")]
    pub inv_nbr: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct InventairesSurplusDTO {
    #[schemars(example = "crate::examples::exemple_magasin")]
    pub nom: String,
    #[schemars(example = "crate::examples::exemple_nom")]
    pub produit_nom: String,
    #[schemars(example = "crate::examples::exemple_nbr")]
    pub inv_nbr: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct TendancesHebdoDTO {
    #[schemars(example = "crate::examples::exemple_magasin")]
    pub magasin: String,
    #[schemars(example = "crate::examples::exemple_semaine")]
    pub semaine: String,
    #[schemars(example = "crate::examples::exemple_total")]
    pub total: f32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ProduitVenduDTO {
    #[schemars(example = "crate::examples::exemple_nom")]
    pub nom_produit: String,
    #[schemars(example = "crate::examples::exemple_nbr")]
    pub nbr_vendue: i32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InventaireRestantDTO {
    #[schemars(example = "crate::examples::exemple_nom")]
    pub nom_produit: String,
    #[schemars(example = "crate::examples::exemple_nbr")]
    pub nbr_inventaire: i32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct AlerteReapprovisionnementDTO {
    #[schemars(example = "crate::examples::exemple_magasin")]
    pub magasin: String,
    #[schemars(example = "crate::examples::exemple_nom")]
    pub produit: String,
    #[schemars(example = "crate::examples::exemple_nbr")]
    pub nbr: i32,
    #[schemars(example = "crate::examples::exemple_status")]
    pub status: String,
    #[schemars(example = "crate::examples::exemple_created_date")]
    pub date_creation: NaiveDateTime,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InventaireUpdateDTO {
    #[schemars(example = "crate::examples::exemple_id_produit")]
    pub id_produit: i32,
    #[schemars(example = "crate::examples::exemple_id_magasin")]
    pub id_magasin: i32,
    #[schemars(example = "crate::examples::exemple_nbr")]
    pub nbr: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ProduitUpdateDTO {
    #[schemars(example = "crate::examples::exemple_nom")]
    pub nom: String,
    #[schemars(example = "crate::examples::exemple_prix")]
    pub prix: f32,
    #[schemars(example = "crate::examples::exemple_description")]
    pub description: String,
}
