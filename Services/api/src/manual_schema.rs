use diesel::table;

// Manually generated!
// Diesel doesn't seem to generate schema for views.
table! {
    application_budgets (id) {
        id -> Int4,
        from_date -> Date,
        to_date -> Date,
        code -> Text,
        description -> Text,
        active -> Bool,
        #[max_length = 3]
        currency -> Varchar,
        amount_cents -> Int4,
        spent_cents_eur -> Int4,
        spent_cents -> Int4,
        transactions -> Int4
    }
}

table! {
    application_alerts (id) {
        id -> Uuid,
        category -> Text,
        message -> Text,
        item -> Text,
        item_id -> Text,
        level -> Text,
    }
}

table! {
    application_accounts (id) {
        id -> Int4,
        #[max_length = 16]
        code -> Varchar,
        description -> Text,
        #[max_length = 3]
        currency -> Varchar,
        pri_transactions_src -> Text,
        #[max_length = 34]
        iban -> Varchar,
        #[max_length = 20]
        status -> Varchar,
        #[max_length = 20]
        asset_type -> Varchar,
        balance_cents -> Int4,
        balance_eur_cents -> Int4,
    }
}


table! {
    application_transactions (id) {
        id -> Int4,
        #[max_length = 16]
        transaction_type -> Varchar,
        account_id -> Int4,
        amount_cents -> Int4,
        category -> Text,
        creditor_name -> Text,
        description -> Text,
        booking_date -> Timestamp,
        value_date -> Timestamp,
        account_to -> Nullable<Int4>,
        amount_cents_eur  -> Int4,
        account_name -> Varchar,
        currency -> Varchar,
        #[max_length = 16]
        account_type -> Varchar,
        account_to_name -> Nullable<Varchar>,
        receipt_id -> Nullable<Int4>,
    }
}

table! {
    application_categories (id) {
        id -> Uuid,
        #[max_length = 128]
        code -> Varchar,
        #[max_length = 6]
        color -> Varchar,
        #[sql_name = "type"]
        #[max_length = 16]
        type_ -> Varchar,
        #[max_length = 10]
        discontinued -> Varchar,
    }
}

table! {
    application_receipts (id) {
        id -> Int4,
        date -> Timestamp,
        amount_cents -> Int4,
        currency -> Text,
        ext_id -> Text,
        merchant_name -> Text,
        merchant_address -> Text,
        scan_file_name -> Text,
        transaction_id -> Nullable<Int4>,
        transaction_category -> Nullable<Text>,
        transaction_amount_cents -> Nullable<Int4>,
        transaction_currency -> Nullable<Text>,
        account_code -> Nullable<Text>,
        account_description -> Nullable<Text>
        }
    }

// This is the raw transaction. We really use it just when inserting to get back the ID.
// TODO: consider changing to raw insert query and just fetch the ID.
table! {
    raw.transactions (id) {
        id -> Int4,
        #[sql_name = "type"]
        #[max_length = 16]
        type_ -> Varchar,
        account_id -> Int4,
        amount_cents -> Int4,
        category -> Text,
        creditor_name -> Text,
        description -> Text,
        booking_date -> Timestamp,
        value_date -> Timestamp,
        account_to -> Nullable<Int4>,
    }
}

