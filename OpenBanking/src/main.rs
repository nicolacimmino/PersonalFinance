use std::env;
use std::io::{stdout, Write};

use curl::easy::{Easy, List};

fn main() {
    let accessToken = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut easy = Easy::new();
    easy.url("https://bankaccountdata.gocardless.com/api/v2/accounts/bcffd65e-842c-435a-8ef7-d91fbcfccf60/transactions/").unwrap();
    let mut list = List::new();
    list.append("Authorization: Bearer xxxx").unwrap();
    easy.http_headers(list).unwrap();

    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}
