// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id_client) {
        id_client -> Int4,
        nom -> Varchar,
        role -> Varchar,
    }
}

diesel::table! {
    inventaires (id_inventaire) {
        id_inventaire -> Int4,
        id_produit -> Int4,
        category -> Varchar,
        nbr -> Int4,
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
        id_produit -> Int4,
        nbr -> Int4,
        total -> Float4,
    }
}

diesel::table! {
    transactions (id_transaction) {
        id_transaction -> Int4,
        id_client -> Int4,
        total -> Float4,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::joinable!(inventaires -> produits (id_produit));
diesel::joinable!(transaction_produits -> produits (id_produit));
diesel::joinable!(transaction_produits -> transactions (id_transaction));
diesel::joinable!(transactions -> clients (id_client));

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    inventaires,
    produits,
    transaction_produits,
    transactions,
);
