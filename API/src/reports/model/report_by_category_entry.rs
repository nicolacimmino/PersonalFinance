use diesel::sql_types::Text;
use diesel::sql_types::BigInt;
use diesel::{QueryableByName};

#[derive(QueryableByName, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReportByCategoryEntry {
    #[diesel(sql_type = Text)]
    pub category: String,
    #[diesel(sql_type = Text)]
    pub currency: String,
    #[diesel(sql_type = BigInt)]
    pub amount_cents: i64,
    #[diesel(sql_type = BigInt)]
    pub transactions_count: i64,
}

