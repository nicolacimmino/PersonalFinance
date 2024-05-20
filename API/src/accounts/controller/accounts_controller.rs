use rocket::get;
use rocket::figment::value::Num;
use rocket::http::Status;
use rocket::response::{content, status};
use crate::accounts::dto::AccountDto;
use crate::accounts::service::AccountsService;
use crate::schema::transactions::dsl::transactions;
use crate::guard::ApiKey;
use crate::schema::accounts::{currency, description};


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
        })
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}
