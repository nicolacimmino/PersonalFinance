// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        #[max_length = 16]
        code -> Varchar,
        description -> Text,
        #[max_length = 3]
        currency -> Varchar,
    }
}

diesel::table! {
    ob_accounts (id) {
        id -> Uuid,
        #[max_length = 128]
        provider -> Varchar,
        #[max_length = 128]
        provider_account_id -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        account_id -> Int4,
    }
}

diesel::table! {
    ob_transactions (id) {
        id -> Uuid,
        ob_account_id -> Uuid,
        transaction_id -> Text,
        booking_date -> Text,
        value_date -> Text,
        booking_date_time -> Text,
        transaction_amount_cents -> Int4,
        #[max_length = 3]
        transaction_amount_currency -> Varchar,
        creditor_name -> Text,
        debtor_name -> Text,
        debtor_account_iban -> Text,
        remittance_information_unstructured -> Text,
        balance_after_transaction_amount_cents -> Int4,
        #[max_length = 3]
        balance_after_transaction_currency -> Varchar,
        balance_after_transaction_type -> Text,
        internal_transaction_id -> Text,
        transformed_transaction_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sp_accounts (id) {
        id -> Uuid,
        #[max_length = 128]
        wallet -> Varchar,
        account_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sp_transactions (id) {
        id -> Uuid,
        date -> Varchar,
        wallet -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        category -> Varchar,
        amount -> Varchar,
        currency -> Varchar,
        note -> Varchar,
        labels -> Varchar,
        author -> Varchar,
        transformed_transaction_id -> Nullable<Int4>,
        sp_account_id -> Uuid,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        date -> Timestamp,
        #[sql_name = "type"]
        #[max_length = 16]
        type_ -> Varchar,
        account_id -> Int4,
        amount_cents -> Int4,
        category -> Text,
        creditor_name -> Text,
        description -> Text,
    }
}

diesel::joinable!(ob_accounts -> accounts (account_id));
diesel::joinable!(ob_transactions -> ob_accounts (ob_account_id));
diesel::joinable!(ob_transactions -> transactions (transformed_transaction_id));
diesel::joinable!(sp_accounts -> accounts (account_id));
diesel::joinable!(sp_transactions -> sp_accounts (sp_account_id));
diesel::joinable!(sp_transactions -> transactions (transformed_transaction_id));
diesel::joinable!(transactions -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    ob_accounts,
    ob_transactions,
    sp_accounts,
    sp_transactions,
    transactions,
);
