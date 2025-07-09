// @generated automatically by Diesel CLI.

diesel::table! {
    inventaires (id_inventaire) {
        id_inventaire -> Int4,
        id_produit -> Int4,
        id_magasin -> Int4,
        category -> Varchar,
        nbr -> Int4,
    }
}

diesel::table! {
    magasins (id_magasin) {
        id_magasin -> Int4,
        nom -> Varchar,
    }
}

diesel::table! {
    messages (id_message) {
        id_message -> Int4,
        id_produit -> Int4,
        id_magasin -> Int4,
        message -> Text,
    }
}

diesel::table! {
    produits (id_produit) {
        id_produit -> Int4,
        nom -> Varchar,
        prix -> Float4,
        description -> Varchar,
    }
}

diesel::table! {
    transaction_produits (id_transaction_produit) {
        id_transaction_produit -> Int4,
        id_transaction -> Int4,
        id_magasin -> Int4,
        produits -> Jsonb,
        total -> Float4,
    }
}

diesel::table! {
    transactions (id_transaction) {
        id_transaction -> Int4,
        id_magasin -> Int4,
        total -> Float4,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    reapprovisionnements (id_reapprovisionnement) {
        id_reapprovisionnement -> Int4,
        id_magasin -> Int4,
        id_produit -> Int4,
        nbr -> Int4,
        status -> Varchar,
        created_date -> Timestamp,
    }
}

diesel::joinable!(inventaires -> magasins (id_magasin));
diesel::joinable!(inventaires -> produits (id_produit));
diesel::joinable!(messages -> magasins (id_magasin));
diesel::joinable!(messages -> produits (id_produit));
diesel::joinable!(transaction_produits -> magasins (id_magasin));
diesel::joinable!(transactions -> magasins (id_magasin));
diesel::joinable!(reapprovisionnements -> magasins (id_magasin));
diesel::joinable!(reapprovisionnements -> produits (id_produit));

diesel::allow_tables_to_appear_in_same_query!(
    inventaires,
    magasins,
    messages,
    produits,
    reapprovisionnements,
    transaction_produits,
    transactions,
);
