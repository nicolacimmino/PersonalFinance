use diesel::{Identifiable, Queryable, Selectable};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Queryable, Identifiable, PartialEq, Selectable, Debug)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub id: Uuid,
    pub code: String,
    pub color: String,
}