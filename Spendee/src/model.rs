use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::sp_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpTransaction {
    pub id: Uuid,
    pub date: String,
    pub wallet: String,
    pub type_: String,
    pub category: String,
    pub amount: String,
    pub currency: String,
    pub note: String,
    pub labels: String,
    pub author: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::sp_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSpTransaction<'a> {
    pub date: &'a str,
    pub wallet: &'a str,
    pub type_: &'a str,
    pub category: &'a str,
    pub amount: &'a str,
    pub currency: &'a str,
    pub note: &'a str,
    pub labels: &'a str,
    pub author: &'a str,
}

