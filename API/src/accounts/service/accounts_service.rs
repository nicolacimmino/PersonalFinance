use diesel::{RunQueryDsl, QueryDsl, SelectableHelper, ExpressionMethods};
use crate::{establish_db_connection, schema};
use crate::accounts::model::Account;
use crate::schema::accounts::dsl::accounts;

pub struct AccountsService {}

impl AccountsService {
    pub fn get_accounts(&mut self) -> Vec<Account> {
        return accounts
            .order(schema::accounts::id.desc())
            .select(Account::as_select())
            .load::<Account>(&mut establish_db_connection())
            .expect("Error loading accounts");
    }
}