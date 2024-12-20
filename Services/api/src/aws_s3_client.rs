use aws_config::BehaviorVersion;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client};

pub struct AwsS3Client {}

impl AwsS3Client {
    pub async fn get_file(&mut self, scan_file_name: String) -> Vec<u8> {
        let region_provider = RegionProviderChain::default_provider().or_else("eu-central-1");
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;
        let client = Client::new(&config);

        let bucket: String = "pfinance-receipts".to_string();
        let object: String = format!("receipts-documents-processed/{}", scan_file_name).to_string();

        return  client
            .get_object()
            .bucket(bucket)
            .key(object)
            .send()
            .await
            .unwrap()
            .body.collect().await.unwrap().to_vec();
    }
}