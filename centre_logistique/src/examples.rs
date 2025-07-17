use serde_json::{json, Value};

pub fn exemple_id_inventaire() -> i32 { 1 }
pub fn exemple_id_produit() -> i32 { 1 }
pub fn exemple_id_magasin() -> i32 { 1 }
pub fn exemple_category() -> String { "Breuvage".to_string() }
pub fn exemple_nbr() -> i32 { 1 }
pub fn exemple_id_message() -> i32 { 20}
pub fn exemple_message() -> String { "Test".to_string() }
pub fn exemple_nom() -> String { "Eau".to_string() }
pub fn exemple_prix() -> f32 { 1.00 }
pub fn exemple_description() -> String { "eau QC".to_string() }
pub fn exemple_id_reapprovisionnement() -> i32 { 100 }
pub fn exemple_status() -> String { "en cours".to_string() }
pub fn exemple_created_date() -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::parse_from_str("2023-07-16 15:30:00", "%Y-%m-%d %H:%M:%S").unwrap()
}
pub fn exemple_id_transaction_produit() -> i32 { 1 }
pub fn exemple_id_transaction() -> i32 { 1 }
pub fn exemple_total() -> f32 { 10.50 }
pub fn exemple_produits() -> Value {
    json!([
        {
            "id_produit": 1,
            "nom": "Eau",
            "prix": 1.0,
            "nbr": 3
        },
        {
            "id_produit": 2,
            "nom": "Café",
            "prix": 2.5,
            "nbr": 5
        }
    ])
}
pub fn exemple_updated_date() -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::parse_from_str("2023-07-16 15:30:00", "%Y-%m-%d %H:%M:%S").unwrap()
}
pub fn exemple_magasin() -> String { "Magasin 1".to_string() }
pub fn exemple_semaine() -> String { "2025-07-14".to_string() }
