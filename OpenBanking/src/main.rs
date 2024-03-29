use std::env;
use dotenvy::dotenv;
use crate::go_cardless_api::GoCardlessApi;

mod go_cardless_api;

fn main() {
    dotenv().ok();
    let go_cardless_secret_id = env::var("GOCARDLESS_SECRET_ID").expect("GOCARDLESS_SECRET_ID must be set");
    let go_cardless_secret_key = env::var("GOCARDLESS_SECRET_KEY").expect("GOCARDLESS_SECRET_KEY must be set");

    let go_cardless_api = GoCardlessApi {
        secret_id: go_cardless_secret_id,
        secret_key: go_cardless_secret_key,
    };

    let response = go_cardless_api.get_token();

    println!("{}", response.access);

    //println!("{:?}", result);

    // dotenv().ok();
    // let accessToken = env::var("GOCARDLESS_SECRET").expect("GOCARDLESS_SECRET must be set");
    //
    // let mut easy = Easy::new();
    // easy.url("https://bankaccountdata.gocardless.com/api/v2/accounts/bcffd65e-842c-435a-8ef7-d91fbcfccf60/transactions/").unwrap();
    // let mut list = List::new();
    // list.append(&format!("Authorization: Bearer {accessToken}")).unwrap();
    // easy.http_headers(list).unwrap();
    //
    // easy.write_function(|data| {
    //     stdout().write_all(data).unwrap();
    //     Ok(data.len())
    // }).unwrap();
    // easy.perform().unwrap();
}
