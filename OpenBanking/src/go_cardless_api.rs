use std::env;
use std::error::Error;
use dotenvy::dotenv;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct GoCardlessApi {
    access_token: String,
}

#[derive(Serialize)]
struct CreateTokenRequest {
    secret_id: String,
    secret_key: String,
}

#[derive(Deserialize)]
pub struct CreateTokenResponse {
    pub(crate) access: String,
}

#[derive(Deserialize)]
pub struct GetTransactionsResponse {
    transactions: Transactions,
}

#[derive(Deserialize)]
pub struct Transactions {
    booked: Vec<Transaction>,
}

#[derive(Deserialize)]
pub struct Amount {
    pub amount: String,
    pub currency: String,
}

#[derive(Deserialize)]
pub struct Account {
    pub iban: String,
}

#[derive(Deserialize)]
pub struct Balance {
    #[serde(rename(deserialize = "balanceAmount"))]
    pub balance_amount: Amount,
    #[serde(rename(deserialize = "balanceType"))]
    pub balance_type: String,
}

#[derive(Deserialize)]
pub struct Transaction {
    #[serde(rename(deserialize = "transactionId"))]
    pub transaction_id: String,
    #[serde(rename(deserialize = "bookingDate"))]
    pub booking_date: String,
    #[serde(rename(deserialize = "valueDate"))]
    pub value_date: String,
    #[serde(rename(deserialize = "bookingDateTime"))]
    pub booking_date_time: String,
    #[serde(rename(deserialize = "transactionAmount"))]
    pub transaction_amount: Amount,
    #[serde(default)]
    #[serde(rename(deserialize = "creditorName"))]
    pub creditor_name: String,
    #[serde(default)]
    #[serde(rename(deserialize = "debtorName"))]
    pub debtor_name: String,
    #[serde(rename(deserialize = "debtorAccount"))]
    pub debtor_account: Option<Account>,
    #[serde(rename(deserialize = "remittanceInformationUnstructured"))]
    pub remittance_information_unstructured: String,
    #[serde(rename(deserialize = "remittanceInformationUnstructuredArray"))]
    pub remittance_information_unstructured_array: Vec<String>,
    #[serde(rename(deserialize = "balanceAfterTransaction"))]
    pub balance_after_transaction: Balance,
    #[serde(rename(deserialize = "internalTransactionId"))]
    pub internal_transaction_id: String,
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

        let response: CreateTokenResponse = self.make_post_request(
            &format!("{gocardless_host}/api/v2/token/new/"),
            CreateTokenRequest {
                secret_id: secret_id.to_string(),
                secret_key: secret_key.to_string(),
            }).unwrap();

        self.access_token = response.access;
    }

    pub fn get_transactions(&mut self, account_id: &String) -> Vec<Transaction> {
        dotenv().ok();

        let gocardless_host = env::var("GOCARDLESS_HOST").expect("GOCARDLESS_HOST");
        let response: GetTransactionsResponse = self.make_get_request(
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