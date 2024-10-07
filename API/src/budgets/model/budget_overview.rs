use chrono::NaiveDate;
use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::manual_schema::budgets_overview)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BudgetOverview {
    pub id: i32,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub code: String,
    pub description: String,
    pub active: bool,
    pub currency: String,
    pub amount_cents: i32,
    pub spent_cents_eur: i32,
    pub spent_cents: i32,
    pub transactions: i32,
}