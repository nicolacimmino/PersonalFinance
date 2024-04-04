use std::env;
use dotenvy::dotenv;
use go_cardless_api::GoCardlessApi;

mod go_cardless_api;

fn main() {
    dotenv().ok();
    let go_cardless_secret_id = env::var("GOCARDLESS_SECRET_ID").expect("GOCARDLESS_SECRET_ID must be set");
    let go_cardless_secret_key = env::var("GOCARDLESS_SECRET_KEY").expect("GOCARDLESS_SECRET_KEY must be set");
    let tmp_account_number = env::var("TMP_ACCOUNT_NUMBER").expect("TMP_ACCOUNT_NUMBER must be set");

    let mut go_cardless_api = GoCardlessApi::new();

    go_cardless_api.get_token(go_cardless_secret_id, go_cardless_secret_key);
    go_cardless_api.get_transactions(tmp_account_number);
}
