use diesel::{RunQueryDsl, QueryDsl, SelectableHelper, ExpressionMethods};
use crate::{establish_db_connection, manual_schema};
use crate::alerts::model::Alert;
use crate::manual_schema::alerts::dsl::alerts;

pub struct AlertsService {}

impl AlertsService {
    pub fn get_alerts(&mut self) -> Vec<Alert> {
        return alerts
            .order(manual_schema::alerts::item.asc())
            .select(Alert::as_select())
            .load::<Alert>(&mut establish_db_connection())
            .expect("Error loading alerts");
    }
}