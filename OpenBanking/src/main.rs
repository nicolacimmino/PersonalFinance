use std::env;

use diesel::{Connection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::expression_methods::ExpressionMethods;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use log::{error, info};
use log4rs;
use uuid::Uuid;

use go_cardless::GoCardlessApi;
use model::{NewObTransaction, ObAccount};
use schema::ob_accounts::dsl::ob_accounts;
use schema::ob_transactions::dsl::ob_transactions;
use crate::go_cardless::GoCardlessTransaction;
use crate::go_cardless::ConvertsToGoCardlessTransaction;

mod schema;
mod go_cardless;
mod model;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting");

    dotenv().ok();

    let connection = &mut establish_db_connection();

    let ob_accounts_to_sync: Vec<ObAccount> = ob_accounts
        .filter(schema::ob_accounts::provider.eq("GOCARDLESS"))
        .select(ObAccount::as_select())
        .load(connection)
        .expect("Error loading ob_accounts");

    info!("Found {} accounts to sync", ob_accounts_to_sync.len());

    for ob_account in ob_accounts_to_sync {
        info!("Syncing account {} ({})", ob_account.name, ob_account.id);
        sync_account_transactions(&ob_account.id, &ob_account.provider_account_id)
    }

    info!("Done");
}

fn sync_account_transactions(account_id: &Uuid, provider_account_id: &String) {
    let go_cardless_secret_id = env::var("GOCARDLESS_SECRET_ID").expect("GOCARDLESS_SECRET_ID must be set");
    let go_cardless_secret_key = env::var("GOCARDLESS_SECRET_KEY").expect("GOCARDLESS_SECRET_KEY must be set");
    let mut inserted_transactions = 0;

    let mut go_cardless_api = GoCardlessApi::new();

    info!("Getting transactions");

    // TODO: this is a concern of the API class
    go_cardless_api.get_token(go_cardless_secret_id, go_cardless_secret_key);

    // TODO: this is a concern of the API class
    let transactions = go_cardless_api.get_transactions(provider_account_id)
        .iter()
        .map(|transaction_dto| transaction_dto.to_gocardless_transaction())
        .collect::<Vec<GoCardlessTransaction>>();

    let found_transactions = &transactions.len();

    info!("Got {} transactions from GoCardless", found_transactions);

    let connection = &mut establish_db_connection();

    for transaction in transactions {
        let found_transactions: i64 = ob_transactions
            .filter(schema::ob_transactions::internal_transaction_id.eq(&*transaction.internal_transaction_id))
            .filter(schema::ob_transactions::ob_account_id.eq(account_id))
            .count()
            .get_result(connection)
            .expect("Error loading transactions");

        if found_transactions > 0 {
            continue;
        }

        info!("Found new transaction {}", transaction.transaction_id);

        diesel::insert_into(ob_transactions)
            .values(NewObTransaction {
                ob_account_id: account_id,
                transaction_id: &*transaction.transaction_id,
                booking_date: &*transaction.booking_date,
                value_date: &*transaction.value_date,
                // TODO: drop from model, it's seldom and inconsistently populated, date is enough
                booking_date_time: "",
                transaction_amount_cents: transaction.transaction_amount_cents,
                transaction_amount_currency: &*transaction.transaction_amount_currency,
                creditor_name: &*transaction.creditor_name,
                debtor_name: &*transaction.debtor_name,
                debtor_account_iban: &*transaction.debtor_account_iban,
                remittance_information_unstructured: &*transaction.remittance_information_unstructured,
                balance_after_transaction_amount_cents: transaction.balance_after_transaction_cents,
                balance_after_transaction_currency: &*transaction.transaction_amount_currency,
                balance_after_transaction_type: &*transaction.balance_after_transaction_type,
                internal_transaction_id: &*transaction.internal_transaction_id,
            })
            .execute(connection)
            .expect("Cannot insert ob_transactions");

        inserted_transactions += 1;
    }

    info!("Added {} transactions, {} already in DB.", inserted_transactions, found_transactions);
}

pub fn establish_db_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_e| error!("DATABASE_URL missing"))
        .expect("config");

    PgConnection::establish(&database_url)
        .map_err(|e| error!("Cannot connect to DB: {}", e.to_string()))
        .unwrap()
}
