use std::env;

use diesel::{Connection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::expression_methods::ExpressionMethods;
use diesel::internal::derives::multiconnection::SelectStatementAccessor;
use diesel::pg::PgConnection;
use dotenvy::dotenv;

use go_cardless_api::GoCardlessApi;
pub use schema::ob_transactions::dsl::*;

use crate::go_cardless_api::Account;
use crate::model::NewObTransaction;

mod schema;

mod go_cardless_api;
mod model;

fn main() {
    dotenv().ok();
    let go_cardless_secret_id = env::var("GOCARDLESS_SECRET_ID").expect("GOCARDLESS_SECRET_ID must be set");
    let go_cardless_secret_key = env::var("GOCARDLESS_SECRET_KEY").expect("GOCARDLESS_SECRET_KEY must be set");
    let tmp_account_number = env::var("TMP_ACCOUNT_NUMBER").expect("TMP_ACCOUNT_NUMBER must be set");

    let mut go_cardless_api = GoCardlessApi::new();

    go_cardless_api.get_token(go_cardless_secret_id, go_cardless_secret_key);
    let transactions = go_cardless_api.get_transactions(&tmp_account_number);

    let connection = &mut establish_db_connection();

    for transaction in transactions {
        let found_transactions: i64 = ob_transactions
            .filter(internal_transaction_id.eq(&*transaction.internal_transaction_id))
            .count()
            .get_result(connection)
            .expect("Error loading transactions");

        if (found_transactions > 0) {
            continue;
        }

        diesel::insert_into(ob_transactions)
            .values(NewObTransaction {
                transaction_id: &*transaction.transaction_id,
                booking_date: &*transaction.booking_date,
                value_date: &*transaction.value_date,
                booking_date_time: &*transaction.booking_date_time,
                transaction_amount_cents: (transaction.transaction_amount.amount.parse::<f64>().expect("Cannot parse transaction_amount") * 100f64) as i32,
                transaction_amount_currency: &*transaction.transaction_amount.currency,
                creditor_name: &*transaction.creditor_name,
                debtor_name: &*transaction.debtor_name,
                debtor_account_iban: &*transaction.debtor_account.unwrap_or(Account {
                    iban: "".to_string()
                }).iban,
                remittance_information_unstructured: &*transaction.remittance_information_unstructured,
                balance_after_transaction_amount_cents: (transaction.balance_after_transaction.balance_amount.amount.parse::<f64>().expect("Cannot parse balance_amount") * 100f64) as i32,
                balance_after_transaction_currency: &*transaction.balance_after_transaction.balance_amount.currency,
                balance_after_transaction_type: &*transaction.balance_after_transaction.balance_type,
                internal_transaction_id: &*transaction.internal_transaction_id,
            })
            .execute(connection).expect("Cannot insert");
    }
}

pub fn establish_db_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}