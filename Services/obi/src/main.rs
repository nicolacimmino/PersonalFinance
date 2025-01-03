mod schema;
mod go_cardless;
mod open_banking;

use std::{env, panic};
use diesel::{Connection, RunQueryDsl};
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use log::{error, info};
use log4rs;
use uuid::Uuid;
use chrono::Utc;
use diesel::ExpressionMethods;
use go_cardless::TransactionsService as GoCardlessTransactionsService;
use go_cardless::AccountsService as GoCardlessAccountsService;
use open_banking::{NewObTransaction, AccountsService as OpenBankingAccountsSerice, TransactionsService as OpenBankingTransactionsService};
use crate::schema::accounts::dsl::accounts;
use crate::schema::ob_accounts::dsl::ob_accounts;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting");

    sync_all_accounts_transactions();

    info!("Done");
}

fn sync_all_accounts_transactions() {
    let connection = &mut establish_db_connection();

    let mut accounts_service = OpenBankingAccountsSerice::new(connection);

    let ob_accounts_to_sync = accounts_service.get_all_accounts();

    info!("Found {} accounts to sync", ob_accounts_to_sync.len());

    for ob_account in ob_accounts_to_sync {
        info!("Syncing account {} ({})", ob_account.name, ob_account.id);
        let result = panic::catch_unwind(|| {
            sync_account_transactions(&ob_account.id, &ob_account.provider_account_id, &ob_account.account_id)
        });

        if result.is_err() {
            error!("Error in account sync.");
        }
    }
}

fn sync_account_transactions(ob_account_id: &Uuid, provider_account_id: &String, account_id: &i32) {
    let connection = &mut establish_db_connection();

    info!("Syncing account status");

    let go_cardless_accounts_service = GoCardlessAccountsService::new();
    let account_info = go_cardless_accounts_service.get_account(provider_account_id);

    diesel::update(ob_accounts)
        .filter(schema::ob_accounts::id.eq(ob_account_id))
        .set(
            schema::ob_accounts::req_status.eq(&account_info.status)
        ).execute(connection).expect("Failed to update ob_account connection status");

    // If the OB account is "READY" it means the requisition is done and still valid.
    //  Anything else means we need to fix something.
    let account_status = if &account_info.status == "READY" { "OK" } else { "ERROR" };

    diesel::update(accounts)
        .filter(schema::accounts::id.eq(account_id))
        .set(
            schema::accounts::status.eq(&account_status)
        )
        .execute(connection).expect("Failed to update account status");

    if account_status != "OK" {
        error!("Account link not ready. Skipping");
        return;
    }

    info!("Getting transactions");

    let mut inserted_transactions = 0;
    let go_cardless_transactions_service = GoCardlessTransactionsService::new();

    let transactions = go_cardless_transactions_service
        .get_transactions(provider_account_id);

    let found_transactions = &transactions.len();

    info!("Got {} transactions from GoCardless", found_transactions);

    let mut transactions_service = OpenBankingTransactionsService::new(connection);

    for transaction in transactions {
        if transactions_service.matching_transaction_exists(
            transaction.internal_transaction_id.clone(),
            ob_account_id,
        ) {
            continue;
        }

        info!("Found new transaction {}", &*transaction.transaction_id);

        transactions_service.add_transaction(
            NewObTransaction {
                ob_account_id,
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
            });

        inserted_transactions += 1;
    }

    info!("Added {} transactions, {} already in DB.", inserted_transactions, found_transactions);

    info!("Updating last sync.");

    diesel::update(ob_accounts)
        .filter(schema::ob_accounts::id.eq(ob_account_id))
        .set(
            schema::ob_accounts::last_sync.eq(Utc::now().naive_utc())
        ).execute(connection).expect("Failed to update ob_account last sync");
}

pub fn establish_db_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|_e| error!("DATABASE_URL missing"))
        .expect("config");

    PgConnection::establish(&database_url)
        .map_err(|e| error!("Cannot connect to DB: {}", e.to_string()))
        .unwrap()
}
