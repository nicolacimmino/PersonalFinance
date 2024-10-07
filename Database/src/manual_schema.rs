// Manually generated!
// Disel doesn't seem to generate schema for views.
table! {
    budgets_overview (id) {
        from_date -> Date,
        to_date -> Date,
        code -> Text,
        description -> Text,
        active -> Boolean,
        #[max_length = 3]
        currency -> Varchar,
        amount_cents -> Int4,
        spent_cents_eur -> Int4,
        spent_cents -> Int4,
        transactions -> Int4
    }
}