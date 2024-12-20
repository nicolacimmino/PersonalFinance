use diesel::{Identifiable, Insertable, Queryable, QueryableByName, Selectable};
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, PartialEq, Selectable, Debug)]
#[diesel(table_name = crate::manual_schema::transactions)]
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
    pub account_to: Option<i32>,
}

#[derive(QueryableByName, Identifiable, PartialEq, Selectable, Debug)]
#[diesel(table_name = crate::manual_schema::application_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ApplicationTransaction {
    pub id: i32,
    pub movement_type: String,
    pub account_id: i32,
    pub amount_cents: i32,
    pub category: String,
    pub creditor_name: String,
    pub description: String,
    pub booking_date: NaiveDateTime,
    pub value_date: NaiveDateTime,
    pub account_to: Option<i32>,
    pub amount_cents_eur: i32,
    pub account_name: String,
    pub currency: String,
    pub account_type: String,
    pub account_to_name: Option<String>,
    pub receipt_id: Option<i32>,
}


#[derive(Queryable, PartialEq, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::manual_schema::transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTransaction {
    pub type_: String,
    pub account_id: i32,
    pub amount_cents: i32,
    pub category: String,
    pub creditor_name: String,
    pub description: String,
    pub booking_date: NaiveDateTime,
    pub value_date: NaiveDateTime,
    pub account_to: Option<i32>,
}