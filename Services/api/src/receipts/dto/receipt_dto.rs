use rocket::figment::value::Num;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ReceiptDto {
    pub id: Num,
    pub date: String,
    pub amount_cents: Num,
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
