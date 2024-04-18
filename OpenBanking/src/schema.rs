diesel::table! {
    ob_accounts (id) {
        id -> Uuid,
        #[max_length = 128]
        provider -> Varchar,
        #[max_length = 128]
        provider_account_id -> Varchar,
        #[max_length = 128]
        name -> Varchar,
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
    }
}

diesel::joinable!(ob_transactions -> ob_accounts (ob_account_id));

diesel::allow_tables_to_appear_in_same_query!(
    ob_accounts,
    ob_transactions,
);
