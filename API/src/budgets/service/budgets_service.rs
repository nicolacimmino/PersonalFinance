use diesel::{RunQueryDsl, sql_query};

use crate::budgets::model::ApplicationBudgets;
use crate::establish_db_connection;

pub struct BudgetsService {}

impl BudgetsService {
    pub fn get_budgets(&mut self) -> Vec<ApplicationBudgets> {
        return sql_query("
            SELECT * FROM application.budgets
                ORDER BY id DESC
          ").load::<ApplicationBudgets>(&mut establish_db_connection())
            .expect("Error loading budgets");
    }
}