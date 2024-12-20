use rocket::get;
use rocket::figment::value::Num;
use rocket::http::Status;
use rocket::response::{content, status};
use crate::budgets::dto::BudgetOverviewDto;
use crate::budgets::service::BudgetsService;
use crate::guard::ApiKey;

#[get("/budgets")]
pub fn get_budgets(_key: ApiKey<'_>) -> status::Custom<content::RawJson<String>> {
    let mut budgets_service = BudgetsService {};

    let mut dtos: Vec<BudgetOverviewDto> = Vec::new();

    for budget_overview in budgets_service.get_budgets() {
        dtos.push(BudgetOverviewDto {
            id: Num::from(budget_overview.id),
            code: budget_overview.code.to_owned(),
            from_date: budget_overview.from_date.to_string(),
            to_date: budget_overview.to_date.to_string(),
            description: budget_overview.description,
            active: budget_overview.active,
            currency: budget_overview.currency,
            amount_cents: budget_overview.amount_cents,
            spent_cents_in_ref_currency: budget_overview.spent_cents_eur,
            spent_cents: budget_overview.spent_cents,
            transactions: budget_overview.transactions,
        })
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}
