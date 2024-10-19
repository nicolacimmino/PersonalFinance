use log::info;
use rocket::{get, patch};
use rocket::figment::value::Num;
use rocket::http::Status;
use rocket::response::{content, status};
use crate::common::ValutaConversionService;
use crate::establish_db_connection;
use crate::transactions::dto::{PatchTransactionDto, TransactionDto};
use crate::guard::ApiKey;
use crate::schema::transactions::dsl::transactions;
use crate::transactions::service::TransactionsService;


#[get("/transactions?<category>")]
pub fn get_transactions(_key: ApiKey<'_>, category: Option<String>) -> status::Custom<content::RawJson<String>> {
    let mut transactions_service = TransactionsService {};
    let mut valuta_conversion_service = ValutaConversionService::new(&mut establish_db_connection());

    let mut dtos: Vec<TransactionDto> = Vec::new();

    for (transaction, account) in transactions_service.get_transactions(category) {
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
            account_to: transaction.account_to,
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
            account_to: transaction.account_to,
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
        account_to: transaction.account_to,
    };
}

#[patch("/transactions/<id>", format = "application/json", data = "<patch_transaction_dto>")]
pub fn patch_transaction(_key: ApiKey<'_>, id: i32, patch_transaction_dto: rocket::serde::json::Json<PatchTransactionDto>) -> status::Custom<content::RawJson<String>> {
    let mut transactions_service = TransactionsService {};

    if patch_transaction_dto.type_.is_some() {
        let new_type = patch_transaction_dto.type_.clone().unwrap_or("".to_string());

        if !vec!["TRANSFER".to_string(), "INCOME".to_string(), "EXPENSE".to_string()].contains(&new_type) {
            panic!("Invalid type.")
        }

        if !new_type.eq(&"TRANSFER".to_string()) {
            let (transaction, _) = transactions_service.get_transaction(id);
            if transaction.amount_cents > 0 && new_type != "INCOME".to_string() {
                panic!("Positive amount is INCOME")
            }
            if transaction.amount_cents <= 0 && new_type != "EXPENSE".to_string() {
                panic!("Negative or zero amount is  EXPENSE")
            }
        }

        transactions_service.update_transaction_type(
            id,
            &new_type,
        );
    }

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

    if patch_transaction_dto.account_to.is_some() {
        transactions_service.update_transaction_account_to(
            id,
            patch_transaction_dto.account_to.clone().unwrap(),
        );
    }

    let dto = build_transaction_dto(id);

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dto).expect("Serialization Failed")),
    )
}
