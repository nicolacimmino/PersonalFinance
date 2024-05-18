use std::env;

use diesel::{Connection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::expression_methods::ExpressionMethods;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use log::{error, info};
use log4rs;
use uuid::Uuid;

use go_cardless::{Account, Amount, Balance};
use go_cardless::GoCardlessApi;
use model::{NewObTransaction, ObAccount};
use schema::ob_accounts::dsl::ob_accounts;
use schema::ob_transactions::dsl::ob_transactions;

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

    go_cardless_api.get_token(go_cardless_secret_id, go_cardless_secret_key);
    let transactions = go_cardless_api.get_transactions(provider_account_id);

    let found_transactions = &transactions.len();

    info!("Got {} transactions", found_transactions);

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

        let balance_after_transaction = transaction.balance_after_transaction.unwrap_or(Balance {
            balance_amount: Amount {
                amount: "0".to_string(),
                currency: "".to_string(),
            },
            balance_type: "".to_string(),
        });

        let test = transaction.remittance_information_unstructured_array.unwrap_or(vec!["".to_string()]).join(" ");

        let remittance_information = transaction.remittance_information_unstructured.unwrap_or(test);

        diesel::insert_into(ob_transactions)
            .values(NewObTransaction {
                ob_account_id: account_id,
                transaction_id: &*transaction.transaction_id,
                booking_date: &*transaction.booking_date,
                value_date: &*transaction.value_date.unwrap_or("".to_string()),
                booking_date_time: &*transaction.booking_date_time.unwrap_or("".to_string()),
                transaction_amount_cents: (transaction.transaction_amount.amount.parse::<f64>().expect("Cannot parse transaction_amount") * 100f64) as i32,
                transaction_amount_currency: &*transaction.transaction_amount.currency,
                creditor_name: &*transaction.creditor_name,
                debtor_name: &*transaction.debtor_name,
                debtor_account_iban: &*transaction.debtor_account.unwrap_or(Account {
                    iban: "".to_string()
                }).iban,
                remittance_information_unstructured: &*remittance_information,
                balance_after_transaction_amount_cents: (balance_after_transaction.balance_amount.amount.parse::<f64>().expect("Cannot parse balance_amount") * 100f64) as i32,
                balance_after_transaction_currency: &*balance_after_transaction.balance_amount.currency,
                balance_after_transaction_type: &*balance_after_transaction.balance_type,
                internal_transaction_id: &*transaction.internal_transaction_id,
            })
            .execute(connection).expect("Cannot insert");

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
