use rocket::get;
use rocket::figment::value::Num;
use rocket::http::Status;
use rocket::response::{content, status};
use crate::accounts::dto::AccountDto;
use crate::accounts::service::AccountsService;
use crate::common::ValutaConversionService;
use crate::establish_db_connection;
use crate::guard::ApiKey;

#[get("/accounts")]
pub fn get_accounts(_key: ApiKey<'_>) -> status::Custom<content::RawJson<String>> {
    let mut accounts_service = AccountsService {};
    let mut valuta_conversion_service = ValutaConversionService::new(&mut establish_db_connection());

    let mut dtos: Vec<AccountDto> = Vec::new();

    let balances = accounts_service.get_accounts_balances();

    for account in accounts_service.get_accounts() {
        let balance = balances.iter().find(|(key, _val)| *key == account.id)
            .unwrap_or(&(0i32, 0i32)).1;

        dtos.push(AccountDto {
            id: Num::from(account.id),
            code: account.code.to_owned(),
            description: account.description.to_owned(),
            currency: account.currency.to_owned(),
            balance_cents: Num::from(balance),
            balance_cents_in_ref_currency: Num::from(valuta_conversion_service.convert(
                account.currency,
                "EUR",
                balance,
            )),
            ref_currency: "EUR".to_string(),
            iban: account.iban,
            status: account.status,
        })
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}
