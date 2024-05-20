mod schema;
mod transactions;
mod guard;
mod accounts;

use std::env;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use rocket::{launch, routes};
use rocket::log::private::error;
use crate::accounts::get_accounts;
use crate::transactions::{get_transactions, get_transactions_for_account};

#[launch]
fn launch() -> _ {
    dotenv().ok();

    rocket::build()
        .configure(rocket::Config::figment().merge(("address", "0.0.0.0")))
        .mount("/api", routes![
            get_accounts,
            get_transactions,
            get_transactions_for_account
        ])
}

pub fn establish_db_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_e| error!("DATABASE_URL missing"))
        .expect("config");

    PgConnection::establish(&database_url)
        .map_err(|e| error!("Cannot connect to DB: {}", e.to_string()))
        .unwrap()
}
