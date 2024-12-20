use std::env;
use dotenvy::dotenv;
use crate::go_cardless::{ConvertsToGoCardlessTransaction, GoCardlessApi, GoCardlessTransaction};

pub struct TransactionsService {
    go_cardless_secret_id: String,
    go_cardless_secret_key: String,
}

impl TransactionsService {
    pub fn new() -> Self {
        dotenv().ok();

        return TransactionsService {
            go_cardless_secret_id: env::var("GOCARDLESS_SECRET_ID").expect("GOCARDLESS_SECRET_ID must be set"),
            go_cardless_secret_key: env::var("GOCARDLESS_SECRET_KEY").expect("GOCARDLESS_SECRET_KEY must be set"),
        };
    }

    pub fn get_transactions(&self, account_id: &String) -> Vec<GoCardlessTransaction> {
        let mut go_cardless_api = GoCardlessApi::new();

        go_cardless_api.get_token(
            self.go_cardless_secret_id.clone(),
            self.go_cardless_secret_key.clone(),
        );

        return go_cardless_api.get_transactions(&account_id)
            .iter()
            .map(|transaction_dto| transaction_dto.to_gocardless_transaction())
            .collect::<Vec<GoCardlessTransaction>>();
    }
}