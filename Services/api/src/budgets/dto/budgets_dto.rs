use rocket::figment::value::Num;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct BudgetOverviewDto {
    pub id: Num,
    pub code: String,
    pub from_date: String,
    pub to_date: String,
    pub description: String,
    pub active: bool,
    pub currency: String,
    pub amount_cents: i32,
    // DTO Is ready for future, internally ref currency is always euros anyway/
    pub spent_cents_in_ref_currency: i32,
    pub spent_cents: i32,
    pub transactions: i32,
}
