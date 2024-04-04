use std::error::Error;

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
    access_expires: i64,
    refresh: String,
    refresh_expires: i64,
}

#[derive(Deserialize)]
pub struct GetTransactionsResponse {
    transactions: Transactions,
}

#[derive(Deserialize)]
pub struct Transactions {
    booked: Vec<Transaction>,
    pending: Vec<Transaction>,
}

#[derive(Deserialize)]
pub struct Amount {
    amount: String,
    currency: String,
}

#[derive(Deserialize)]
pub struct Account {
    iban: String,
}

#[derive(Deserialize)]
pub struct Balance {
    balanceAmount: Amount,
    balanceType: String,
}

#[derive(Deserialize)]
pub struct Transaction {
    transactionId: String,
    bookingDate: String,
    valueDate: String,
    bookingDateTime: String,
    transactionAmount: Amount,
    #[serde(default)]
    creditorName: String,
    #[serde(default)]
    debtorName: String,
    debtorAccount: Option<Account>,
    remittanceInformationUnstructured: String,
    remittanceInformationUnstructuredArray: Vec<String>,
    balanceAfterTransaction: Balance,
    internalTransactionId: String,
}

impl GoCardlessApi {
    pub fn new() -> GoCardlessApi {
        GoCardlessApi {
            access_token: "".to_string()
        }
    }

    pub fn get_token(&mut self, secret_id: String, secret_key: String) {
        let response: CreateTokenResponse = self.make_post_request(
            "https://bankaccountdata.gocardless.com/api/v2/token/new/",
            CreateTokenRequest {
                secret_id: secret_id.to_string(),
                secret_key: secret_key.to_string(),
            }).unwrap();

        println!("{}", response.access);

        self.access_token = response.access;
    }

    pub fn get_transactions(&mut self,account_id: &str) {
        let response: GetTransactionsResponse = self.make_get_request(
            &format!("https://bankaccountdata.gocardless.com/api/v2/accounts/{account_id}/transactions/"),
        ).unwrap();

        println!("{}", response.transactions.booked[0].remittanceInformationUnstructured);
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

        println!("{}", response_text);

        match serde_json::from_str(&*response_text) {
            Ok(response) => Ok(response),
            Err(e) => panic!("{}", e.to_string()),
        }
    }
}