use diesel::{ExpressionMethods, RunQueryDsl, sql_query};
use diesel::sql_types::{Integer, Nullable, Numeric, Text, VarChar};
use log::debug;

use crate::{establish_db_connection};
use crate::manual_schema::transactions::dsl::transactions;
use crate::transactions::model::{ApplicationTransaction, NewTransaction, Transaction};

pub struct TransactionsService {}

impl TransactionsService {
    pub fn get_transactions(
        &mut self,
        category: Option<String>,
        account_id: Option<i32>,
    ) -> Vec<ApplicationTransaction> {
        debug!("{:?}", account_id);
        return sql_query("
            SELECT * FROM application.transactions
                WHERE category like $1
                AND (account_id = $2 OR $2 IS NULL)
                ORDER BY booking_date DESC
          ").bind::<VarChar, _>(format!("{}%", category.unwrap_or("".to_string())))
            .bind::<Nullable<Integer>, _>(account_id)
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

    pub fn update_transaction(
        &mut self,
        transaction_id: i32,
        category: &Option<String>,
        type_: &Option<String>,
        description: &Option<String>,
        account_to: &Option<i32>,
    ) {
        if "TRANSFER".to_string() != type_.clone().unwrap_or("".to_string()) && account_to.is_some() {
            panic!("Cannot set account_to of transaction.")
        }

        sql_query("
            UPDATE raw.transactions
                SET
                category = COALESCE($1, category),
                type = COALESCE($2, type),
                description = COALESCE($3, description),
                account_to = COALESCE($4, account_to)
                WHERE id = $5
          ")
            .bind::<Nullable<Text>, _>(category)
            .bind::<Nullable<Text>, _>(type_)
            .bind::<Nullable<Text>, _>(description)
            .bind::<Nullable<Integer>, _>(account_to)
            .bind::<Integer, _>(transaction_id)
            .execute(&mut establish_db_connection())
            .expect("Error while updating transaction");
    }

    pub fn create_transaction(&mut self, transaction: NewTransaction) -> i32 {
        let result = diesel::insert_into(transactions)
            .values(transaction)
            .get_result::<Transaction>(&mut establish_db_connection()).expect("Failed to create transaction");

        return result.id;
    }
}