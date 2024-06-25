use std::env;
use std::error::Error;

use dotenvy::dotenv;
use log::info;
use serde::{Deserialize, Serialize};
use crate::go_cardless::dto::prelude::*;

#[derive(Default)]
pub struct GoCardlessApi {
    access_token: String,
}

impl GoCardlessApi {
    pub fn new() -> GoCardlessApi {
        GoCardlessApi {
            access_token: "".to_string()
        }
    }

    pub fn get_token(&mut self, secret_id: String, secret_key: String) {
        dotenv().ok();

        let gocardless_host = env::var("GOCARDLESS_HOST").expect("GOCARDLESS_HOST");

        let response: CreateTokenResponseDto = self.make_post_request(
            &format!("{gocardless_host}/api/v2/token/new/"),
            CreateTokenRequestDto {
                secret_id: secret_id.to_string(),
                secret_key: secret_key.to_string(),
            }).unwrap();

        self.access_token = response.access;
    }

    pub fn get_transactions(&mut self, account_id: &String) -> Vec<TransactionDto> {
        dotenv().ok();

        let gocardless_host = env::var("GOCARDLESS_HOST").expect("GOCARDLESS_HOST");
        let response: GetTransactionsResponseDto = self.make_get_request(
            &format!("{gocardless_host}/api/v2/accounts/{account_id}/transactions/"),
        ).unwrap();

        return response.transactions.booked;
    }

    fn make_post_request<RequestT, ResponseT>(&mut self, url: &str, request: RequestT) -> Result<ResponseT, &'static dyn Error>
        where
            RequestT: Serialize,
            ResponseT: for<'a> Deserialize<'a> + Sized
    {
        let json_body = match serde_json::to_string(&request) {
            Ok(json) => json,
            Err(_e) => panic!("Error!"),
        };

        let client = reqwest::blocking::Client::new();
        let response_text = client
            .post(url)
            .header("Content-Type", "application/json")
            .body(json_body)
            .send().unwrap().text().unwrap();

        match serde_json::from_str(&*response_text) {
            Ok(response) => Ok(response),
            Err(_e) => panic!("conversion error"),
        }
    }

    fn make_get_request<ResponseT>(&mut self, url: &str) -> Result<ResponseT, &'static dyn Error>
        where
            ResponseT: for<'a> Deserialize<'a> + Sized
    {
        let client = reqwest::blocking::Client::new();
        let access_token = self.access_token.to_string();
        let response_text = client
            .get(url)
            .header("Authorization", &format!("Bearer {access_token}"))
            .send().unwrap().text().unwrap();

        match serde_json::from_str(&*response_text) {
            Ok(response) => Ok(response),
            Err(e) => panic!("{}", e.to_string()),
        }
    }
}