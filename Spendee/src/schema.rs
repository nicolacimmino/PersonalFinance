
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
    }
}


