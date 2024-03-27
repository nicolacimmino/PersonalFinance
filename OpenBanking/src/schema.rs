// @generated automatically by Diesel CLI.

diesel::table! {
    ob_accounts (id) {
        id -> Uuid,
        #[max_length = 128]
        provider -> Varchar,
        #[max_length = 128]
        provider_account_id -> Varchar,
    }
}
