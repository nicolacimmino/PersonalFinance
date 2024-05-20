use rocket::get;
use rocket::figment::value::Num;
use rocket::http::Status;
use rocket::response::{content, status};
use crate::schema::transactions::dsl::transactions;
use crate::transactions::dto::TransactionDto;
use crate::guard::ApiKey;
use crate::transactions::service::TransactionsService;


#[get("/transactions")]
pub fn get_transactions(_key: ApiKey<'_>) -> status::Custom<content::RawJson<String>> {
    let mut transactions_service = TransactionsService {};

    let mut dtos: Vec<TransactionDto> = Vec::new();

    for transaction in transactions_service.get_transactions() {
        dtos.push(TransactionDto {
            id: Num::from(transaction.id),
            type_: transaction.type_.to_owned(),
            account_id: Num::from(transaction.account_id),
            booking_date: transaction.booking_date.to_string(),
            category: transaction.category.to_owned(),
            creditor_name: transaction.creditor_name.to_owned(),
            description: transaction.description.to_owned(),
            amount_cents: Num::from(transaction.amount_cents),
        })
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}

#[get("/transactions/<account_id>")]
pub fn get_transactions_for_account(_key: ApiKey<'_>, account_id: i32) -> status::Custom<content::RawJson<String>> {
    let mut transactions_service = TransactionsService {};

    let mut dtos: Vec<TransactionDto> = Vec::new();

    for transaction in transactions_service.get_transactions_for_account(account_id) {
        dtos.push(TransactionDto {
            id: Num::from(transaction.id),
            type_: transaction.type_.to_owned(),
            account_id: Num::from(transaction.account_id),
            booking_date: transaction.booking_date.to_string(),
            category: transaction.category.to_owned(),
            creditor_name: transaction.creditor_name.to_owned(),
            description: transaction.description.to_owned(),
            amount_cents: Num::from(transaction.amount_cents),
        })
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}
