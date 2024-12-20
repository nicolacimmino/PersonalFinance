use std::env;
use dotenvy::dotenv;
use crate::go_cardless::{ConvertsToGoCardlessAccountInfo, GoCardlessApi, GoCardlessAccountInfo};

pub struct AccountsService {
    go_cardless_secret_id: String,
    go_cardless_secret_key: String,
}

impl AccountsService {
    pub fn new() -> Self {
        dotenv().ok();

        return AccountsService {
            go_cardless_secret_id: env::var("GOCARDLESS_SECRET_ID").expect("GOCARDLESS_SECRET_ID must be set"),
            go_cardless_secret_key: env::var("GOCARDLESS_SECRET_KEY").expect("GOCARDLESS_SECRET_KEY must be set"),
        };
    }

    pub fn get_account(&self, account_id: &String) -> GoCardlessAccountInfo {
        let mut go_cardless_api = GoCardlessApi::new();

        go_cardless_api.get_token(
            self.go_cardless_secret_id.clone(),
            self.go_cardless_secret_key.clone(),
        );

        return (&go_cardless_api.get_account_info(&account_id)).to_gocardless_account_info();
    }
}