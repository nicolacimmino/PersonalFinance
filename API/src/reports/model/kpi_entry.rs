use diesel::sql_types::Text;
use diesel::sql_types::Integer;
use diesel::{QueryableByName};

#[derive(QueryableByName, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct KpiEntry {
    #[diesel(sql_type = Text)]
    pub label: String,
    #[diesel(sql_type = Integer)]
    pub amount_cents: i32,
}

