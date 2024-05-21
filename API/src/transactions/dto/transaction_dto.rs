use rocket::figment::value::Num;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct TransactionDto {
    pub id: Num,
    pub type_: String,
    pub account_id: Num,
    pub booking_date: String,
    pub category: String,
    pub creditor_name: String,
    pub description: String,
    pub amount_cents: Num,
    pub currency: String
}
