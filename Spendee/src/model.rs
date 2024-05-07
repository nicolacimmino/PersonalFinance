use alloc::string::String;
use diesel::{Associations, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::sp_accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpAccount {
    pub id: Uuid,
    pub wallet: String,
    pub account_id: Option<i32>,
}

#[derive(Queryable, Selectable, Associations, Debug)]
#[diesel(belongs_to(SpAccount, foreign_key = sp_account_id))]
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
    pub sp_account_id: Uuid
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
    pub sp_account_id: &'a Uuid
}

