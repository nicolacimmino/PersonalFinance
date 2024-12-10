use diesel::table;

// Manually generated!
// Diesel doesn't seem to generate schema for views.
table! {
    budgets_overview (id) {
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
    alerts (id) {
        id -> Uuid,
        category -> Text,
        message -> Text,
        item -> Text,
        item_id -> Text,
        level -> Text,
    }
}

diesel::table! {
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

diesel::table! {
    application_transactions (id) {
        id -> Int4,
        #[max_length = 16]
        movement_type -> Varchar,
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
        account_to_name -> Varchar
    }
}
