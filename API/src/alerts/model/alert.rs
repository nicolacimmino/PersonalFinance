use diesel::{Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::manual_schema::alerts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Alert {
    pub category: String,
    pub message: String,
    pub item: String,
    pub item_id: String,
    pub level: String
}