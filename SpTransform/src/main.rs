use std::env;
use std::ops::{Add, Sub};
use chrono::{NaiveDate, NaiveTime, TimeDelta};

use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, SelectableHelper};
use diesel::RunQueryDsl;
use dotenvy::dotenv;
use log::{error, info, warn};
use strsim::{jaro};

use crate::model::{Account, NewTransaction, SpTransaction, Transaction};
use crate::schema::accounts::dsl::accounts;
use crate::schema::sp_accounts::dsl::sp_accounts;
use crate::schema::sp_transactions::dsl::sp_transactions;
use crate::schema::transactions::dsl::transactions;

mod schema;
mod model;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting");

    dotenv().ok();

    let connection = &mut establish_db_connection();

    let to_transform = sp_transactions
        .filter(schema::sp_transactions::transformed_transaction_id.is_null())
        .inner_join(sp_accounts.inner_join(accounts))
        .select((SpTransaction::as_select(), Account::as_select()))
        .load::<(SpTransaction, Account)>(connection)
        .expect("Failed to get sp_transactions");

    info!("Found {} records", to_transform.len());

    for item_to_transform in to_transform {
        let sp_transaction = item_to_transform.0;
        let item_to_transform_account = item_to_transform.1;

        let sp_transaction_date = NaiveDate::parse_and_remainder(&*sp_transaction.date, "%Y-%m-%d")
            .expect("Invalid date").0
            .and_time(NaiveTime::default());

        let sp_transaction_amount_cents = (
            sp_transaction.amount.parse::<f64>()
                .expect("Cannot parse transaction_amount") * 100f64) as i32;

        let sp_category = match sp_transaction.category.as_str() {
            "Other" => "OTH",
            "Other Income" => "REV.OTH",
            "Company Net Revenue" => "REV.COM",
            "Interest" => "REV.INT",
            "Paper Capital Gain" => "REV.PCG",
            "Paper Capital Loss" => "PCL",
            "DST.Refunds" => "DST.REF",
            "ESS.Refunds" => "ESS.REF",
            _ => &*sp_transaction.category
        };

        if sp_category.len() > 3 && !sp_category.contains(".") {
            error!("Missing mapping for {}", sp_category);
            continue;
        }

        let sp_label = match sp_transaction.labels.as_str() {
            "Repeating" => "",
            _ => &*sp_transaction.labels
        };

        let categories: Vec<&str> = vec![&*sp_category, &*sp_label]
            .iter().filter(|&val| !val.is_empty()).cloned().collect();

        let combined_category = categories.join(".");

        // If Spendee Transform is the primary source just create a new transaction,
        //  don't attempt to match.
        if item_to_transform_account.pri_transactions_src == "SPT" {
            let sp_transaction_type = match sp_transaction.type_.as_str() {
                "Outgoing Transfer" => "TRANSFER",
                "Incoming Transfer" => "TRANSFER",
                "Expense" => "EXPENSE",
                "Income" => "INCOME",
                _ => panic!("Cannot determine type for: {}", sp_transaction.type_)
            };

            let new_transaction: Transaction = diesel::insert_into(transactions)
                .values(NewTransaction {
                    type_: sp_transaction_type,
                    account_id: item_to_transform_account.id,
                    amount_cents: sp_transaction_amount_cents,
                    category: &*combined_category,
                    creditor_name: "",
                    description: &*sp_transaction.note,
                    booking_date: &sp_transaction_date,
                    value_date: &sp_transaction_date,
                })
                .get_result(connection)
                .unwrap();

            diesel::update(sp_transactions)
                .filter(schema::sp_transactions::id.eq(sp_transaction.id))
                .set(schema::sp_transactions::transformed_transaction_id.eq(new_transaction.id))
                .execute(connection).expect("Failed to update sp_transaction");

            continue;
        }

        // Attempt to find by booking_date
        let mut existing_transactions = transactions
            .filter(schema::transactions::booking_date.eq(sp_transaction_date))
            .filter(schema::transactions::amount_cents.eq(sp_transaction_amount_cents))
            .filter(schema::transactions::account_id.eq(item_to_transform_account.id))
            .filter(schema::transactions::category.eq(""))
            .select(Transaction::as_select())
            .load(connection)
            .expect("Failed to search for matching transactions");

        // If no exact match attempt by value date.
        if existing_transactions.len() == 0 {
            existing_transactions = transactions
                .filter(schema::transactions::value_date.eq(sp_transaction_date))
                .filter(schema::transactions::amount_cents.eq(sp_transaction_amount_cents))
                .filter(schema::transactions::account_id.eq(item_to_transform_account.id))
                .filter(schema::transactions::category.eq(""))
                .select(Transaction::as_select())
                .load(connection)
                .expect("Failed to search for matching transactions");
        }

        // If still no match attempt a +/-4 days.
        if existing_transactions.len() == 0 {
            existing_transactions = transactions
                .filter(schema::transactions::value_date.between(
                    sp_transaction_date.sub(TimeDelta::days(7)),
                    sp_transaction_date.add(TimeDelta::days(7)),
                ))
                .filter(schema::transactions::amount_cents.eq(sp_transaction_amount_cents))
                .filter(schema::transactions::account_id.eq(item_to_transform_account.id))
                .filter(schema::transactions::category.eq(""))
                .select(Transaction::as_select())
                .load(connection)
                .expect("Failed to search for matching transactions");
        }

        if existing_transactions.len() == 0 {
            continue;
        }

        if existing_transactions.len() > 1 {
            warn!("Multiple matches found. Proceeding with first.\r\n{:?}\r\n{:?}\r\n-------------------------------", sp_transaction, existing_transactions);
        }

        let matching_transaction = existing_transactions.first()
            .expect("Something off, we have one transaction but can't get first!");

        let transaction_type = match sp_transaction.type_.as_str() {
            "Outgoing Transfer" => "TRANSFER",
            "Incoming Transfer" => "TRANSFER",
            _ => &*matching_transaction.type_
        };

        let mut updated_description = matching_transaction.description.clone();

        if jaro(&*matching_transaction.description, &*sp_transaction.note) < 0.8 {
            // Spendee for some banks uses other parts of the original OB transaction. Ignore different
            //  if the spendee description starts withe same, otherwise append.
            if !sp_transaction.note.starts_with(&*matching_transaction.description) {
                updated_description += &*format!("//{}", &*sp_transaction.note);
            }
        }

        diesel::update(transactions)
            .filter(schema::transactions::id.eq(matching_transaction.id))
            .set((
                schema::transactions::category.eq(combined_category),
                schema::transactions::description.eq(updated_description),
                schema::transactions::type_.eq(transaction_type)
            ))
            .execute(connection).expect("Failed to update transaction");

        diesel::update(&sp_transaction)
            .filter(schema::sp_transactions::id.eq(sp_transaction.id))
            .set(schema::sp_transactions::transformed_transaction_id.eq(matching_transaction.id))
            .execute(connection).expect("Failed to update sp_transaction");
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
