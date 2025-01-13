use diesel::{QueryableByName, Selectable};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(QueryableByName, Selectable, Debug)]
#[diesel(table_name = crate::manual_schema::application_categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ApplicationCategory {
    pub id: Uuid,
    pub code: String,
    pub color: String,
    pub discontinued: String
}