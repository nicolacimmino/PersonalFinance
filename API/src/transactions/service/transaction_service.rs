use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, sql_query, TextExpressionMethods};
use diesel::sql_types::{Integer, VarChar};

use crate::{establish_db_connection, schema};
use crate::accounts::model::{Account, ApplicationAccount};
use crate::schema::accounts::dsl::accounts;
use crate::schema::transactions::dsl::transactions;
use crate::transactions::model::{ApplicationTransaction, NewTransaction, Transaction};

pub struct TransactionsService {}

impl TransactionsService {
    pub fn get_transactions(&mut self, category: Option<String>) -> Vec<ApplicationTransaction> {
        return sql_query("
            SELECT * FROM application.transactions
                WHERE category like $1
                ORDER BY booking_date DESC
          ").bind::<VarChar, _>(format!("{}%", category.unwrap_or("".to_string())))
            .load::<ApplicationTransaction>(&mut establish_db_connection())
            .expect("Error loading Transactions");
    }

    pub fn get_transactions_for_account(&mut self, account_id: i32) -> Vec<ApplicationTransaction> {
        return sql_query("
            SELECT * FROM application.transactions
                WHERE account_id = $1
                ORDER BY booking_date DESC
          ").bind::<Integer, _>(account_id)
            .load::<ApplicationTransaction>(&mut establish_db_connection())
            .expect("Error loading Transactions");
    }

    pub fn get_transaction(&mut self, transaction_id: i32) -> ApplicationTransaction {
        return sql_query("
            SELECT * FROM application.transactions
                WHERE id = $1
          ").bind::<Integer, _>(transaction_id)
            .load::<ApplicationTransaction>(&mut establish_db_connection())
            .expect("Error loading Transactions")
            .into_iter().nth(0)
            .expect("No transaction found");
    }

    pub fn update_transaction_category(&mut self, transaction_id: i32, category: String) {
        diesel::update(transactions)
            .filter(schema::transactions::id.eq(transaction_id))
            .set(schema::transactions::category.eq(category))
            .execute(&mut establish_db_connection()).expect("Failed to update transaction category");
    }

    pub fn update_transaction_type(&mut self, transaction_id: i32, type_: &String) {
        diesel::update(transactions)
            .filter(schema::transactions::id.eq(transaction_id))
            .set(schema::transactions::type_.eq(type_))
            .execute(&mut establish_db_connection()).expect("Failed to update transaction type");
    }

    pub fn update_transaction_description(&mut self, transaction_id: i32, description: String) {
        diesel::update(transactions)
            .filter(schema::transactions::id.eq(transaction_id))
            .set(schema::transactions::description.eq(description))
            .execute(&mut establish_db_connection()).expect("Failed to update transaction description");
    }

    pub fn create_transaction(&mut self, transaction: NewTransaction) -> Transaction {
        return diesel::insert_into(transactions)
            .values(transaction)
            .get_result::<Transaction>(&mut establish_db_connection()).expect("Failed to create transaction");
    }

    pub fn update_transaction_account_to(&mut self, transaction_id: i32, account_to: i32) {
        let transaction = self.get_transaction(transaction_id);

        if transaction.movement_type != "TRANSFER" {
            panic!("Cannot set account_to of transaction.")
        }

        diesel::update(transactions)
            .filter(schema::transactions::id.eq(transaction_id))
            .set(schema::transactions::account_to.eq(account_to))
            .execute(&mut establish_db_connection()).expect("Failed to update transaction account to");
    }
}