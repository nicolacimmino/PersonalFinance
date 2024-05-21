use diesel::{RunQueryDsl, QueryDsl, SelectableHelper, ExpressionMethods};
use crate::{establish_db_connection, schema};
use crate::schema::accounts::dsl::accounts;
use crate::schema::transactions::dsl::transactions;
use crate::transactions::model::Transaction;
use crate::accounts::model::Account;


pub struct TransactionsService {}

impl TransactionsService {
    pub fn get_transactions(&mut self) -> Vec<(Transaction, Account)> {
        return transactions
            .inner_join(accounts)
            .order(schema::transactions::booking_date.desc())
            .select((Transaction::as_select(), Account::as_select()))
            .load::<(Transaction, Account)>(&mut establish_db_connection())
            .expect("Error loading transactions");
    }

    pub fn get_transactions_for_account(&mut self, account_id: i32) -> Vec<(Transaction, Account)> {
        return transactions
            .inner_join(accounts)
            .filter(schema::transactions::account_id.eq(account_id))
            .order(schema::transactions::booking_date.desc())
            .select((Transaction::as_select(), Account::as_select()))
            .load::<(Transaction, Account)>(&mut establish_db_connection())
            .expect("Error loading transactions");
    }
}