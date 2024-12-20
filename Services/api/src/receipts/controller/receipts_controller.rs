use rocket::figment::value::Num;
use rocket::{get};
use rocket::http::{ContentType, Status};
use rocket::response::{content, status};
use crate::aws_s3_client::AwsS3Client;

use crate::guard::ApiKey;
use crate::receipts::dto::ReceiptDto;
use crate::receipts::model::ApplicationReceipt;
use crate::receipts::service::ReceiptsService;

#[get("/receipts")]
pub fn get_receipts(_key: ApiKey<'_>) -> status::Custom<content::RawJson<String>> {
    let mut receipts_service = ReceiptsService {};

    let mut dtos: Vec<ReceiptDto> = Vec::new();

    for receipt in receipts_service.get_receipts() {
        dtos.push(create_receipt_dto(receipt))
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}

#[get("/receipts/<id>")]
pub fn get_receipt(_key: ApiKey<'_>, id: i32) -> status::Custom<content::RawJson<String>> {
    let mut receipts_service = ReceiptsService {};

    let receipt = receipts_service.get_receipt(id);

    let receipt_dto = create_receipt_dto(receipt);

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&receipt_dto).expect("Serialization Failed")),
    )
}

#[get("/receipts/<id>/pdf")]
pub async fn get_receipt_pdf(_key: ApiKey<'_>, id: i32) -> (ContentType, Vec<u8>) {
    let mut receipts_service = ReceiptsService {};

    let receipt = receipts_service.get_receipt(id);

    let pdf = AwsS3Client {}.get_file(receipt.scan_file_name).await;

    (ContentType::PDF, pdf)
}

fn create_receipt_dto(receipt: ApplicationReceipt) -> ReceiptDto {
    return ReceiptDto {
        id: Num::from(receipt.id),
        date: receipt.date.to_string(),
        amount_cents: Num::from(receipt.amount_cents),
        currency: receipt.currency,
        ext_id: receipt.ext_id,
        merchant_name: receipt.merchant_name,
        merchant_address: receipt.merchant_address,
        scan_file_name: receipt.scan_file_name,
        transaction_id: receipt.transaction_id,
        transaction_category: receipt.transaction_category,
        transaction_amount_cents: receipt.transaction_amount_cents,
        transaction_currency: receipt.transaction_currency,
        account_code: receipt.account_code,
        account_description: receipt.account_description,
    };
}