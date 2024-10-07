use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use crate::budgets::model::BudgetOverview;

use crate::{establish_db_connection, manual_schema};
use crate::manual_schema::budgets_overview::dsl::budgets_overview;

pub struct BudgetsService {}

impl BudgetsService {
    pub fn get_budgets(&mut self) -> Vec<BudgetOverview> {
        return budgets_overview
            .order(manual_schema::budgets_overview::id.desc())
            .select(BudgetOverview::as_select())
            .load::<BudgetOverview>(&mut establish_db_connection())
            .expect("Error loading budgets");
    }
}