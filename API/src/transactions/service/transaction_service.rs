use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{establish_db_connection, schema};
use crate::accounts::model::Account;
use crate::schema::accounts::dsl::accounts;
use crate::schema::transactions::dsl::transactions;
use crate::transactions::model::Transaction;

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

    pub fn get_transaction(&mut self, transaction_id: i32) -> (Transaction, Account) {
        return transactions
            .inner_join(accounts)
            .filter(schema::transactions::id.eq(transaction_id))
            .select((Transaction::as_select(), Account::as_select()))
            .load::<(Transaction, Account)>(&mut establish_db_connection())
            .expect("Error loading transactions")
            .into_iter().nth(0)
            .expect("No transaction found");
    }

    pub fn update_transaction_category(&mut self, transaction_id: i32, category: String) {
        diesel::update(transactions)
            .filter(schema::transactions::id.eq(transaction_id))
            .set(schema::transactions::category.eq(category))
            .execute(&mut establish_db_connection()).expect("Failed to update transaction category");
    }

    pub fn update_transaction_description(&mut self, transaction_id: i32, description: String) {
        diesel::update(transactions)
            .filter(schema::transactions::id.eq(transaction_id))
            .set(schema::transactions::description.eq(description))
            .execute(&mut establish_db_connection()).expect("Failed to update transaction description");
    }
}