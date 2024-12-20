use diesel::{RunQueryDsl, sql_query};
use diesel::sql_types::Integer;

use crate::establish_db_connection;
use crate::receipts::model::ApplicationReceipt;

pub struct ReceiptsService {}

impl ReceiptsService {
    pub fn get_receipts(&mut self) -> Vec<ApplicationReceipt> {
        return sql_query("
            SELECT * FROM application.receipts
          ").load::<ApplicationReceipt>(&mut establish_db_connection())
            .expect("Error loading Receipts");
    }

    pub fn get_receipt(&mut self, receipt_id: i32) -> ApplicationReceipt {
        return sql_query("
            SELECT * FROM application.receipts WHERE id = $1
          ").bind::<Integer, _>(receipt_id)
            .load::<ApplicationReceipt>(&mut establish_db_connection())
            .expect("Error loading Receipts")
            .into_iter().nth(0)
            .unwrap();
    }
}