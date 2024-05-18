use diesel::{PgConnection, RunQueryDsl, QueryDsl, ExpressionMethods};
use uuid::Uuid;
use crate::schema;
use schema::ob_transactions::dsl::ob_transactions;
use crate::open_banking::NewObTransaction;

pub struct TransactionsService<'a> {
    connection: &'a mut PgConnection,
}

impl TransactionsService<'_> {
    pub fn new(connection: &mut PgConnection) -> TransactionsService {
        return TransactionsService {
            connection
        };
    }
    pub fn matching_transaction_exists(
        &mut self,
        internal_transaction_id: String,
        account_id: &Uuid,
    ) -> bool {
        let found_transactions: i64 = ob_transactions
            .filter(schema::ob_transactions::internal_transaction_id.eq(internal_transaction_id))
            .filter(schema::ob_transactions::ob_account_id.eq(account_id))
            .count()
            .get_result(self.connection)
            .expect("Error loading transactions");

        return found_transactions > 0;
    }

    pub fn add_transaction(
        &mut self,
        transaction: NewObTransaction) {
        diesel::insert_into(ob_transactions)
            .values(transaction)
            .execute(self.connection)
            .expect("Cannot insert ob_transactions");
    }
}