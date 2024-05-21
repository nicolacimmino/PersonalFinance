use rocket::figment::value::Num;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct AccountDto {
    pub id: Num,
    pub code: String,
    pub description: String,
    pub currency: String,
    pub balance_cents: Num,
}
