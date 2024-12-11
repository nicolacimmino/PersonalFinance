use diesel::{QueryableByName, Selectable};

#[derive(QueryableByName, Selectable, Debug)]
#[diesel(table_name = crate::manual_schema::application_alerts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Alert {
    pub category: String,
    pub message: String,
    pub item: String,
    pub item_id: String,
    pub level: String,
}