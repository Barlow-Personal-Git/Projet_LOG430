use cached::TimedCache;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::models::produit::Produit;
use crate::models::inventaire::Inventaire;

pub static PRODUITS_CACHE: Lazy<Mutex<TimedCache<i32, Produit>>> = Lazy::new(|| {
    Mutex::new(TimedCache::with_lifespan(60))
});

pub static INVENTAIRES_CACHE: Lazy<Mutex<TimedCache<i32, Inventaire>>> = Lazy::new(|| {
    Mutex::new(TimedCache::with_lifespan(60))
});

pub static INVENTAIRES_STRING_CACHE: Lazy<Mutex<TimedCache<String, Vec<Inventaire>>>> = Lazy::new(|| {
    Mutex::new(TimedCache::with_lifespan(60))
});