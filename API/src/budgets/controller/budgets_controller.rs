use chrono::NaiveDate;
use rocket::get;
use rocket::figment::value::Num;
use rocket::http::Status;
use rocket::response::{content, status};
use crate::accounts::dto::AccountDto;
use crate::accounts::service::AccountsService;
use crate::budgets::dto::BudgetOverviewDto;
use crate::budgets::service::BudgetsService;
use crate::common::ValutaConversionService;
use crate::establish_db_connection;
use crate::guard::ApiKey;
use crate::schema::sp_accounts::account_id;

#[get("/budgets")]
pub fn get_budgets(_key: ApiKey<'_>) -> status::Custom<content::RawJson<String>> {
    let mut budgets_service = BudgetsService {};

    let mut dtos: Vec<BudgetOverviewDto> = Vec::new();

    for budgetOverview in budgets_service.get_budgets() {
        dtos.push(BudgetOverviewDto {
            id: Num::from(budgetOverview.id),
            code: budgetOverview.code.to_owned(),
            from_date: budgetOverview.from_date.to_string(),
            to_date: budgetOverview.to_date.to_string(),
            description: budgetOverview.description,
            active: budgetOverview.active,
            currency: budgetOverview.currency,
            amount_cents: budgetOverview.amount_cents,
            spent_cents_in_ref_currency: budgetOverview.spent_cents_eur,
            spent_cents: budgetOverview.spent_cents,
            transactions: budgetOverview.transactions,
        })
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}
