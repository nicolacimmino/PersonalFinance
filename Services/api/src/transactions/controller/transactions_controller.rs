use chrono::{Datelike, NaiveDate, NaiveTime, Utc};
use rocket::{FromForm, get, patch, post};
use rocket::figment::value::Num;
use rocket::http::Status;
use rocket::response::{content, status};

use crate::guard::ApiKey;
use crate::transactions::dto::{CreateTransactionDto, PatchTransactionDto, TransactionDto};
use crate::transactions::model::{ApplicationTransaction, NewTransaction};
use crate::transactions::service::TransactionsService;

#[get("/transactions?<params..>")]
pub fn get_transactions(
    _key: ApiKey<'_>,
    params: GetTransactionsParameters,
) -> status::Custom<content::RawJson<String>> {
    let mut transactions_service = TransactionsService {};

    let year = Utc::now().naive_utc().date().year();

    let date_from = NaiveDate::parse_from_str(
        &*params.date_from.unwrap_or(format!("{}-01-01", year)), "%Y-%m-%d",
    ).expect("Invalid date_from");

    let date_to = NaiveDate::parse_from_str(
        &*params.date_to.unwrap_or(format!("{}-12-31", year)), "%Y-%m-%d",
    ).expect("Invalid date_to");

    let mut dtos: Vec<TransactionDto> = Vec::new();

    for transaction in transactions_service.get_transactions(
        params.category,
        params.account,
        date_from,
        date_to
    ) {
        dtos.push(build_transaction_dto(transaction))
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}

#[get("/transactions/<id>")]
pub fn get_transaction(_key: ApiKey<'_>, id: i32) -> status::Custom<content::RawJson<String>> {
    let mut transactions_service = TransactionsService {};
    let transaction = transactions_service.get_transaction(id);

    let dto = build_transaction_dto(transaction);

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dto).expect("Serialization Failed")),
    )
}

#[post("/transactions", format = "application/json", data = "<create_transaction_dto>")]
pub fn create_transaction(_key: ApiKey<'_>, create_transaction_dto: rocket::serde::json::Json<CreateTransactionDto>) -> status::Custom<content::RawJson<String>> {
    let mut transactions_service = TransactionsService {};

    let id = transactions_service.create_transaction(NewTransaction {
        type_: create_transaction_dto.type_.clone().unwrap_or("".to_string()).clone(),
        account_id: create_transaction_dto.account_id.clone(),
        amount_cents: create_transaction_dto.amount_cents.clone(),
        category: create_transaction_dto.category.clone().unwrap_or("".to_string()),
        creditor_name: create_transaction_dto.creditor_name.clone().unwrap_or("".to_string()),
        description: create_transaction_dto.description.clone().unwrap_or("".to_string()),
        booking_date: NaiveDate::parse_from_str(
            &*create_transaction_dto.booking_date.clone().unwrap_or("1970-01-01".to_string()), "%Y-%m-%d",
        ).unwrap().and_time(NaiveTime::default()),
        value_date: NaiveDate::parse_from_str(
            &*create_transaction_dto.value_date.clone().unwrap_or("1970-01-01".to_string()), "%Y-%m-%d",
        ).unwrap().and_time(NaiveTime::default()),
        account_to: create_transaction_dto.account_to.clone(),
    });

    // Fetch the full object so that we get it enriched with the ApplicationTransaction fields
    let mut transactions_service = TransactionsService {};
    let transaction = transactions_service.get_transaction(id);

    let dto = build_transaction_dto(transaction);

    status::Custom(Status::Created, content::RawJson(
        serde_json::to_string(&dto).expect("Serialization Failed")),
    )
}

#[patch("/transactions/<id>", format = "application/json", data = "<patch_transaction_dto>")]
pub fn patch_transaction(_key: ApiKey<'_>, id: i32, patch_transaction_dto: rocket::serde::json::Json<PatchTransactionDto>) -> status::Custom<content::RawJson<String>> {
    let mut transactions_service = TransactionsService {};

    transactions_service.update_transaction(
        id,
        &patch_transaction_dto.category,
        &patch_transaction_dto.type_,
        &patch_transaction_dto.description,
        &patch_transaction_dto.account_to,
    );

    let dto = build_transaction_dto(transactions_service.get_transaction(id));

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dto).expect("Serialization Failed")),
    )
}

fn build_transaction_dto(transaction: ApplicationTransaction) -> TransactionDto {
    return TransactionDto {
        id: Num::from(transaction.id),
        type_: transaction.transaction_type.to_owned(),
        account_id: Num::from(transaction.account_id),
        account_name: transaction.account_name,
        booking_date: transaction.booking_date.to_string(),
        category: transaction.category.to_owned(),
        creditor_name: transaction.creditor_name.to_owned(),
        description: transaction.description.to_owned(),
        amount_cents: Num::from(transaction.amount_cents),
        currency: transaction.currency,
        amount_cents_in_ref_currency: Num::from(transaction.amount_cents_eur),
        ref_currency: "EUR".to_string(),
        account_to: transaction.account_to,
        account_to_name: transaction.account_to_name,
        receipt_id: transaction.receipt_id,
    };
}

#[derive(FromForm)]
pub struct GetTransactionsParameters {
    category: Option<String>,
    account: Option<i32>,
    date_from: Option<String>,
    date_to: Option<String>,
}