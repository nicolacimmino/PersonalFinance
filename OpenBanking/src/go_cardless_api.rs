use std::io::Read;

use curl::easy::{Easy, List};
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
        let binding = serde_json::to_string(&CreateTokenRequest {
            secret_id: secret_id.to_string(),
            secret_key: secret_key.to_string(),
        }).unwrap();
        let mut body = binding.as_bytes();
        let mut dst = Vec::new();
        let mut easy = Easy::new();

        easy.url("https://bankaccountdata.gocardless.com/api/v2/token/new/").unwrap();
        easy.post(true).unwrap();
        easy.post_field_size(body.len() as u64).unwrap();

        let mut list = List::new();
        list.append(&format!("Content-Type: application/json")).unwrap();
        easy.http_headers(list).unwrap();

        {
            let mut transfer = easy.transfer();
            transfer.read_function(|buf| {
                Ok(body.read(buf).unwrap_or(0))
            }).unwrap();

            transfer
                .write_function(|buf| {
                    dst.extend_from_slice(buf);
                    Ok(buf.len())
                })
                .unwrap();

            transfer.perform().unwrap();
        }
        let response_json = String::from_utf8(dst).expect("Cannot parse string");

        let response: CreateTokenResponse = serde_json::from_str(&*response_json)
            .expect("Cannot deserialize");

        println!("{}", response.access);

        self.access_token = response.access;
    }
}