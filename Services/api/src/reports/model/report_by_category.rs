use diesel::sql_types::Text;
use diesel::sql_types::Integer;
use diesel::{QueryableByName};

#[allow(dead_code)]
#[derive(QueryableByName, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReportByCategory {
    #[diesel(sql_type = Text)]
    pub category: String,
    #[diesel(sql_type = Text)]
    pub category_type: String,
    #[diesel(sql_type = Integer)]
    pub amount_cents_eur: i32,
    #[diesel(sql_type = Integer)]
    pub transactions_count: i32,
}

