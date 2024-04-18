use std::env;

use diesel::{Connection, PgConnection, RunQueryDsl, ExpressionMethods, QueryDsl};
use dotenvy::dotenv;
use log::{error, info};

use crate::model::NewSpTransaction;
use crate::schema::sp_transactions::{amount, date, note, type_, wallet};
use crate::schema::sp_transactions::dsl::sp_transactions;
use crate::spendee::Spendee;

mod spendee;
mod schema;
mod model;

fn main() {
    dotenv().ok();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting");

    let args: Vec<String> = env::args().collect();

    let connection = &mut establish_db_connection();

    let transactions = Spendee::import(&args[1]);

    info!("Found {} transactions to sync", transactions.len());

    let mut added = 0;
    let mut skipped = 0;

    for transaction in transactions {
        let found_transactions: i64 = sp_transactions
            .filter(date.eq(&*transaction.date))
            .filter(amount.eq(&*transaction.amount))
            .filter(note.eq(&*transaction.note))
            .filter(wallet.eq(&*transaction.wallet))
            .filter(type_.eq(&*transaction.type_))
            .count()
            .get_result(connection)
            .expect("Error loading transactions");

        if found_transactions > 0 {
            skipped += 1;
            continue;
        }

        info!("Found new transaction {:?}", transaction);

        diesel::insert_into(sp_transactions)
            .values(NewSpTransaction {
                date: &*transaction.date,
                wallet: &*transaction.wallet,
                type_: &*transaction.type_,
                category: &*transaction.category_name,
                amount: &*transaction.amount,
                currency: &*transaction.currency,
                note: &*transaction.note,
                labels: &*transaction.labels,
                author: &*transaction.author,

            })
            .execute(connection).expect("Cannot insert");

        added += 1;
    }

    info!("Added {} transactions, {} already in DB.", added, skipped);
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
