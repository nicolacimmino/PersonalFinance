use chrono::{NaiveDate, NaiveTime};
use rocket::{get, patch, post};
use rocket::figment::value::Num;
use rocket::http::Status;
use rocket::response::{content, status};

use crate::guard::ApiKey;
use crate::transactions::dto::{CreateTransactionDto, PatchTransactionDto, TransactionDto};
use crate::transactions::model::{ApplicationTransaction, NewTransaction};
use crate::transactions::service::TransactionsService;

#[get("/transactions?<category>&<account>")]
pub fn get_transactions(
    _key: ApiKey<'_>,
    category: Option<String>,
    account: Option<i32>,
) -> status::Custom<content::RawJson<String>> {
    let mut transactions_service = TransactionsService {};

    let mut dtos: Vec<TransactionDto> = Vec::new();

    for transaction in transactions_service.get_transactions(category, account) {
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

    // Fetch the full object so we get it enriched with the ApplicationTransaction fields
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
    //
    // if patch_transaction_dto.type_.is_some() {
    //     let new_type = patch_transaction_dto.type_.clone().unwrap_or("".to_string());
    //
    //     if !vec!["TRANSFER".to_string(), "INCOME".to_string(), "EXPENSE".to_string()].contains(&new_type) {
    //         panic!("Invalid type.")
    //     }
    //
    //     if !new_type.eq(&"TRANSFER".to_string()) {
    //         let transaction = transactions_service.get_transaction(id);
    //         if transaction.amount_cents > 0 && new_type != "INCOME".to_string() {
    //             panic!("Positive amount is INCOME")
    //         }
    //         if transaction.amount_cents <= 0 && new_type != "EXPENSE".to_string() {
    //             panic!("Negative or zero amount is  EXPENSE")
    //         }
    //     }
    //
    //     transactions_service.update_transaction_type(
    //         id,
    //         &new_type,
    //     );
    // }
    //
    // if patch_transaction_dto.category.is_some() {
    //     transactions_service.update_transaction_category(
    //         id,
    //         patch_transaction_dto.category.clone().unwrap(),
    //     );
    // }
    //
    // if patch_transaction_dto.description.is_some() {
    //     transactions_service.update_transaction_description(
    //         id,
    //         patch_transaction_dto.description.clone().unwrap(),
    //     );
    // }
    //
    // if patch_transaction_dto.account_to.is_some() {
    //     transactions_service.update_transaction_account_to(
    //         id,
    //         patch_transaction_dto.account_to.clone().unwrap(),
    //     );
    // }

    let dto = build_transaction_dto(transactions_service.get_transaction(id));

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dto).expect("Serialization Failed")),
    )
}

fn build_transaction_dto(transaction: ApplicationTransaction) -> TransactionDto {
    return TransactionDto {
        id: Num::from(transaction.id),
        type_: transaction.movement_type.to_owned(),
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
    };
}
