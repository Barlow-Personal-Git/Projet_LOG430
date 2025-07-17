use crate::models::magasin::Magasin;
use crate::models::produit::Produit;
use crate::schema::reapprovisionnements;
use chrono::NaiveDateTime;
use diesel::{Associations, Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize, Clone, JsonSchema)]
#[diesel(belongs_to(Magasin, foreign_key = id_magasin))]
#[diesel(belongs_to(Produit, foreign_key = id_produit))]
#[serde(crate = "rocket::serde")]
pub struct Reapprovisionnement {
    #[schemars(example = "crate::examples::exemple_id_reapprovisionnement")]
    pub id_reapprovisionnement: i32,
    #[schemars(example = "crate::examples::exemple_id_produit")]
    pub id_produit: i32,
    #[schemars(example = "crate::examples::exemple_id_magasin")]
    pub id_magasin: i32,
    #[schemars(example = "crate::examples::exemple_nbr")]
    pub nbr: i32,
    #[schemars(example = "crate::examples::exemple_status")]
    pub status: String,
    #[schemars(example = "crate::examples::exemple_created_date")]
    pub created_date: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = reapprovisionnements)]
pub struct NouveauReapprovisionnement {
    #[schemars(example = "crate::examples::exemple_id_produit")]
    pub id_produit: i32,
    #[schemars(example = "crate::examples::exemple_id_magasin")]
    pub id_magasin: i32,
    #[schemars(example = "crate::examples::exemple_nbr")]
    pub nbr: i32,
    #[schemars(example = "crate::examples::exemple_status")]
    pub status: String,
    #[schemars(example = "crate::examples::exemple_created_date")]
    pub created_date: NaiveDateTime,
}
