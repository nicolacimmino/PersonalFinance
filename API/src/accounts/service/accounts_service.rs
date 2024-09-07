use diesel::{RunQueryDsl, QueryDsl, SelectableHelper, ExpressionMethods};
use crate::{establish_db_connection, schema};
use crate::accounts::model::Account;
use crate::schema::accounts::dsl::accounts;
use crate::schema::transactions::dsl::transactions;

pub struct AccountsService {}

impl AccountsService {
    pub fn get_accounts(&mut self) -> Vec<Account> {
        return accounts
            .order(schema::accounts::id.desc())
            .select(Account::as_select())
            .load::<Account>(&mut establish_db_connection())
            .expect("Error loading accounts");
    }

    pub fn get_account(&mut self, account_id: i32) -> Account {
        return accounts
            .filter(schema::accounts::id.eq(account_id))
            .select(Account::as_select())
            .load::<Account>(&mut establish_db_connection())
            .expect("Cannot load accounts")
            .into_iter().nth(0)
            .expect("Account not found");
    }

    pub fn get_accounts_balances(&mut self) -> Vec<(i32, i32)> {
        return accounts
            .inner_join(transactions)
            .group_by(schema::accounts::id)
            .select((schema::accounts::id, diesel::dsl::sql::<diesel::sql_types::Integer>("CAST(sum(amount_cents) as int4)")))
            .load::<(i32, i32)>(&mut establish_db_connection())
            .expect("Error loading balances");
    }
}