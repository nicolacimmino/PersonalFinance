use diesel::{RunQueryDsl, sql_query};
use diesel::sql_types::Integer;

use crate::accounts::model::{ApplicationAccount};
use crate::establish_db_connection;

pub struct AccountsService {}

impl AccountsService {
    pub fn get_accounts(&mut self) -> Vec<ApplicationAccount> {
        return sql_query("
            SELECT * FROM application.accounts
          ").load::<ApplicationAccount>(&mut establish_db_connection())
            .expect("Error loading Accounts");
    }

    pub fn get_account(&mut self, account_id: i32) -> ApplicationAccount {
        return sql_query("
            SELECT * FROM application.accounts WHERE id = $1
          ").bind::<Integer, _>(account_id)
            .load::<ApplicationAccount>(&mut establish_db_connection())
            .expect("Error loading Accounts")
            .into_iter().nth(0)
            .unwrap();
    }
}