extern crate alloc;
extern crate core;

mod model;
mod schema;

use std::env;
use std::path::Path;
use std::process::ExitCode;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use chrono::{NaiveDateTime};

use diesel::{Connection, PgConnection, RunQueryDsl, ExpressionMethods, QueryDsl};
use dotenvy::dotenv;
use log::{error, info};
use serde::Deserialize;
use crate::model::{NewReceipt, NewReceiptLineItem, Receipt};


fn main() -> ExitCode {
    dotenv().ok();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting");

    let args: Vec<String> = env::args().collect();

    let connection = &mut establish_db_connection();

    let taggun_host = env::var("TAGGUN_HOST").expect("TAGGUN_HOST");
    let taggun_api_key = env::var("TAGGUN_APIKEY").expect("TAGGUN_APIKEY");

    let form = reqwest::blocking::multipart::Form::new()
        .text("extractLineItems", "true")
        .text("extractTime", "true")
        .file("file", &args[1])
        .expect("Cannot load file");

    // Keep the file name without path as reference so we can link the receipt
    //  to the file in S3 processed folder for instance.
    let scan_file_name = Path::new(&args[1]).file_name().unwrap().to_str().unwrap();

    let found_scans: i64 = schema::receipts::dsl::receipts
        .filter(schema::receipts::scan_file_name.eq(&scan_file_name))
        .count()
        .get_result(connection)
        .expect("Error looking for existing scans");

    if found_scans > 0 {
        error!(
            "Receipt from same scan file {} already imported.",
            &scan_file_name
        );
        return ExitCode::FAILURE;
    }

    let client = reqwest::blocking::Client::new();
    let response_text = client.post(format!("{}/api/receipt/v1/verbose/file", taggun_host))
        .multipart(form)
        .header("apikey", taggun_api_key)
        .header("Accept", "application/json")
        .send()
        .expect("Cannot invoke API")
        .text().unwrap();

    let response = serde_json::from_str::<TaggunResponseDto>(&*response_text)
        .expect("Cannot deserialize response");

    let ext_id = response.entities.receipt_number.data.unwrap_or("".parse().unwrap());
    let date = NaiveDateTime::parse_and_remainder(&*response.date.data, "%Y-%m-%dT%H:%M:%S")
        .expect(&format!("Invalid receipt date{}", response.date.data)).0;
    let amount = (response.total_amount.data * 100.0).to_i32().unwrap();
    let merchant_name = response.merchant_name.data.unwrap_or("".parse().unwrap());
    let merchant_address = response.merchant_address.data.unwrap_or("".parse().unwrap());

    let found_receipts: i64 = schema::receipts::dsl::receipts
        .filter(schema::receipts::ext_id.eq(&ext_id))
        .filter(schema::receipts::amount_cents.eq(&amount))
        .filter(schema::receipts::date.eq(&date))
        .count()
        .get_result(connection)
        .expect("Error looking for existing receipts");

    if found_receipts > 0 {
        error!(
            "Receipt with amount:{} {}, date:'{}' and id:'{}' already imported.",
            &amount.to_f32().unwrap() / 100.0f32,
            &response.total_amount.currency_code,
            &date,
            &ext_id
        );
        return ExitCode::FAILURE;
    }

    let new_receipt: Receipt = diesel::insert_into(schema::receipts::dsl::receipts)
        .values(NewReceipt {
            ext_id: &ext_id,
            date: &date,
            amount_cents: &amount,
            currency: &response.total_amount.currency_code,
            merchant_name: &merchant_name,
            merchant_address: &merchant_address,
            original_data: &*response_text,
            scan_file_name: &scan_file_name,
        })
        .get_result(connection)
        .unwrap();

    for line_item in response.entities.product_line_items {
        let mut amount_cents = 0i32;
        if !line_item.data.total_price.is_none() {
            amount_cents = (line_item.data.total_price.unwrap().data * 100.0) as i32
        }

        let mut unit_price_cents = 0i32;
        if !line_item.data.unit_price.is_none() {
            unit_price_cents = (line_item.data.unit_price.unwrap().data * 100.0) as i32
        }

        let mut quantity = 0f32;
        if !line_item.data.quantity.is_none() {
            quantity = line_item.data.quantity.unwrap().data
        }

        diesel::insert_into(schema::receipts_line_items::dsl::receipts_line_items)
            .values(NewReceiptLineItem {
                receipt_id: &new_receipt.id,
                quantity: &BigDecimal::from_f32(quantity).unwrap(),
                unit_price_cents: &unit_price_cents,
                amount_cents: &amount_cents,
                description: &line_item.data.name.data,
                raw_text: &line_item.text,
            })
            .execute(connection)
            .unwrap();
    }

    info!(
        "Imported ext_id:'{:?}', merchant:'{:?}', total:{:?} {:?}",
        &ext_id,
        &merchant_name,
        &amount.to_f32().unwrap() / 100.0f32,
        &response.total_amount.currency_code
    );

    info!("Done");

    return ExitCode::SUCCESS;
}

pub fn establish_db_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_e| error!("DATABASE_URL missing"))
        .expect("config");

    PgConnection::establish(&database_url)
        .map_err(|e| error!("Cannot connect to DB: {}", e.to_string()))
        .unwrap()
}

#[derive(Deserialize, Clone, Debug)]
struct TaggunEntityNumericDataEntryDto {
    data: f32,
}

#[derive(Deserialize, Clone, Debug)]
struct TaggunEntityStringDataEntryDto {
    data: String,
}

#[derive(Deserialize, Clone, Debug)]
struct TaggunProductLineItemDataDto {
    quantity: Option<TaggunEntityNumericDataEntryDto>,
    #[serde(rename(deserialize = "unitPrice"))]
    unit_price: Option<TaggunEntityNumericDataEntryDto>,
    #[serde(rename(deserialize = "totalPrice"))]
    total_price: Option<TaggunEntityNumericDataEntryDto>,
    name: TaggunEntityStringDataEntryDto,
}

#[derive(Deserialize, Clone, Debug)]
struct TaggunProductLineItemDto {
    pub data: TaggunProductLineItemDataDto,
    pub text: String,
}

#[derive(Deserialize, Clone, Debug)]
struct TaggunReceiptNumberDto {
    pub data: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
struct TaggunEntitiesDto {
    #[serde(rename(deserialize = "productLineItems"))]
    pub product_line_items: Vec<TaggunProductLineItemDto>,
    #[serde(rename(deserialize = "receiptNumber"))]
    pub receipt_number: TaggunReceiptNumberDto,
}

#[derive(Deserialize, Clone, Debug)]
struct TaggunDateDto {
    pub data: String,
}

#[derive(Deserialize, Clone, Debug)]
struct TaggunTotalAmountDto {
    pub data: f32,
    #[serde(rename(deserialize = "currencyCode"))]
    pub currency_code: String,
}

#[derive(Deserialize, Clone, Debug)]
struct TaggunMerchantNameDto {
    pub data: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
struct TaggunMerchantAddressDto {
    pub data: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
struct TaggunResponseDto {
    pub date: TaggunDateDto,
    #[serde(rename(deserialize = "totalAmount"))]
    pub total_amount: TaggunTotalAmountDto,
    pub entities: TaggunEntitiesDto,
    #[serde(rename(deserialize = "merchantName"))]
    pub merchant_name: TaggunMerchantNameDto,
    #[serde(rename(deserialize = "merchantAddress"))]
    pub merchant_address: TaggunMerchantAddressDto,
}