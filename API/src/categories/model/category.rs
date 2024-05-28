use diesel::{Queryable};

#[derive(Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub category: String,
    pub type_: String,
    pub currency: Option<String>,
    pub amount_cents: Option<i32>,
}