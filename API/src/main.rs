mod schema;
mod transactions;
mod guard;
mod accounts;
mod categories;
mod reports;
mod common;

use std::env;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use rocket::{launch, routes};
use rocket::log::private::error;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use crate::accounts::get_accounts;
use crate::categories::get_categories;
use crate::reports::get_report_by_category;
use crate::transactions::{get_transactions, get_transaction, get_transactions_for_account, patch_transaction};

#[launch]
fn launch() -> _ {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    dotenv().ok();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Put, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::build()
        .configure(rocket::Config::figment().merge(("address", "0.0.0.0")))
        .mount("/api", routes![
            get_categories,
            get_accounts,
            get_transactions,
            get_transaction,
            patch_transaction,
            get_transactions_for_account,
            get_report_by_category
        ])
        .attach(cors.to_cors().unwrap())
}

pub fn establish_db_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_e| error!("DATABASE_URL missing"))
        .expect("config");

    PgConnection::establish(&database_url)
        .map_err(|e| error!("Cannot connect to DB: {}", e.to_string()))
        .unwrap()
}
