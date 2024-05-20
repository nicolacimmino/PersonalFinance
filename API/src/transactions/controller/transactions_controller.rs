use rocket::get;
use rocket::figment::value::Num;
use rocket::http::Status;
use rocket::response::{content, status};
use crate::transactions::dto::TransactionDto;
use crate::transactions::guard::ApiKey;
use crate::transactions::service::TransactionsService;


#[get("/api/transactions")]
pub fn get_transactions(_key: ApiKey<'_>) -> status::Custom<content::RawJson<String>> {
    let mut transactions_service = TransactionsService {};

    let mut dtos: Vec<TransactionDto> = Vec::new();

    for transaction in transactions_service.get_transactions() {
        dtos.push(TransactionDto {
            id: transaction.id,
            description: transaction.description.to_owned(),
            amount_cents: Num::from(transaction.amount_cents),
        })
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}
