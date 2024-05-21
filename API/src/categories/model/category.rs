use diesel::{Queryable};

#[derive(Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub category: String,
    pub type_: String,
}