use std::env;

use chrono::{NaiveDate, NaiveTime};
use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, SelectableHelper};
use diesel::RunQueryDsl;
use dotenvy::dotenv;
use log::{error, info};

use crate::model::{Account, NewTransaction, ObAccount, ObTransaction, Transaction};
use crate::schema::accounts::dsl::accounts;
use crate::schema::ob_accounts::dsl::ob_accounts;
use crate::schema::ob_transactions::{transformed_transaction_id};
use crate::schema::ob_transactions::dsl::ob_transactions;
use crate::schema::transactions::dsl::transactions;

mod schema;
mod model;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting");

    dotenv().ok();

    let connection = &mut establish_db_connection();

    let to_transform = ob_transactions
        .filter(transformed_transaction_id.is_null())
        .inner_join(ob_accounts.inner_join(accounts))
        .select((ObTransaction::as_select(), ObAccount::as_select(), Account::as_select()))
        .load::<(ObTransaction, ObAccount, Account)>(connection)
        .expect("Failed to get ob_transactions");

    info!("Found {} records", to_transform.len());

    for item_to_transform in to_transform {
        let ob_transaction = item_to_transform.0;
        let account = item_to_transform.2;

        let mut transaction_type = "INCOME";
        if ob_transaction.transaction_amount_cents < 0 {
            transaction_type = "EXPENSE";
        }

        // Most institutions report a booking and a value date. However, should some report only one
        //  we default the other to same.
        let value_date = NaiveDate::parse_from_str(&*ob_transaction.value_date, "%Y-%m-%d")
            .unwrap_or(
                NaiveDate::parse_from_str(&*ob_transaction.booking_date, "%Y-%m-%d")
                    .expect(&format!("Invalid booking_date{}", ob_transaction.booking_date))
            );

        let booking_date = NaiveDate::parse_from_str(&*ob_transaction.booking_date, "%Y-%m-%d")
            .unwrap_or(value_date);

        let new_transaction: Transaction = diesel::insert_into(transactions)
            .values(NewTransaction {
                type_: transaction_type,
                account_id: account.id,
                amount_cents: ob_transaction.transaction_amount_cents,
                category: "",
                creditor_name: &*ob_transaction.creditor_name,
                description: &*ob_transaction.remittance_information_unstructured,
                booking_date: &booking_date.and_time(NaiveTime::default()),
                value_date: &value_date.and_time(NaiveTime::default()),
            })
            .get_result(connection)
            .unwrap();

        diesel::update(ob_transactions)
            .filter(schema::ob_transactions::id.eq(ob_transaction.id))
            .set(transformed_transaction_id.eq(new_transaction.id))
            .execute(connection).expect("Failed to update ob_transaction");

        info!("Converted {}", ob_transaction.id)
    }

    info!("Done");
}

pub fn establish_db_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_e| error!("DATABASE_URL missing"))
        .expect("config");

    PgConnection::establish(&database_url)
        .map_err(|e| error!("Cannot connect to DB: {}", e.to_string()))
        .unwrap()
}
