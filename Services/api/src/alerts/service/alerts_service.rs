use diesel::{RunQueryDsl, sql_query};

use crate::alerts::model::Alert;
use crate::establish_db_connection;

pub struct AlertsService {}

impl AlertsService {
    pub fn get_alerts(&mut self) -> Vec<Alert> {
        return sql_query("
            SELECT * FROM application.alerts
                ORDER BY item ASC
          ").load::<Alert>(&mut establish_db_connection())
            .expect("Error loading alerts");
    }
}