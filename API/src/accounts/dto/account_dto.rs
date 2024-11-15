use rocket::figment::value::Num;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct AccountDto {
    pub id: Num,
    pub code: String,
    pub description: String,
    pub currency: String,
    pub balance_cents: Num,
    pub balance_cents_in_ref_currency: Num,
    pub ref_currency: String,
    pub iban: String,
    pub status: String,
    #[serde(rename(serialize = "type"))]
    pub type_: String,
    pub can_create_transactions: String
}
