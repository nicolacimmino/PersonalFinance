use log::info;
use rocket::{get, patch};
use rocket::figment::value::Num;
use rocket::http::Status;
use rocket::response::{content, status};
use crate::common::ValutaConversionService;
use crate::establish_db_connection;
use crate::transactions::dto::{PatchTransactionDto, TransactionDto};
use crate::guard::ApiKey;
use crate::transactions::service::TransactionsService;


#[get("/transactions")]
pub fn get_transactions(_key: ApiKey<'_>) -> status::Custom<content::RawJson<String>> {
    let mut transactions_service = TransactionsService {};
    let mut valuta_conversion_service = ValutaConversionService::new(&mut establish_db_connection());

    let mut dtos: Vec<TransactionDto> = Vec::new();

    for (transaction, account) in transactions_service.get_transactions() {
        dtos.push(TransactionDto {
            id: Num::from(transaction.id),
            type_: transaction.type_.to_owned(),
            account_id: Num::from(transaction.account_id),
            account_name: account.description,
            booking_date: transaction.booking_date.to_string(),
            category: transaction.category.to_owned(),
            creditor_name: transaction.creditor_name.to_owned(),
            description: transaction.description.to_owned(),
            amount_cents: Num::from(transaction.amount_cents),
            currency: account.currency.to_owned(),
            amount_cents_in_ref_currency: Num::from(
                valuta_conversion_service.convert(
                    account.currency,
                    "EUR",
                    transaction.amount_cents)),
            ref_currency: "EUR".to_string(),
        })
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}

#[get("/accounts/<account_id>/transactions")]
pub fn get_transactions_for_account(_key: ApiKey<'_>, account_id: i32) -> status::Custom<content::RawJson<String>> {
    let mut transactions_service = TransactionsService {};
    let mut valuta_conversion_service = ValutaConversionService::new(&mut establish_db_connection());

    let mut dtos: Vec<TransactionDto> = Vec::new();

    for (transaction, account) in transactions_service.get_transactions_for_account(account_id) {
        dtos.push(TransactionDto {
            id: Num::from(transaction.id),
            type_: transaction.type_.to_owned(),
            account_id: Num::from(transaction.account_id),
            account_name: account.description,
            booking_date: transaction.booking_date.to_string(),
            category: transaction.category.to_owned(),
            creditor_name: transaction.creditor_name.to_owned(),
            description: transaction.description.to_owned(),
            amount_cents: Num::from(transaction.amount_cents),
            currency: account.currency.to_owned(),
            amount_cents_in_ref_currency: Num::from(
                valuta_conversion_service.convert(
                    account.currency,
                    "EUR",
                    transaction.amount_cents)),
            ref_currency: "EUR".to_string(),
        })
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}

#[get("/transactions/<id>")]
pub fn get_transaction(_key: ApiKey<'_>, id: i32) -> status::Custom<content::RawJson<String>> {
    let dto = build_transaction_dto(id);

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dto).expect("Serialization Failed")),
    )
}

fn build_transaction_dto(id: i32) -> TransactionDto {
    let mut transactions_service = TransactionsService {};
    let mut valuta_conversion_service = ValutaConversionService::new(&mut establish_db_connection());

    let (transaction, account) = transactions_service.get_transaction(id);

    return TransactionDto {
        id: Num::from(transaction.id),
        type_: transaction.type_.to_owned(),
        account_id: Num::from(transaction.account_id),
        account_name: account.description,
        booking_date: transaction.booking_date.to_string(),
        category: transaction.category.to_owned(),
        creditor_name: transaction.creditor_name.to_owned(),
        description: transaction.description.to_owned(),
        amount_cents: Num::from(transaction.amount_cents),
        currency: account.currency.to_owned(),
        amount_cents_in_ref_currency: Num::from(
            valuta_conversion_service.convert(
                account.currency.to_owned(),
                "EUR",
                transaction.amount_cents)),
        ref_currency: "EUR".to_string(),
    };
}

#[patch("/transactions/<id>", format = "application/json", data = "<patch_transaction_dto>")]
pub fn patch_transaction(_key: ApiKey<'_>, id: i32, patch_transaction_dto: rocket::serde::json::Json<PatchTransactionDto>) -> status::Custom<content::RawJson<String>> {
    let mut transactions_service = TransactionsService {};

    if patch_transaction_dto.category.is_some() {
        transactions_service.update_transaction_category(
            id,
            patch_transaction_dto.category.clone().unwrap(),
        );
    }

    if patch_transaction_dto.description.is_some() {
        transactions_service.update_transaction_description(
            id,
            patch_transaction_dto.description.clone().unwrap(),
        );
    }

    let dto = build_transaction_dto(id);

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dto).expect("Serialization Failed")),
    )
}
