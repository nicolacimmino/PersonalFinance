use std::fs;
use std::io::BufReader;

#[derive(Debug, serde::Deserialize)]
pub struct SpendeeRecord {
    #[serde(rename(deserialize = "Date"))]
    pub date: String,
    #[serde(rename(deserialize = "Wallet"))]
    pub wallet: String,
    #[serde(rename(deserialize = "Type"))]
    pub type_: String,
    #[serde(rename(deserialize = "Category name"))]
    pub category_name: String,
    #[serde(rename(deserialize = "Amount"))]
    pub amount: String,
    #[serde(rename(deserialize = "Currency"))]
    pub currency: String,
    #[serde(rename(deserialize = "Note"))]
    pub note: String,
    #[serde(rename(deserialize = "Labels"))]
    pub labels: String,
    #[serde(rename(deserialize = "Author"))]
    pub author: String,
}

pub struct Spendee {

}

impl Spendee {
    pub fn import(file_name: &String) -> Vec<SpendeeRecord> {
        let mut records: Vec::<SpendeeRecord> = vec![];

        let csv_data =
            fs::read_to_string(file_name)
                .expect("Cannot open file");

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(BufReader::new(csv_data.as_bytes()));

        for result in reader.deserialize() {
            let record: SpendeeRecord = result.expect("Error reading record");
          //  println!("{:?}", record);
            records.push(record);
        }

        return records;
    }
}
