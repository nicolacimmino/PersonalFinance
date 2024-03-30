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

    fn make_post_request<RequestT, ResponseT>(&mut self, url: &str, request: RequestT) -> Result<ResponseT, &'static dyn Error>
        where
            RequestT: Serialize,
            ResponseT: for<'a> Deserialize<'a> + Sized
    {
        let json_body = match serde_json::to_string(&request) {
            Ok(json) => json,
            Err(_e) => panic!("Error!"),
        };
        //let body = json_body.as_bytes();

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
}