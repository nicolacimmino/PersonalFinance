use chrono::NaiveDateTime;
use diesel::{QueryableByName, Selectable};

#[derive(QueryableByName, Selectable, Debug)]
#[diesel(table_name = crate::manual_schema::application_receipts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ApplicationReceipt {
    pub id: i32,
    pub date: NaiveDateTime,
    pub amount_cents: i32,
    pub currency: String,
    pub ext_id: String,
    pub merchant_name: String,
    pub merchant_address: String,
    pub scan_file_name: String,
    pub transaction_id: Option<i32>,
    pub transaction_category: Option<String>,
    pub transaction_amount_cents: Option<i32>,
    pub transaction_currency: Option<String>,
    pub account_code: Option<String>,
    pub account_description: Option<String>,
}