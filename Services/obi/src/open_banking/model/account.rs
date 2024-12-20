use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Identifiable, PartialEq, Selectable, Associations, Debug)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub code: String,
    pub description: String,
    pub currency: String,
    pub pri_transactions_src: String,
    pub status: String,
    pub iban: String,
}