mod schema;
mod transactions;

use std::env;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use rocket::{launch, routes};
use rocket::log::private::error;
use crate::transactions::get_transactions;

#[launch]
fn launch() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/", routes![get_transactions])
}

pub fn establish_db_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_e| error!("DATABASE_URL missing"))
        .expect("config");

    PgConnection::establish(&database_url)
        .map_err(|e| error!("Cannot connect to DB: {}", e.to_string()))
        .unwrap()
}