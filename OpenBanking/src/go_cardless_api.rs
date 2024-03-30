use std::error::Error;
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
        let mut body = json_body.as_bytes();

        let mut dst = Vec::new();
        let mut easy = Easy::new();

        easy.url(url).unwrap();
        easy.post(true).unwrap();
        easy.post_field_size(body.len() as u64).unwrap();

        let mut list = List::new();
        list.append(&"Content-Type: application/json".to_string()).unwrap();
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

        //return serde_json::from_str(&*response_json)
        //.expect("Cannot deserialize");

        match serde_json::from_str(&*response_json) {
            Ok(response) => Ok(response),
            Err(_e) => panic!("conversion error"),
        }
        // println!("{}", response.access);
        //
        // self.access_token = response.access;
    }
}