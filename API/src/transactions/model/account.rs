use diesel::{Queryable, Selectable};

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub code: String,
    pub description: String,
    pub currency: String,
}