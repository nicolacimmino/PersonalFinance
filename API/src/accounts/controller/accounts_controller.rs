use rocket::get;
use rocket::figment::value::Num;
use rocket::http::Status;
use rocket::response::{content, status};
use crate::accounts::dto::AccountDto;
use crate::accounts::service::AccountsService;
use crate::establish_db_connection;
use crate::guard::ApiKey;

#[get("/accounts")]
pub fn get_accounts(_key: ApiKey<'_>) -> status::Custom<content::RawJson<String>> {
    let mut accounts_service = AccountsService {};

    let mut dtos: Vec<AccountDto> = Vec::new();

    for account in accounts_service.get_accounts() {
        dtos.push(AccountDto {
            id: Num::from(account.id),
            code: account.code.to_owned(),
            description: account.description.to_owned(),
            currency: account.currency.to_owned(),
            balance_cents: Num::from(account.balance_cents),
            balance_cents_in_ref_currency: Num::from(account.balance_eur_cents),
            ref_currency: "EUR".to_string(),
            iban: account.iban,
            status: account.status,
            type_: account.asset_type,
            can_create_transactions: if account.pri_transactions_src != "OBI" { "1".to_string() } else { "0".to_string() },
        })
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}

#[get("/accounts/<id>")]
pub fn get_account(_key: ApiKey<'_>, id: i32) -> status::Custom<content::RawJson<String>> {
    let mut accounts_service = AccountsService {};

    let account = accounts_service.get_account(id);

    let account_dto = AccountDto {
        id: Num::from(account.id),
        code: account.code.to_owned(),
        description: account.description.to_owned(),
        currency: account.currency.to_owned(),
        balance_cents: Num::from(account.balance_cents),
        balance_cents_in_ref_currency: Num::from(account.balance_eur_cents),
        ref_currency: "EUR".to_string(),
        iban: account.iban,
        status: account.status,
        type_: account.asset_type,
        can_create_transactions: if account.pri_transactions_src != "OBI" { "1".to_string() } else { "0".to_string() },
    };

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&account_dto).expect("Serialization Failed")),
    )
}
