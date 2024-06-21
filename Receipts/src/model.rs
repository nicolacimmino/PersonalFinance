use alloc::string::String;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Associations, Insertable, Queryable, Selectable};

#[allow(dead_code)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::receipts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Receipt {
    pub id: i32,
    pub date: NaiveDateTime,
    pub amount_cents: i32,
    pub currency: String,
    pub ext_id: String,
    pub merchant_name: String,
    pub merchant_address: String,
    pub original_data: String,
    pub scan_file_name: String,
}

#[allow(dead_code)]
#[derive(Queryable, Selectable, Associations, Debug)]
#[diesel(belongs_to(Receipt, foreign_key = receipt_id))]
#[diesel(table_name = crate::schema::receipts_line_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReceiptLineItem {
    pub id: i32,
    pub receipt_id: i32,
    pub quantity: BigDecimal,
    pub unit_price_cents: i32,
    pub amount_cents: i32,
    pub description: String,
    pub raw_text: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::receipts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewReceipt<'a> {
    pub date: &'a NaiveDateTime,
    pub amount_cents: &'a i32,
    pub currency: &'a str,
    pub ext_id: &'a str,
    pub merchant_name: &'a str,
    pub merchant_address: &'a str,
    pub original_data: &'a str,
    pub scan_file_name: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::receipts_line_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewReceiptLineItem<'a> {
    pub receipt_id: &'a i32,
    pub quantity: &'a BigDecimal,
    pub unit_price_cents: &'a i32,
    pub amount_cents: &'a i32,
    pub description: &'a String,
    pub raw_text: &'a String,
}
