use rocket::figment::value::Num;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct TransactionDto {
    pub id: i32,
    pub description: String,
    pub amount_cents: Num,
}

