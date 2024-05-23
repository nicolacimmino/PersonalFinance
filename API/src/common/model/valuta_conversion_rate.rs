use bigdecimal::BigDecimal;
use diesel::QueryableByName;
use diesel::sql_types::{Text, Numeric};

#[derive(QueryableByName, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ValutaConversionRate {
    #[diesel(sql_type = Text)]
    pub valuta_from: String,
    #[diesel(sql_type = Text)]
    pub valuta_to: String,
    #[diesel(sql_type = Numeric)]
    pub factor: BigDecimal,
}

