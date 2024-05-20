use diesel::{Queryable, Selectable};
use chrono::NaiveDateTime;

#[derive(Queryable, PartialEq, Selectable, Debug)]
#[diesel(table_name = crate::schema::transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Transaction {
    pub id: i32,
    pub type_: String,
    pub account_id: i32,
    pub amount_cents: i32,
    pub category: String,
    pub creditor_name: String,
    pub description: String,
    pub booking_date: NaiveDateTime,
    pub value_date: NaiveDateTime,
}