use rocket::figment::value::Num;
use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Serialize)]
pub struct TransactionDto {
    pub id: Num,
    #[serde(rename(serialize = "type"))]
    pub type_: String,
    pub account_id: Num,
    pub account_name: String,
    pub booking_date: String,
    pub category: String,
    pub creditor_name: String,
    pub description: String,
    pub amount_cents: Num,
    pub currency: String,
    pub amount_cents_in_ref_currency: Num,
    pub ref_currency: String,
    pub account_to: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct PatchTransactionDto {
    pub category: Option<String>,
    pub description: Option<String>,
    pub account_to: Option<i32>,
    #[serde(rename(deserialize = "type"))]
    pub type_: Option<String>
}