use diesel::sql_types::Text;
use diesel::sql_types::BigInt;
use diesel::{QueryableByName};

#[derive(QueryableByName)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReportByCategoryEntry {
    #[diesel(sql_type = Text)]
    pub category: String,
    #[diesel(sql_type = Text)]
    pub currency: String,
    #[diesel(sql_type = Text)]
    pub type_: String,
    #[diesel(sql_type = BigInt)]
    pub total_cents: i64,
}