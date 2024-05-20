use diesel::{ RunQueryDsl, QueryDsl, SelectableHelper};
use crate::establish_db_connection;
use crate::schema::transactions::dsl::transactions;
use crate::transactions::model::Transaction;


pub struct TransactionsService {}

impl TransactionsService {
    pub fn get_transactions(&mut self) -> Vec<Transaction> {
        return transactions
            .select(Transaction::as_select())
            .load::<Transaction>(&mut establish_db_connection())
            .expect("Error loading transactions");
    }
}